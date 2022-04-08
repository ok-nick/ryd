mod config;
mod display;
mod options;
mod service;

use std::{ffi::OsString, io, os::windows::prelude::OsStrExt};

use clap::StructOpt;
use thiserror::Error;
use windows_service::{define_windows_service, service_dispatcher};
use windows_sys::Win32::Foundation;

use crate::{
    display::Display,
    options::{Action, Options},
};

define_windows_service!(ffi_main, service_main);

fn service_main(arguments: Vec<OsString>) {}

fn ran_as_service() -> windows_service::Result<bool> {
    match service_dispatcher::start("ahz", ffi_main) {
        Ok(_) => Ok(true),
        Err(err) => {
            if let windows_service::Error::Winapi(ref err) = err {
                if let Some(code) = err.raw_os_error() {
                    if code == Foundation::ERROR_FAILED_SERVICE_CONTROLLER_CONNECT as i32 {
                        return Ok(false);
                    }
                }
            }

            Err(err)
        }
    }
}

fn main() {
    // config file should be at %LOCALAPPDATA%

    match ran_as_service() {
        Ok(is_service) => match is_service {
            true => {}
            false => {
                let args = Options::parse();
                match args.action {
                    Action::Install => {}
                    Action::Uninstall { delete_config } => {}
                    Action::Update => {}
                    Action::List {} => {}
                    Action::Set {
                        name,
                        refresh_rate,
                        width,
                        height,
                        x,
                        y,
                    } => {
                        let name = name.encode_wide().collect::<Vec<u16>>().as_ptr();
                        // TODO: remove unwrap
                        let mut display = Display::new(name).unwrap();
                        if let Some(refresh_rate) = refresh_rate {
                            display.set_refresh_rate(refresh_rate);
                        }
                        if let Some(width) = width {
                            display.set_width(width);
                        }
                        if let Some(height) = height {
                            display.set_height(height);
                        }

                        // x is required when y is specified and y is required when x is specified
                        if x.is_some() || y.is_some() {
                            display.set_position((x.unwrap(), y.unwrap()))
                        }

                        // TODO: handle result
                        display.update().unwrap();
                    }
                }
            }
        },
        // TODO
        Err(err) => Err(err).unwrap(),
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Win32(io::Error),
}
