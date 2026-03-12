use nix::sched::{CloneFlags, unshare};
use nix::unistd::sethostname;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::os::unix::fs::chroot;
use std::path::Path;
use std::process::Command;

struct ContainerConfig {
    project_name: String,
    root_path: String,
    run: String,
}

impl ContainerConfig {
    fn new(name: &str, path: &str, cmd: &str) -> Self {
        ContainerConfig {
            project_name: name.to_string(),
            root_path: path.to_string(),
            run: cmd.to_string(),
        }
    }
}

fn main() {
    let config = ContainerConfig::new("mini-nexus", "./rootfs", "/bin/busybox");

    init_rootfs(&config);
    let copy_folder = format!("{}/bin/busybox", config.root_path);
    fs::copy("/bin/busybox", copy_folder).expect("Failed to copy file");
    run_container(&config);
}

//Init base structure
fn init_rootfs(config: &ContainerConfig) {
    let folder_to_create = [
        Path::new(config.root_path.as_str()).join("bin"),
        Path::new(config.root_path.as_str()).join("etc"),
        Path::new(config.root_path.as_str()).join("proc"),
    ];

    for dir in &folder_to_create {
        fs::create_dir_all(dir).expect("An error occured while creating the folder")
    }

    let add_content = config.project_name.as_str();
    let hostname_file_path = format!("{}/etc/hostname", config.root_path.to_string());
    fs::write(hostname_file_path, add_content).expect("An error occured while creatin the file")
}

//Install base components and run one container
fn run_container(config: &ContainerConfig) {
    unshare(
        CloneFlags::CLONE_NEWPID
            | CloneFlags::CLONE_NEWNS
            | CloneFlags::CLONE_NEWUTS
            | CloneFlags::CLONE_NEWNET,
    )
    .expect("Failed to create namespace PID");
    chroot(config.root_path.as_str()).expect("An error occured while doing 'chroot'");
    env::set_current_dir("/").expect("An error occured while transfering to the the root");
    sethostname(OsStr::new(config.project_name.as_str()))
        .expect("Error : failed to change hostname");
    Command::new(config.run.as_str())
        .arg("sh")
        .arg("-c")
        .arg(format!(
            "{} --install -s /bin && mount -t proc proc /proc && ip link set lo up && exec sh",
            config.run
        ))
        .status()
        .expect("Failed to run container");
}
