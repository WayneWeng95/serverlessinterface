use fuser::{FileType, Filesystem, MountOption, ReplyAttr, Request};
use std::ffi::OsStr;
use std::time::{Duration, SystemTime};

struct MyFS;

impl Filesystem for MyFS {
    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        let attr = fuser::FileAttr {
            ino: 1,
            size: 4096000000,
            blksize: 4096000,
            padding: 0,
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

    // fn read(
    //     &mut self,
    //     _req: &Request,
    //     inode: u64,
    //     fh: u64,
    //     offset: i64,
    //     size: u32,
    //     _flags: i32,
    //     _lock_owner: Option<u64>,
    //     reply: ReplyData,
    // ) {
    //     debug!(
    //         "read() called on {:?} offset={:?} size={:?}",
    //         inode, offset, size
    //     );
    //     assert!(offset >= 0);
    //     if !self.check_file_handle_read(fh) {
    //         reply.error(libc::EACCES);
    //         return;
    //     }

    //     let path = self.content_path(inode);
    //     if let Ok(file) = File::open(path) {
    //         let file_size = file.metadata().unwrap().len();
    //         // Could underflow if file length is less than local_start
    //         let read_size = min(size, file_size.saturating_sub(offset as u64) as u32);

    //         let mut buffer = vec![0; read_size as usize];
    //         file.read_exact_at(&mut buffer, offset as u64).unwrap();
    //         reply.data(&buffer);
    //     } else {
    //         reply.error(libc::ENOENT);
    //     }
    // }

    // fn write(
    //     &mut self,
    //     _req: &Request,
    //     inode: u64,
    //     fh: u64,
    //     offset: i64,
    //     data: &[u8],
    //     _write_flags: u32,
    //     #[allow(unused_variables)] flags: i32,
    //     _lock_owner: Option<u64>,
    //     reply: ReplyWrite,
    // ) {
    //     debug!("write() called with {:?} size={:?}", inode, data.len());
    //     assert!(offset >= 0);
    //     if !self.check_file_handle_write(fh) {
    //         reply.error(libc::EACCES);
    //         return;
    //     }

    //     let path = self.content_path(inode);
    //     if let Ok(mut file) = OpenOptions::new().write(true).open(path) {
    //         file.seek(SeekFrom::Start(offset as u64)).unwrap();
    //         file.write_all(data).unwrap();

    //         let mut attrs = self.get_inode(inode).unwrap();
    //         attrs.last_metadata_changed = time_now();
    //         attrs.last_modified = time_now();
    //         if data.len() + offset as usize > attrs.size as usize {
    //             attrs.size = (data.len() + offset as usize) as u64;
    //         }
    //         // #[cfg(feature = "abi-7-31")]
    //         // if flags & FUSE_WRITE_KILL_PRIV as i32 != 0 {
    //         //     clear_suid_sgid(&mut attrs);
    //         // }
    //         // XXX: In theory we should only need to do this when WRITE_KILL_PRIV is set for 7.31+
    //         // However, xfstests fail in that case
    //         clear_suid_sgid(&mut attrs);
    //         self.write_inode(&attrs);

    //         reply.written(data.len() as u32);
    //     } else {
    //         reply.error(libc::EBADF);
    //     }
    // }

    // Implement other filesystem methods as needed
}

use clap::{Arg, ArgAction, Command};

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
    let filesystem = MyFS;
    let mut options = vec![MountOption::RW, MountOption::FSName("hello".to_string())];

    options.push(MountOption::AllowOther);

    options.push(MountOption::AutoUnmount);

    fuser::mount2(filesystem, mountpoint, &options).unwrap();
}