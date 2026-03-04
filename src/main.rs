use std::env;
use std::fs;
use std::os::unix::fs::chroot;
use std::process::Command;

fn main() {
    init_rootfs();
    fs::copy("/bin/busybox", "rootfs/bin/busybox").expect("Failed to copy file");
    run_container();
}

fn init_rootfs() {
    let folder_to_create = ["rootfs/bin", "rootfs/etc"];

    for dir in &folder_to_create {
        fs::create_dir_all(dir).expect("An error occured while creating the folder")
    }

    let add_content = "Lorem Ipsum\n";
    fs::write("rootfs/etc/hostname", add_content).expect("An error occured while creatin the file")
}

fn run_container() {
    chroot("./rootfs").expect("An error occured while doing 'chroot'");
    env::set_current_dir("/").expect("An error occured while transfering to the the root");

    Command::new("/bin/busybox")
        .args(["--install", "-s", "/bin"])
        .status()
        .expect("Failed to install shortcuts");

    Command::new("/bin/busybox")
        .arg("sh")
        .status()
        .expect("failed to execute process");
}
