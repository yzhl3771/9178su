use std::ffi::CString;
use std::io::{ErrorKind, Write};

use anyhow::{Context, Result};
use rustix::fs::{Mode, symlink, unlink};
use rustix::{
    fd::AsFd,
    fs::{Access, CWD, FileType, access, makedev, mkdir, mknodat},
    mount::{
        FsMountFlags, FsOpenFlags, MountAttrFlags, MoveMountFlags, UnmountFlags, fsconfig_create,
        fsmount, fsopen, move_mount, unmount,
    },
};

#[cfg(target_os = "android")]
unsafe extern "C" {
    fn fork() -> i32;
}

fn persist_load_error(msg: &str) {
    // Best effort: also keep a copy in the rootfs ramdisk.
    let _ = std::fs::write("/9178su_load_error.txt", msg);

    // Write to pstore (survives reboot); readable afterwards via
    // /sys/fs/pstore/pmsg-ramoops-0 with root access.
    for dev in ["/dev/pmsg0", "/dev/pmsg"] {
        if let Ok(mut f) = std::fs::OpenOptions::new().write(true).open(dev) {
            let _ = f.write_all(msg.as_bytes());
            let _ = f.write_all(b"\n");
            break;
        }
    }

    #[cfg(target_os = "android")]
    unsafe {
        // Fork a helper so it can keep running while the real init takes over.
        // It waits for /data to be available and then saves the error where
        // adb shell can read it.
        if fork() == 0 {
            for _ in 0..300 {
                if std::path::Path::new("/data/local/tmp").is_dir() {
                    if std::fs::write("/data/local/tmp/9178su_load_error.log", msg).is_ok() {
                        break;
                    }
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            std::process::exit(0);
        }
    }
}

struct AutoUmount {
    mountpoints: Vec<String>,
}

impl Drop for AutoUmount {
    fn drop(&mut self) {
        for mountpoint in self.mountpoints.iter().rev() {
            if let Err(e) = unmount(mountpoint.as_str(), UnmountFlags::DETACH) {
                log::error!("Cannot umount {}: {}", mountpoint, e)
            }
        }
    }
}

fn mount_filesystem(name: &str, mountpoint: &str) -> Result<()> {
    mkdir(mountpoint, Mode::from_raw_mode(0o755)).or_else(|err| match err.kind() {
        ErrorKind::AlreadyExists => Ok(()),
        _ => Err(err),
    })?;
    let fs_fd = fsopen(name, FsOpenFlags::FSOPEN_CLOEXEC)?;
    fsconfig_create(fs_fd.as_fd())?;
    let mount_fd = fsmount(
        fs_fd.as_fd(),
        FsMountFlags::FSMOUNT_CLOEXEC,
        MountAttrFlags::empty(),
    )?;
    move_mount(
        mount_fd.as_fd(),
        "",
        CWD,
        mountpoint,
        MoveMountFlags::MOVE_MOUNT_F_EMPTY_PATH,
    )?;
    Ok(())
}

fn prepare_mount() -> AutoUmount {
    let mut mountpoints = vec![];

    // mount procfs
    match mount_filesystem("proc", "/proc") {
        Ok(_) => mountpoints.push("/proc".to_string()),
        Err(e) => log::error!("Cannot mount procfs: {:?}", e),
    }

    AutoUmount { mountpoints }
}

fn setup_kmsg() {
    const KMSG: &str = "/dev/kmsg";
    let device = match access(KMSG, Access::EXISTS) {
        Ok(_) => KMSG,
        Err(_) => {
            // try to create it
            mknodat(
                CWD,
                "/kmsg",
                FileType::CharacterDevice,
                0o666.into(),
                makedev(1, 11),
            )
            .ok();
            "/kmsg"
        }
    };

    let _ = kernlog::init_with_device(device);
}

fn unlimit_kmsg() {
    // Disable kmsg rate limiting
    if let Ok(mut rate) = std::fs::File::options()
        .write(true)
        .open("/proc/sys/kernel/printk_devkmsg")
    {
        writeln!(rate, "on").ok();
    }
}

pub fn init() -> Result<()> {
    // Setup kernel log first
    setup_kmsg();

    log::info!("Hello, 9178su!");

    // mount /proc to access kernel interface
    let _dontdrop = prepare_mount();

    // This relies on the fact that we have /proc mounted
    unlimit_kmsg();

    if n9178suinit::has_kernelsu() {
        log::info!("9178su may be already loaded in kernel, skip!");
    } else {
        log::info!("Loading 9178su.ko..");
        if let Err(e) = load_module_from_path("/9178su.ko") {
            let msg = format!("Cannot load 9178su.ko: {:?}\n", e);
            log::error!("{}", msg.trim_end());
            persist_load_error(&msg);
        }
    }

    // And now we should prepare the real init to transfer control to it
    unlink("/init")?;

    let real_init = match access("/init.real", Access::EXISTS) {
        Ok(_) => "init.real",
        Err(_) => "/system/bin/init",
    };

    log::info!("init is {}", real_init);
    symlink(real_init, "/init")?;

    Ok(())
}

fn load_module_from_path(path: &str) -> Result<()> {
    anyhow::ensure!(rustix::process::getpid().is_init(), "Invalid process");
    let buffer = std::fs::read(path).with_context(|| format!("Cannot read file {}", path))?;
    let params = std::fs::read("/n9178su_config").unwrap_or_default();
    let params = unsafe { CString::from_vec_unchecked(params) };
    log::info!("load 9178su with params {params:?}");
    n9178suinit::load_module(&buffer, &params)
}
