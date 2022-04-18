use std::ffi::OsString;

use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
pub struct Options {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Action {
    /// Lists available display names
    List,
    /// Set the config for a display
    Set {
        /// Name of the display
        #[clap(short, long, group = "id")]
        name: Option<OsString>,
        /// Index of the monitor
        #[clap(short, long, group = "id")]
        index: Option<u32>,
        /// Refresh rate in hertz
        #[clap(short('r'), long)]
        refresh_rate: Option<u32>,
        /// Resolution width in pixels
        #[clap(long)]
        width: Option<u32>,
        /// Resolution height in pixels
        #[clap(long)]
        height: Option<u32>,
        ///  `x` position of the display
        #[clap(short, long, requires = "y")]
        x: Option<i32>,
        /// `y` position of the display
        #[clap(short, long, requires = "x")]
        y: Option<i32>,
    },
}
