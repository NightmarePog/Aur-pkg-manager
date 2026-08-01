use std::{ffi::OsString, process::{Child, Command}};

use super::SpawnError;

pub struct Bwrap(Child);

impl Bwrap {
    pub fn new(args:  Vec<OsString>) -> Result<Bwrap, SpawnError> {
        let child = Command::new("bwrap")
            .args(args)
            .spawn()?;

        Ok(Bwrap(child))
    }
}
