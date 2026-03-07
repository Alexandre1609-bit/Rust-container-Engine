use nix::sched::{CloneFlags, unshare};
use nix::unistd::sethostname;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::os::unix::fs::chroot;
use std::process::Command;

fn main() {
    init_rootfs();
    fs::copy("/bin/busybox", "rootfs/bin/busybox").expect("Failed to copy file");
    run_container();
}

//Init base structure
fn init_rootfs() {
    let folder_to_create = ["rootfs/bin", "rootfs/etc", "rootfs/proc"];

    for dir in &folder_to_create {
        fs::create_dir_all(dir).expect("An error occured while creating the folder")
    }

    let add_content = "Lorem Ipsum\n";
    fs::write("rootfs/etc/hostname", add_content).expect("An error occured while creatin the file")
}

//Install base components and run one container
fn run_container() {
    unshare(CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNS | CloneFlags::CLONE_NEWUTS)
        .expect("Failed to create namespace PID");
    chroot("./rootfs").expect("An error occured while doing 'chroot'");
    env::set_current_dir("/").expect("An error occured while transfering to the the root");
    sethostname(OsStr::new("mini-nexus")).expect("Error : failed to change hostname");
    Command::new("/bin/busybox")
        .arg("sh")
        .arg("-c")
        .arg("/bin/busybox --install -s /bin && mount -t proc proc /proc && exec sh")
        .status()
        .expect("Failed to run container");
}
