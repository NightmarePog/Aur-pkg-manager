use std::{ffi::OsString, process::{Child, Command}};

pub struct Bwrap(Child);

impl Bwrap {
    pub fn new(args:  Vec<OsString>) -> anyhow::Result<Bwrap> {
        let child = Command::new("bwrap")
            .args(args)
            .spawn()?;

        Ok(Bwrap(child))
    }
}
