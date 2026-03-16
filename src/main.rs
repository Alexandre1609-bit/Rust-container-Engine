use nix::errno::Errno;
use nix::libc::proc_cn_event;
use nix::mount::{MntFlags, MsFlags, mount};
use nix::sched::{CloneFlags, unshare};
use nix::unistd::Uid;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ContainerConfig::new("mini-nexus", "./rootfs", "/bin/busybox");

    init_rootfs(&config);
    let copy_folder = format!("{}/bin/busybox", config.root_path);
    fs::copy("/bin/busybox", copy_folder).expect("Failed to copy file");
    run_container(&config);
}

//Init base structure
fn init_rootfs(config: &ContainerConfig) -> Result<(), std::io::Error> {
    let folder_to_create = [
        Path::new(config.root_path.as_str()).join("bin"),
        Path::new(config.root_path.as_str()).join("etc"),
        Path::new(config.root_path.as_str()).join("proc"),
    ];

    for dir in &folder_to_create {
        fs::create_dir_all(dir)?;
    }

    let add_content = config.project_name.as_str();
    let hostname_file_path = format!("{}/etc/hostname", config.root_path);
    fs::write(hostname_file_path, add_content)?;
    Ok(())
}

//Install base components and run one container
fn run_container(config: &ContainerConfig) {
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

fn setup_namespaces() -> Result<(), nix::Error> {
    unshare(
        CloneFlags::CLONE_NEWPID
            | CloneFlags::CLONE_NEWNS
            | CloneFlags::CLONE_NEWUTS
            | CloneFlags::CLONE_NEWNET,
    )?;
    Ok(())
}

fn isolate_rootfs(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    chroot(path)?;
    env::set_current_dir("/")?;
    Ok(())
}

fn setup_hostname(hostname: &str) -> Result<(), nix::Error> {
    sethostname(OsStr::new(hostname))?;
    Ok(())
}

fn mount_proc() -> Result<(), Box<dyn std::error::Error>> {
    mount(
        None::<&str>,
        "/proc",
        Some("proc"),
        MsFlags::empty(),
        None::<&str>,
    )?;
    Ok(())
}

fn setup_loopback() -> Result<(), Box<dyn std::error::Error>> {
    // TODO : remplacer par rtnetlink une fois que j'aurais plus de compétences en rust et async rust
    Command::new("ip")
        .args(["link", "set", "lo", "up"])
        .status()?;
    Ok(())
}

fn run_container(config: &ContainerConfig) -> Result<(), Box<dyn std::error::Error>> {
    todo!();
    Ok(())
}

ok(())

