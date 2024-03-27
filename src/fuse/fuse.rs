use fuser::consts::FOPEN_DIRECT_IO;
use fuser::{
    FileType, Filesystem, KernelConfig, MountOption, ReplyAttr, ReplyCreate, ReplyData, ReplyEmpty,
    ReplyEntry, ReplyOpen, ReplyWrite, Request, FUSE_ROOT_ID,
};
use libc;
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use std::cmp::min;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, ErrorKind, Read, Seek, SeekFrom, Write};
use std::os::raw::c_int;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

struct MyFS {
    data_dir: String,
    next_file_handle: AtomicU64,
    direct_io: bool,
    suid_support: bool,
}

const FILE_HANDLE_READ_BIT: u64 = 1 << 63;
const FILE_HANDLE_WRITE_BIT: u64 = 1 << 62;
const BLOCK_SIZE: u64 = 512;
const MAX_NAME_LENGTH: u32 = 255;
const MAX_FILE_SIZE: u64 = 1024 * 1024 * 1024 * 1024; // 1 TB
type Inode = u64;

const FMODE_EXEC: i32 = 0x20;

type DirectoryDescriptor = BTreeMap<Vec<u8>, (Inode, FileKind)>;

#[derive(Serialize, Deserialize)]
struct InodeAttributes {
    pub inode: Inode,
    pub open_file_handles: u64, // Ref count of open file handles to this inode
    pub size: u64,
    pub last_accessed: (i64, u32),
    pub last_modified: (i64, u32),
    pub last_metadata_changed: (i64, u32),
    pub kind: FileKind,
    // Permissions and special mode bits
    pub mode: u16,
    pub hardlinks: u32,
    pub uid: u32,
    pub gid: u32,
    pub xattrs: BTreeMap<Vec<u8>, Vec<u8>>,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
enum FileKind {
    File,
    Directory,
    Symlink,
}

impl From<FileKind> for fuser::FileType {
    fn from(kind: FileKind) -> Self {
        match kind {
            FileKind::File => fuser::FileType::RegularFile,
            FileKind::Directory => fuser::FileType::Directory,
            FileKind::Symlink => fuser::FileType::Symlink,
        }
    }
}

impl From<InodeAttributes> for fuser::FileAttr {
    fn from(attrs: InodeAttributes) -> Self {
        fuser::FileAttr {
            ino: attrs.inode,
            size: attrs.size,
            blocks: (attrs.size + BLOCK_SIZE - 1) / BLOCK_SIZE,
            atime: system_time_from_time(attrs.last_accessed.0, attrs.last_accessed.1),
            mtime: system_time_from_time(attrs.last_modified.0, attrs.last_modified.1),
            ctime: system_time_from_time(
                attrs.last_metadata_changed.0,
                attrs.last_metadata_changed.1,
            ),
            crtime: SystemTime::UNIX_EPOCH,
            kind: attrs.kind.into(),
            perm: attrs.mode,
            nlink: attrs.hardlinks,
            uid: attrs.uid,
            gid: attrs.gid,
            rdev: 0,
            blksize: BLOCK_SIZE as u32,
            flags: 0,
        }
    }
}

fn time_now() -> (i64, u32) {
    time_from_system_time(&SystemTime::now())
}

fn system_time_from_time(secs: i64, nsecs: u32) -> SystemTime {
    if secs >= 0 {
        UNIX_EPOCH + Duration::new(secs as u64, nsecs)
    } else {
        UNIX_EPOCH - Duration::new((-secs) as u64, nsecs)
    }
}

fn time_from_system_time(system_time: &SystemTime) -> (i64, u32) {
    // Convert to signed 64-bit time with epoch at 0
    match system_time.duration_since(UNIX_EPOCH) {
        Ok(duration) => (duration.as_secs() as i64, duration.subsec_nanos()),
        Err(before_epoch_error) => (
            -(before_epoch_error.duration().as_secs() as i64),
            before_epoch_error.duration().subsec_nanos(),
        ),
    }
}

// fn clear_suid_sgid(attr: &mut InodeAttributes) {
//     attr.mode &= !libc::S_ISUID as u16;
//     // SGID is only suppose to be cleared if XGRP is set
//     if attr.mode & libc::S_IXGRP as u16 != 0 {
//         attr.mode &= !libc::S_ISGID as u16;
//     }
// }

// pub fn check_access(
//     //Could be cut
//     file_uid: u32,
//     file_gid: u32,
//     file_mode: u16,
//     uid: u32,
//     gid: u32,
//     mut access_mask: i32,
// ) -> bool {
//     // F_OK tests for existence of file
//     if access_mask == libc::F_OK {
//         return true;
//     }
//     let file_mode = i32::from(file_mode);

//     // root is allowed to read & write anything
//     if uid == 0 {
//         // root only allowed to exec if one of the X bits is set
//         access_mask &= libc::X_OK;
//         access_mask -= access_mask & (file_mode >> 6);
//         access_mask -= access_mask & (file_mode >> 3);
//         access_mask -= access_mask & file_mode;
//         return access_mask == 0;
//     }

//     if uid == file_uid {
//         access_mask -= access_mask & (file_mode >> 6);
//     } else if gid == file_gid {
//         access_mask -= access_mask & (file_mode >> 3);
//     } else {
//         access_mask -= access_mask & file_mode;
//     }

//     return access_mask == 0;
// }

fn as_file_kind(mut mode: u32) -> FileKind {
    mode &= libc::S_IFMT as u32;

    if mode == libc::S_IFREG as u32 {
        return FileKind::File;
    } else if mode == libc::S_IFLNK as u32 {
        return FileKind::Symlink;
    } else if mode == libc::S_IFDIR as u32 {
        return FileKind::Directory;
    } else {
        unimplemented!("{}", mode);
    }
}

impl Filesystem for MyFS {
    fn init(
        &mut self,
        _req: &Request,
        #[allow(unused_variables)] config: &mut KernelConfig,
    ) -> Result<(), c_int> {
        // #[cfg(feature = "abi-7-26")]
        // config.add_capabilities(FUSE_HANDLE_KILLPRIV).unwrap();

        fs::create_dir_all(Path::new(&self.data_dir).join("inodes")).unwrap();
        fs::create_dir_all(Path::new(&self.data_dir).join("contents")).unwrap();
        if self.get_inode(FUSE_ROOT_ID).is_err() {
            // Initialize with empty filesystem
            let root = InodeAttributes {
                inode: FUSE_ROOT_ID,
                open_file_handles: 0,
                size: 0,
                last_accessed: time_now(),
                last_modified: time_now(),
                last_metadata_changed: time_now(),
                kind: FileKind::Directory,
                mode: 0o777,
                hardlinks: 2,
                uid: 0,
                gid: 0,
                xattrs: Default::default(),
            };
            self.write_inode(&root);
            let mut entries = BTreeMap::new();
            entries.insert(b".".to_vec(), (FUSE_ROOT_ID, FileKind::Directory));
            self.write_directory_content(FUSE_ROOT_ID, entries);
        }
        Ok(())
    }
    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        let attr = fuser::FileAttr {
            ino: 1,
            size: 4096000000,
            blksize: 4096000,
            blocks: 1,
            atime: SystemTime::now().into(),
            mtime: SystemTime::now().into(),
            ctime: SystemTime::now().into(),
            crtime: SystemTime::now().into(),
            kind: FileType::Directory,
            perm: 0o755,
            nlink: 2,
            uid: 1000,
            gid: 1000,
            rdev: 0,
            flags: 0,
        };
        reply.attr(&Duration::new(0, 0), &attr);
    }

    fn lookup(&mut self, req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        if name.len() > MAX_NAME_LENGTH as usize {
            reply.error(libc::ENAMETOOLONG);
            return;
        }
        let parent_attrs = self.get_inode(parent).unwrap();
        // if !check_access(
        //     parent_attrs.uid,
        //     parent_attrs.gid,
        //     parent_attrs.mode,
        //     req.uid(),
        //     req.gid(),
        //     libc::X_OK,
        // ) {
        //     reply.error(libc::EACCES);
        //     return;
        // }

        match self.lookup_name(parent, name) {
            Ok(attrs) => reply.entry(&Duration::new(0, 0), &attrs.into(), 0),
            Err(error_code) => reply.error(error_code),
        }
    }

    fn forget(&mut self, _req: &Request, _ino: u64, _nlookup: u64) {}

    // fn getattr(&mut self, _req: &Request, inode: u64, reply: ReplyAttr) {
    //     match self.get_inode(inode) {
    //         Ok(attrs) => reply.attr(&Duration::new(0, 0), &attrs.into()),
    //         Err(error_code) => reply.error(error_code),
    //     }
    // }

    // fn setattr(
    //     &mut self,
    //     req: &Request,
    //     inode: u64,
    //     mode: Option<u32>,
    //     uid: Option<u32>,
    //     gid: Option<u32>,
    //     size: Option<u64>,
    //     atime: Option<TimeOrNow>,
    //     mtime: Option<TimeOrNow>,
    //     _ctime: Option<SystemTime>,
    //     fh: Option<u64>,
    //     _crtime: Option<SystemTime>,
    //     _chgtime: Option<SystemTime>,
    //     _bkuptime: Option<SystemTime>,
    //     _flags: Option<u32>,
    //     reply: ReplyAttr,
    // ) {
    //     let mut attrs = match self.get_inode(inode) {
    //         Ok(attrs) => attrs,
    //         Err(error_code) => {
    //             reply.error(error_code);
    //             return;
    //         }
    //     };

    //     if let Some(mode) = mode {
    //         debug!("chmod() called with {:?}, {:o}", inode, mode);
    //         if req.uid() != 0 && req.uid() != attrs.uid {
    //             reply.error(libc::EPERM);
    //             return;
    //         }
    //         if req.uid() != 0
    //             && req.gid() != attrs.gid
    //             && !get_groups(req.pid()).contains(&attrs.gid)
    //         {
    //             // If SGID is set and the file belongs to a group that the caller is not part of
    //             // then the SGID bit is suppose to be cleared during chmod
    //             attrs.mode = (mode & !libc::S_ISGID as u32) as u16;
    //         } else {
    //             attrs.mode = mode as u16;
    //         }
    //         attrs.last_metadata_changed = time_now();
    //         self.write_inode(&attrs);
    //         reply.attr(&Duration::new(0, 0), &attrs.into());
    //         return;
    //     }

    //     if uid.is_some() || gid.is_some() {
    //         debug!("chown() called with {:?} {:?} {:?}", inode, uid, gid);
    //         if let Some(gid) = gid {
    //             // Non-root users can only change gid to a group they're in
    //             if req.uid() != 0 && !get_groups(req.pid()).contains(&gid) {
    //                 reply.error(libc::EPERM);
    //                 return;
    //             }
    //         }
    //         if let Some(uid) = uid {
    //             if req.uid() != 0
    //                 // but no-op changes by the owner are not an error
    //                 && !(uid == attrs.uid && req.uid() == attrs.uid)
    //             {
    //                 reply.error(libc::EPERM);
    //                 return;
    //             }
    //         }
    //         // Only owner may change the group
    //         if gid.is_some() && req.uid() != 0 && req.uid() != attrs.uid {
    //             reply.error(libc::EPERM);
    //             return;
    //         }

    //         if attrs.mode & (libc::S_IXUSR | libc::S_IXGRP | libc::S_IXOTH) as u16 != 0 {
    //             // SUID & SGID are suppose to be cleared when chown'ing an executable file
    //             clear_suid_sgid(&mut attrs);
    //         }

    //         if let Some(uid) = uid {
    //             attrs.uid = uid;
    //             // Clear SETUID on owner change
    //             attrs.mode &= !libc::S_ISUID as u16;
    //         }
    //         if let Some(gid) = gid {
    //             attrs.gid = gid;
    //             // Clear SETGID unless user is root
    //             if req.uid() != 0 {
    //                 attrs.mode &= !libc::S_ISGID as u16;
    //             }
    //         }
    //         attrs.last_metadata_changed = time_now();
    //         self.write_inode(&attrs);
    //         reply.attr(&Duration::new(0, 0), &attrs.into());
    //         return;
    //     }

    //     if let Some(size) = size {
    //         debug!("truncate() called with {:?} {:?}", inode, size);
    //         if let Some(handle) = fh {
    //             // If the file handle is available, check access locally.
    //             // This is important as it preserves the semantic that a file handle opened
    //             // with W_OK will never fail to truncate, even if the file has been subsequently
    //             // chmod'ed
    //             if self.check_file_handle_write(handle) {
    //                 if let Err(error_code) = self.truncate(inode, size, 0, 0) {
    //                     reply.error(error_code);
    //                     return;
    //                 }
    //             } else {
    //                 reply.error(libc::EACCES);
    //                 return;
    //             }
    //         } else if let Err(error_code) = self.truncate(inode, size, req.uid(), req.gid()) {
    //             reply.error(error_code);
    //             return;
    //         }
    //     }

    //     let now = time_now();
    //     if let Some(atime) = atime {
    //         debug!("utimens() called with {:?}, atime={:?}", inode, atime);

    //         if attrs.uid != req.uid() && req.uid() != 0 && atime != Now {
    //             reply.error(libc::EPERM);
    //             return;
    //         }

    //         if attrs.uid != req.uid()
    //             && !check_access(
    //                 attrs.uid,
    //                 attrs.gid,
    //                 attrs.mode,
    //                 req.uid(),
    //                 req.gid(),
    //                 libc::W_OK,
    //             )
    //         {
    //             reply.error(libc::EACCES);
    //             return;
    //         }

    //         attrs.last_accessed = match atime {
    //             TimeOrNow::SpecificTime(time) => time_from_system_time(&time),
    //             Now => now,
    //         };
    //         attrs.last_metadata_changed = now;
    //         self.write_inode(&attrs);
    //     }
    //     if let Some(mtime) = mtime {
    //         debug!("utimens() called with {:?}, mtime={:?}", inode, mtime);

    //         if attrs.uid != req.uid() && req.uid() != 0 && mtime != Now {
    //             reply.error(libc::EPERM);
    //             return;
    //         }

    //         if attrs.uid != req.uid()
    //             && !check_access(
    //                 attrs.uid,
    //                 attrs.gid,
    //                 attrs.mode,
    //                 req.uid(),
    //                 req.gid(),
    //                 libc::W_OK,
    //             )
    //         {
    //             reply.error(libc::EACCES);
    //             return;
    //         }

    //         attrs.last_modified = match mtime {
    //             TimeOrNow::SpecificTime(time) => time_from_system_time(&time),
    //             Now => now,
    //         };
    //         attrs.last_metadata_changed = now;
    //         self.write_inode(&attrs);
    //     }

    //     let attrs = self.get_inode(inode).unwrap();
    //     reply.attr(&Duration::new(0, 0), &attrs.into());
    //     return;
    // }

    fn open(&mut self, req: &Request, inode: u64, flags: i32, reply: ReplyOpen) {
        debug!("open() called for {:?}", inode);
        let (access_mask, read, write) = match flags & libc::O_ACCMODE {
            libc::O_RDONLY => {
                // Behavior is undefined, but most filesystems return EACCES
                if flags & libc::O_TRUNC != 0 {
                    reply.error(libc::EACCES);
                    return;
                }
                if flags & FMODE_EXEC != 0 {
                    // Open is from internal exec syscall
                    (libc::X_OK, true, false)
                } else {
                    (libc::R_OK, true, false)
                }
            }
            libc::O_WRONLY => (libc::W_OK, false, true),
            libc::O_RDWR => (libc::R_OK | libc::W_OK, true, true),
            // Exactly one access mode flag must be specified
            _ => {
                reply.error(libc::EINVAL);
                return;
            }
        };

        match self.get_inode(inode) {
            Ok(mut attr) => {
                attr.open_file_handles += 1;
                self.write_inode(&attr);
                let open_flags = if self.direct_io { FOPEN_DIRECT_IO } else { 0 };
                reply.opened(self.allocate_next_file_handle(read, write), open_flags);

                return;
            }
            Err(error_code) => reply.error(error_code),
        }
    }

    fn read(
        &mut self,
        _req: &Request,
        inode: Inode,
        fh: u64,
        offset: i64,
        size: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
        reply: ReplyData,
    ) {
        debug!(
            "read() called on {:?} offset={:?} size={:?}",
            inode, offset, size
        );
        assert!(offset >= 0);
        if !self.check_file_handle_read(fh) {
            reply.error(libc::EACCES);
            return;
        }

        let path = self.content_path(inode);
        if let Ok(file) = File::open(path) {
            let file_size = file.metadata().unwrap().len();
            // Could underflow if file length is less than local_start
            let read_size = min(size, file_size.saturating_sub(offset as u64) as u32);

            let mut buffer = vec![0; read_size as usize];
            file.read_exact_at(&mut buffer, offset as u64).unwrap();
            reply.data(&buffer);
        } else {
            reply.error(libc::ENOENT);
        }
    }

    fn write(
        &mut self,
        _req: &Request,
        inode: Inode,
        fh: u64,
        offset: i64,
        data: &[u8],
        _write_flags: u32,
        #[allow(unused_variables)] flags: i32,
        _lock_owner: Option<u64>,
        reply: ReplyWrite,
    ) {
        debug!("write() called with {:?} size={:?}", inode, data.len());
        assert!(offset >= 0);
        if !self.check_file_handle_write(fh) {
            reply.error(libc::EACCES);
            return;
        }

        let path = self.content_path(inode);
        if let Ok(mut file) = OpenOptions::new().write(true).open(path) {
            file.seek(SeekFrom::Start(offset as u64)).unwrap();
            file.write_all(data).unwrap();

            let mut attrs = self.get_inode(inode).unwrap();
            attrs.last_metadata_changed = time_now();
            attrs.last_modified = time_now();
            if data.len() + offset as usize > attrs.size as usize {
                attrs.size = (data.len() + offset as usize) as u64;
            }

            // clear_suid_sgid(&mut attrs);
            self.write_inode(&attrs);

            reply.written(data.len() as u32);
        } else {
            reply.error(libc::EBADF);
        }
    }

    fn release(
        &mut self,
        _req: &Request<'_>,
        inode: u64,
        _fh: u64,
        _flags: i32,
        _lock_owner: Option<u64>,
        _flush: bool,
        reply: ReplyEmpty,
    ) {
        if let Ok(mut attrs) = self.get_inode(inode) {
            attrs.open_file_handles -= 1;
        }
        reply.ok();
    }

    fn access(&mut self, req: &Request, inode: u64, mask: i32, reply: ReplyEmpty) {
        debug!("access() called with {:?} {:?}", inode, mask);
        match self.get_inode(inode) {
            Ok(attr) => {
                reply.ok();
            }
            Err(error_code) => reply.error(error_code),
        }
    }

    fn create(
        &mut self,
        req: &Request,
        parent: u64,
        name: &OsStr,
        mut mode: u32,
        _umask: u32,
        flags: i32,
        reply: ReplyCreate,
    ) {
        debug!("create() called with {:?} {:?}", parent, name);
        if self.lookup_name(parent, name).is_ok() {
            reply.error(libc::EEXIST);
            return;
        }

        let (read, write) = match flags & libc::O_ACCMODE {
            libc::O_RDONLY => (true, false),
            libc::O_WRONLY => (false, true),
            libc::O_RDWR => (true, true),
            // Exactly one access mode flag must be specified
            _ => {
                reply.error(libc::EINVAL);
                return;
            }
        };

        let mut parent_attrs = match self.get_inode(parent) {
            Ok(attrs) => attrs,
            Err(error_code) => {
                reply.error(error_code);
                return;
            }
        };

        // if !check_access(
        //     parent_attrs.uid,
        //     parent_attrs.gid,
        //     parent_attrs.mode,
        //     req.uid(),
        //     req.gid(),
        //     libc::W_OK,
        // ) {
        //     reply.error(libc::EACCES);
        //     return;
        // }
        parent_attrs.last_modified = time_now();
        parent_attrs.last_metadata_changed = time_now();
        self.write_inode(&parent_attrs);

        if req.uid() != 0 {
            mode &= !(libc::S_ISUID | libc::S_ISGID) as u32;
        }

        let inode = self.allocate_next_inode();
        let attrs = InodeAttributes {
            inode,
            open_file_handles: 1,
            size: 0,
            last_accessed: time_now(),
            last_modified: time_now(),
            last_metadata_changed: time_now(),
            kind: as_file_kind(mode),
            mode: 0o777,//self.creation_mode(mode),
            hardlinks: 1,
            uid: 0,//req.uid(),
            gid: 0,//creation_gid(&parent_attrs, req.gid()),
            xattrs: Default::default(),
        };
        self.write_inode(&attrs);
        File::create(self.content_path(inode)).unwrap();

        // if as_file_kind(mode) == FileKind::Directory {
        //     let mut entries = BTreeMap::new();
        //     entries.insert(b".".to_vec(), (inode, FileKind::Directory));
        //     entries.insert(b"..".to_vec(), (parent, FileKind::Directory));
        //     self.write_directory_content(inode, entries);
        // }

        let mut entries = self.get_directory_content(parent).unwrap();
        entries.insert(name.as_bytes().to_vec(), (inode, attrs.kind));
        self.write_directory_content(parent, entries);

        // TODO: implement flags
        reply.created(
            &Duration::new(0, 0),
            &attrs.into(),
            0,
            self.allocate_next_file_handle(read, write),
            0,
        );
    }

    // #[cfg(target_os = "linux")]
    // fn fallocate(
    //     &mut self,
    //     _req: &Request<'_>,
    //     inode: u64,
    //     _fh: u64,
    //     offset: i64,
    //     length: i64,
    //     mode: i32,
    //     reply: ReplyEmpty,
    // ) {
    //     let path = self.content_path(inode);
    //     if let Ok(file) = OpenOptions::new().write(true).open(path) {
    //         unsafe {
    //             libc::fallocate64(file.into_raw_fd(), mode, offset, length);
    //         }
    //         if mode & libc::FALLOC_FL_KEEP_SIZE == 0 {
    //             let mut attrs = self.get_inode(inode).unwrap();
    //             attrs.last_metadata_changed = time_now();
    //             attrs.last_modified = time_now();
    //             if (offset + length) as u64 > attrs.size {
    //                 attrs.size = (offset + length) as u64;
    //             }
    //             self.write_inode(&attrs);
    //         }
    //         reply.ok();
    //     } else {
    //         reply.error(libc::ENOENT);
    //     }
    // }

    fn copy_file_range(
        &mut self,
        _req: &Request<'_>,
        src_inode: u64,
        src_fh: u64,
        src_offset: i64,
        dest_inode: u64,
        dest_fh: u64,
        dest_offset: i64,
        size: u64,
        _flags: u32,
        reply: ReplyWrite,
    ) {
        debug!(
            "copy_file_range() called with src ({}, {}, {}) dest ({}, {}, {}) size={}",
            src_fh, src_inode, src_offset, dest_fh, dest_inode, dest_offset, size
        );
        if !self.check_file_handle_read(src_fh) {
            reply.error(libc::EACCES);
            return;
        }
        if !self.check_file_handle_write(dest_fh) {
            reply.error(libc::EACCES);
            return;
        }

        let src_path = self.content_path(src_inode);
        if let Ok(file) = File::open(src_path) {
            let file_size = file.metadata().unwrap().len();
            // Could underflow if file length is less than local_start
            let read_size = min(size, file_size.saturating_sub(src_offset as u64));

            let mut data = vec![0; read_size as usize];
            file.read_exact_at(&mut data, src_offset as u64).unwrap();

            let dest_path = self.content_path(dest_inode);
            if let Ok(mut file) = OpenOptions::new().write(true).open(dest_path) {
                file.seek(SeekFrom::Start(dest_offset as u64)).unwrap();
                file.write_all(&data).unwrap();

                let mut attrs = self.get_inode(dest_inode).unwrap();
                attrs.last_metadata_changed = time_now();
                attrs.last_modified = time_now();
                if data.len() + dest_offset as usize > attrs.size as usize {
                    attrs.size = (data.len() + dest_offset as usize) as u64;
                }
                self.write_inode(&attrs);

                reply.written(data.len() as u32);
            } else {
                reply.error(libc::EBADF);
            }
        } else {
            reply.error(libc::ENOENT);
        }
    }
    // Implement other filesystem methods as needed
}

impl MyFS {
    fn new(
        data_dir: String,
        direct_io: bool,
        #[allow(unused_variables)] suid_support: bool,
    ) -> MyFS {
        MyFS {
            data_dir,
            next_file_handle: AtomicU64::new(1),
            direct_io,
            suid_support: false,
            //Put a indexing data structure here
        }
    }

    fn allocate_next_inode(&self) -> Inode {
        let path = Path::new(&self.data_dir).join("superblock");
        let current_inode = if let Ok(file) = File::open(&path) {
            bincode::deserialize_from(file).unwrap()
        } else {
            fuser::FUSE_ROOT_ID
        };

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .unwrap();
        bincode::serialize_into(file, &(current_inode + 1)).unwrap();

        current_inode + 1
    }

    fn allocate_next_file_handle(&self, read: bool, write: bool) -> u64 {
        let mut fh = self.next_file_handle.fetch_add(1, Ordering::SeqCst);
        // Assert that we haven't run out of file handles
        assert!(fh < FILE_HANDLE_READ_BIT.min(FILE_HANDLE_WRITE_BIT));
        if read {
            fh |= FILE_HANDLE_READ_BIT;
        }
        if write {
            fh |= FILE_HANDLE_WRITE_BIT;
        }

        fh
    }

    fn check_file_handle_read(&self, file_handle: u64) -> bool {
        (file_handle & FILE_HANDLE_READ_BIT) != 0
    }

    fn check_file_handle_write(&self, file_handle: u64) -> bool {
        (file_handle & FILE_HANDLE_WRITE_BIT) != 0
    }

    fn content_path(&self, inode: Inode) -> PathBuf {
        Path::new(&self.data_dir)
            .join("contents")
            .join(inode.to_string())
    }

    fn get_directory_content(&self, inode: Inode) -> Result<DirectoryDescriptor, c_int> {
        let path = Path::new(&self.data_dir)
            .join("contents")
            .join(inode.to_string());
        if let Ok(file) = File::open(path) {
            Ok(bincode::deserialize_from(file).unwrap())
        } else {
            Err(libc::ENOENT)
        }
    }

    fn write_directory_content(&self, inode: Inode, entries: DirectoryDescriptor) {
        let path = Path::new(&self.data_dir)
            .join("contents")
            .join(inode.to_string());
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .unwrap();
        bincode::serialize_into(file, &entries).unwrap();
    }

    fn get_inode(&self, inode: Inode) -> Result<InodeAttributes, c_int> {
        let path = Path::new(&self.data_dir)
            .join("inodes")
            .join(inode.to_string());
        if let Ok(file) = File::open(path) {
            Ok(bincode::deserialize_from(file).unwrap())
        } else {
            Err(libc::ENOENT)
        }
    }

    fn write_inode(&self, inode: &InodeAttributes) {
        let path = Path::new(&self.data_dir)
            .join("inodes")
            .join(inode.inode.to_string());
        //Put the chunking and indexing staff here
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .unwrap();
        bincode::serialize_into(file, inode).unwrap();
    }

    // Check whether a file should be removed from storage. Should be called after decrementing
    // the link count, or closing a file handle
    fn gc_inode(&self, inode: &InodeAttributes) -> bool {
        if inode.hardlinks == 0 && inode.open_file_handles == 0 {
            let inode_path = Path::new(&self.data_dir)
                .join("inodes")
                .join(inode.inode.to_string());
            fs::remove_file(inode_path).unwrap();
            let content_path = Path::new(&self.data_dir)
                .join("contents")
                .join(inode.inode.to_string());
            fs::remove_file(content_path).unwrap();

            return true;
        }

        return false;
    }

    fn truncate(
        &self,
        inode: Inode,
        new_length: u64,
        uid: u32,
        gid: u32,
    ) -> Result<InodeAttributes, c_int> {
        if new_length > MAX_FILE_SIZE {
            return Err(libc::EFBIG);
        }

        let mut attrs = self.get_inode(inode)?;

        // if !check_access(attrs.uid, attrs.gid, attrs.mode, uid, gid, libc::W_OK) {
        //     return Err(libc::EACCES);
        // }

        let path = self.content_path(inode);
        let file = OpenOptions::new().write(true).open(path).unwrap();
        file.set_len(new_length).unwrap();

        attrs.size = new_length;
        attrs.last_metadata_changed = time_now();
        attrs.last_modified = time_now();

        // Clear SETUID & SETGID on truncate
        // clear_suid_sgid(&mut attrs);

        self.write_inode(&attrs);

        Ok(attrs)
    }
    fn lookup_name(&self, parent: u64, name: &OsStr) -> Result<InodeAttributes, c_int> {
        let entries = self.get_directory_content(parent)?;
        if let Some((inode, _)) = entries.get(name.as_bytes()) {
            return self.get_inode(*inode);
        } else {
            return Err(libc::ENOENT);
        }
    }
}

pub fn fuse_main() {
    // let matches = Command::new("hello")
    //     .author("Christopher Berner")
    //     .arg(
    //         Arg::new("MOUNT_POINT")
    //             .required(true)
    //             .index(1)
    //             .help("Act as a client, and mount FUSE at given path"),
    //     )
    //     .arg(
    //         Arg::new("auto_unmount")
    //             .long("auto_unmount")
    //             .action(ArgAction::SetTrue)
    //             .help("Automatically unmount on process exit"),
    //     )
    //     .arg(
    //         Arg::new("allow-root")
    //             .long("allow-root")
    //             .action(ArgAction::SetTrue)
    //             .help("Allow root user to access filesystem"),
    //     )
    //     .get_matches();
    env_logger::init();
    let mountpoint = "/home/weikang/Documents/serverlessinterface/testfolder";
    let data_dir = "data-dir".to_string();
    // let filesystem = MyFS; //This is the only thing need to test
    let mut options = vec![MountOption::FSName("fuser".to_string())];
    options.push(MountOption::AutoUnmount);
    let result = fuser::mount2(
        MyFS::new(
            data_dir,
            true, //Direct IO means: file reads and writes go directly from the applications to the storage device, bypassing the operating system read and write caches.
            false, // Suid means: SUID, short for Set User ID, is a special permission that can be assigned to executable files.
        ),
        mountpoint,
        &options,
    );

    let mut options = vec![MountOption::RW, MountOption::FSName("hello".to_string())];

    options.push(MountOption::AllowOther);

    options.push(MountOption::AutoUnmount);

    // fuser::mount2(filesystem, mountpoint, &options).unwrap();
}
