use std::fs;

fn main() {
    init_rootfs();
}

fn init_rootfs() {
    let folder_to_create = ["rootfs/bin", "rootfs/etc"];

    for dir in &folder_to_create {
        fs::create_dir_all(dir).expect("An error occured while creating the folder")
    }
}
