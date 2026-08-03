use std::io::{self, BufRead, Write};

use crate::ui::UiError;

pub fn prompt() -> Result<String, UiError> {
    io::stdout().flush()?;

    if let Some(input) = io::stdin().lock().lines().next().transpose()? {
        Ok(input)
    } else {
        Err(UiError::StdinClosed)
    }
}
