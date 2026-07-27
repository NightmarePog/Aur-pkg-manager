use std::ffi::OsString;

mod runner;
use runner::Bwrap;

pub struct Builder(Vec<OsString>);

impl Builder {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    fn arg(&mut self, arg: impl Into<OsString>) -> &mut Self {
        self.0.push(arg.into());
        self
    }

    pub fn unshare_all(&mut self) -> &mut Self {
        self.arg("--unshare-all")
    }

    pub fn share_net(&mut self) -> &mut Self {
        self.arg("--share-net")
    }

    pub fn unshare_net(&mut self) -> &mut Self {
        self.arg("--unshare-net")
    }

    pub fn die_with_parent(&mut self) -> &mut Self {
        self.arg("--die-with-parent")
    }

    pub fn ro_bind(
        &mut self,
        src: impl Into<OsString>,
        dst: impl Into<OsString>,
    ) -> &mut Self {
        self.arg("--ro-bind")
            .arg(src)
            .arg(dst)
    }

    pub fn bind(
        &mut self,
        src: impl Into<OsString>,
        dst: impl Into<OsString>,
    ) -> &mut Self {
        self.arg("--bind")
            .arg(src)
            .arg(dst)
    }

    pub fn dir(&mut self, path: impl Into<OsString>) -> &mut Self {
        self.arg("--dir")
            .arg(path)
    }

    pub fn tmpfs(&mut self, path: impl Into<OsString>) -> &mut Self {
        self.arg("--tmpfs")
            .arg(path)
    }

    pub fn proc(&mut self, path: impl Into<OsString>) -> &mut Self {
        self.arg("--proc")
            .arg(path)
    }

    pub fn dev(&mut self, path: impl Into<OsString>) -> &mut Self {
        self.arg("--dev")
            .arg(path)
    }

    pub fn setenv(
        &mut self,
        key: impl Into<OsString>,
        value: impl Into<OsString>,
    ) -> &mut Self {
        self.arg("--setenv")
            .arg(key)
            .arg(value)
    }

    pub fn hostname(&mut self, name: impl Into<OsString>) -> &mut Self {
        self.arg("--hostname")
            .arg(name)
    }

    pub fn chdir(&mut self, path: impl Into<OsString>) -> &mut Self {
        self.arg("--chdir")
            .arg(path)
    }

    pub fn clearenv(&mut self) -> &mut Self {
        self.arg("--clearenv")
    }

    pub fn build(self) -> anyhow::Result<runner::Bwrap> {
        Ok(runner::Bwrap::new(self.0)?)
    }
}
