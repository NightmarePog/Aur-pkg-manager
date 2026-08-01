use std::{fs, path::PathBuf};

use thiserror::Error;

use crate::{bwrap, config};

const SANDBOX_NAME: &str = "aur-pkg-manager";

#[derive(Debug, Error)]
pub enum SandboxError {
    #[error("failed to prepare the sandbox")]
    Io(#[from] std::io::Error),

    #[error("failed to locate the user data directory")]
    MissingDataDir,

    #[error(transparent)]
    Spawn(#[from] bwrap::SpawnError),
}

fn sandbox_path() -> Result<PathBuf, SandboxError> {
    dirs::data_dir()
        .map(|path| path.join(SANDBOX_NAME))
        .ok_or(SandboxError::MissingDataDir)
}

pub struct SandboxFiles {
    path: PathBuf,
}

impl SandboxFiles {
    pub fn initialize() -> Result<Self, SandboxError> {
        let path = sandbox_path()?;

        fs::create_dir_all(&path)?;

        Self::copy_if_missing("/etc/pacman.conf", path.join("pacman.conf"))?;
        Self::copy_if_missing(
            "/etc/pacman.d/mirrorlist",
            path.join("mirrorlist"),
        )?;
        Self::copy_if_missing(
            "/etc/makepkg.conf",
            path.join("makepkg.conf"),
        )?;

        if !path.join("passwd").exists() {
            fs::write(
                path.join("passwd"),
                "builder:x:1000:1000:builder:/build:/bin/bash\n",
            )?;
        }

        if !path.join("group").exists() {
            fs::write(
                path.join("group"),
                "builder:x:1000:\n",
            )?;
        }

        Ok(Self { path })
    }

    fn copy_if_missing(from: &str, to: PathBuf) -> Result<(), SandboxError> {
        if !to.exists() {
            fs::copy(from, to)?;
        }

        Ok(())
    }

    fn file(&self, name: &str) -> PathBuf {
        self.path.join(name)
    }
}

pub struct Environment {
    bwrap: bwrap::runner::Bwrap,
}

impl Environment {
    pub fn new(sandbox: &SandboxFiles) -> Result<Self, SandboxError> {
        fs::create_dir_all(config::BUILD_PATH)?;

        let mut builder = bwrap::Builder::new();

        builder
            .unshare_all()
            .die_with_parent()
            .proc("/proc")
            .dev("/dev")
            .tmpfs("/tmp")

            .clearenv()
            .setenv("HOME", "/build")
            .setenv(
                "PATH",
                "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
            )
            .setenv("MAKEFLAGS", "-j4")

            // build output
            .bind(config::BUILD_PATH, "/build/pkg")

            // system
            .ro_bind("/usr", "/usr")
            .ro_bind("/bin", "/bin")
            .ro_bind("/lib", "/lib")
            .ro_bind("/lib64", "/lib64")

            // pacman
            .ro_bind("/var/lib/pacman", "/var/lib/pacman")
            .bind("/var/cache/pacman/pkg", "/var/cache/pacman/pkg")

            // configs
            .ro_bind(sandbox.file("pacman.conf"), "/etc/pacman.conf")
            .ro_bind(sandbox.file("mirrorlist"), "/etc/pacman.d/mirrorlist")
            .ro_bind(sandbox.file("makepkg.conf"), "/etc/makepkg.conf")
            .ro_bind(sandbox.file("passwd"), "/etc/passwd")
            .ro_bind(sandbox.file("group"), "/etc/group")

            .chdir("/build/pkg")
            .makepkg();

        Ok(Self {
            bwrap: builder.build()?,
        })
    }
}
