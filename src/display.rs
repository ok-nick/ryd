use std::{io, mem::MaybeUninit, ptr};

use windows_sys::{
    core::PCWSTR,
    Win32::{
        Foundation::POINTL,
        Graphics::Gdi::{
            self, ChangeDisplaySettingsW, EnumDisplayDevicesW, EnumDisplaySettingsW, DEVMODEW,
            DISPLAY_DEVICEW,
        },
    },
};

use crate::Error;

#[derive(Clone)]
pub struct Display {
    inner: DEVMODEW,
}

impl Display {
    pub fn new(name: PCWSTR) -> Result<Self, Error> {
        let mut info = MaybeUninit::<DEVMODEW>::zeroed();
        match unsafe {
            EnumDisplaySettingsW(
                name,
                Gdi::ENUM_CURRENT_SETTINGS,
                &mut info as *mut _ as *mut _,
            )
        } {
            0 => Err(Error::Win32(io::Error::last_os_error())),
            _ => Ok(Self {
                inner: unsafe { info.assume_init() },
            }),
        }
    }

    pub fn set_refresh_rate(&mut self, refresh_rate: u32) {
        self.inner.dmDisplayFrequency = refresh_rate;
    }

    pub fn set_width(&mut self, width: u32) {
        self.inner.dmPelsWidth = width
    }

    pub fn set_height(&mut self, height: u32) {
        self.inner.dmPelsHeight = height;
    }

    pub fn set_position(&mut self, position: (i32, i32)) {
        self.inner.Anonymous1.Anonymous2.dmPosition = POINTL {
            x: position.0,
            y: position.1,
        };
    }

    // TODO: other props,
    // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-changedisplaysettingsw

    pub fn update(&self) -> Result<(), Error> {
        match unsafe { ChangeDisplaySettingsW(&self.inner as *const _, 0) } {
            Gdi::DISP_CHANGE_SUCCESSFUL => Ok(()),
            _ => Err(Error::Win32(io::Error::last_os_error())),
        }
    }
}

// TODO
// impl Debug for Display {}

#[derive(Debug)]
pub struct DisplayIterator {
    index: u32,
}

impl DisplayIterator {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Iterator for DisplayIterator {
    type Item = Result<Display, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut info = MaybeUninit::<DISPLAY_DEVICEW>::zeroed();
        let success = unsafe {
            EnumDisplayDevicesW(
                ptr::null_mut() as PCWSTR,
                self.index,
                &mut info as *mut _ as *mut _,
                0,
            )
        };

        match success {
            0 => None,
            _ => {
                self.index += 1;

                let info = unsafe { info.assume_init() };
                Some(Display::new(&info.DeviceName as PCWSTR))
            }
        }
    }
}
