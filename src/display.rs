use std::{
    mem::{self, MaybeUninit},
    ptr::{self, addr_of_mut},
};

use widestring::{U16CStr, U16CString};
use windows_sys::{
    core::PCWSTR,
    Win32::{
        Foundation::{self, POINTL},
        Graphics::Gdi::{
            self, ChangeDisplaySettingsW, EnumDisplayDevicesW, EnumDisplaySettingsW, DEVMODEW,
            DISPLAY_DEVICEW,
        },
    },
};

use crate::Error;

#[derive(Debug, Clone, Copy)]
pub struct DisplayConfig {
    pub refresh_rate: Option<u32>,
    pub position: Option<(i32, i32)>,
    pub size: Option<(u32, u32)>,
}

#[derive(Clone)]
pub struct Display {
    name: U16CString,
    inner: DEVMODEW,
}

impl Display {
    pub fn new(name: PCWSTR) -> Result<Self, Error> {
        let mut info = MaybeUninit::<DEVMODEW>::zeroed();
        unsafe {
            addr_of_mut!((*info.as_mut_ptr()).dmSize).write(mem::size_of::<DEVMODEW>() as u16);
        }

        match unsafe {
            EnumDisplaySettingsW(
                name,
                Gdi::ENUM_CURRENT_SETTINGS,
                &mut info as *mut _ as *mut _,
            )
        } {
            0 => Err(Error::last_os_error()),
            _ => Ok(Self {
                name: unsafe { U16CString::from_ptr_str(name) },
                inner: unsafe { info.assume_init() },
            }),
        }
    }

    pub fn from_index(index: u32) -> Result<Self, Error> {
        let mut info = MaybeUninit::<DISPLAY_DEVICEW>::zeroed();
        unsafe {
            addr_of_mut!((*info.as_mut_ptr()).cb).write(mem::size_of::<DISPLAY_DEVICEW>() as u32);
        }

        match unsafe {
            EnumDisplayDevicesW(
                ptr::null_mut() as PCWSTR,
                index,
                &mut info as *mut _ as *mut _,
                0,
            )
        } {
            0 => Err(Error::last_os_error()),
            _ => {
                let info = unsafe { info.assume_init() };
                Display::new(&info.DeviceName as PCWSTR)
            }
        }
    }

    /// Refresh rate of the display in hertz. This value may be out of date.  
    pub fn refresh_rate(&self) -> u32 {
        self.inner.dmDisplayFrequency
    }

    /// Size of the disdplay. This value may be out of date.
    pub fn size(&self) -> (u32, u32) {
        (self.inner.dmPelsWidth, self.inner.dmPelsHeight)
    }

    /// Name of the display. This value may be out of date.
    pub fn name(&self) -> &U16CStr {
        &self.name
    }

    // TODO: other props,
    // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-changedisplaysettingsw

    // TODO: `self.inner` should be cloned or something so that in the case of an error, the three
    // getter functions do not output incorrect values.
    /// Updates the display's configuration. 
    pub fn update(&mut self, config: DisplayConfig) -> Result<(), Error> {
        if let Some(refresh_rate) = config.refresh_rate {
            self.inner.dmFields |= Gdi::DM_DISPLAYFREQUENCY as u32;
            self.inner.dmDisplayFrequency = refresh_rate;
        }
        if let Some(size) = config.size {
            self.inner.dmFields |= Gdi::DM_PELSWIDTH as u32 | Gdi::DM_PELSHEIGHT as u32;
            self.inner.dmPelsWidth = size.0;
            self.inner.dmPelsHeight = size.1;
        }
        if let Some(position) = config.position {
            self.inner.dmFields |= Gdi::DM_POSITION as u32;
            self.inner.Anonymous1.Anonymous2.dmPosition = POINTL {
                x: position.0,
                y: position.1,
            };
        }

        match unsafe { ChangeDisplaySettingsW(&self.inner as *const _, 0) } {
            Gdi::DISP_CHANGE_SUCCESSFUL => Ok(()),
            _ => Err(Error::last_os_error()),
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
        match Display::from_index(self.index) {
            Ok(display) => {
                self.index += 1;
                Some(Ok(display))
            }
            Err(err) => match err {
                Error::Win32(err) => {
                    if err.raw_os_error() == Some(Foundation::ERROR_INVALID_HANDLE as i32) {
                        // in this case the handle is invalid and there are no more displays'
                        None
                    } else {
                        Some(Err(Error::Win32(err)))
                    }
                }
            },
        }
    }
}
