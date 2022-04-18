mod display;
mod options;

use std::io;

use clap::StructOpt;
use display::{DisplayConfig, DisplayIterator};
use thiserror::Error;
use widestring::U16String;

use crate::{
    display::Display,
    options::{Action, Options},
};

pub fn run(action: Action) -> Result<(), Error> {
    match action {
        Action::List => {
            for (index, display) in DisplayIterator::new().enumerate() {
                let display = display?;
                let size = display.size();

                // this adds an additional space between each display
                // it is done prior to ensure there is no trailing whitespace
                if index != 0 {
                    println!("");
                }

                println!(
                    "{}:
    Index: {}
    Refresh Rate: {}
    Size: {}x{}",
                    display.name().to_string_lossy(),
                    index,
                    display.refresh_rate(),
                    size.0,
                    size.1
                );
            }
        }
        Action::Set {
            name,
            index,
            refresh_rate,
            width,
            height,
            x,
            y,
        } => {
            let mut display = match name {
                Some(name) => {
                    let name = U16String::from_os_str(&name).as_ptr();
                    Display::new(name)?
                }
                // in this case the `index` is guaranteed to be specified
                None => {
                    let index = index.unwrap();
                    Display::from_index(index)?
                }
            };

            let size = display.size();
            let config = DisplayConfig {
                refresh_rate,
                position: if x.is_some() || y.is_some() {
                    Some((x.unwrap(), y.unwrap()))
                } else {
                    None
                },
                size: Some((width.unwrap_or(size.0), height.unwrap_or(size.1))),
            };

            display.update(config)?;
        }
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    let args = Options::parse();
    run(args.action)
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Win32(io::Error),
}

impl Error {
    pub fn last_os_error() -> Error {
        Error::Win32(io::Error::last_os_error())
    }
}
