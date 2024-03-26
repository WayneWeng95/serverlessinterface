use fuser::{FileType, Filesystem, MountOption, ReplyAttr, ReplyData, ReplyWrite, Request};
use libc;
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use std::cmp::min;
use std::collections::BTreeMap;
use std::ffi::OsStr;
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
type Inode = u64;

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

fn clear_suid_sgid(attr: &mut InodeAttributes) {
    attr.mode &= !libc::S_ISUID as u16;
    // SGID is only suppose to be cleared if XGRP is set
    if attr.mode & libc::S_IXGRP as u16 != 0 {
        attr.mode &= !libc::S_ISGID as u16;
    }
}

impl Filesystem for MyFS {
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

            clear_suid_sgid(&mut attrs);
            self.write_inode(&attrs);

            reply.written(data.len() as u32);
        } else {
            reply.error(libc::EBADF);
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
}

fn fuse_main() {
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
