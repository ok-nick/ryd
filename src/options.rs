use std::ffi::OsString;

use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
pub struct Options {
    // install, uninstall, list, set
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Action {
    /// Installs the Windows Service
    Install,
    /// Uninstall the Windows Service
    Uninstall {
        /// Deletes the configuration file
        #[clap(short, long)]
        delete_config: bool,
    },
    /// Reapplies the display config
    Update,
    /// Lists available display names
    List {
        // TODO: some configs, like associating the name with the index, their position
    },
    /// Sets a display's configuration
    Set {
        /// The name of the display
        // TODO: they should be able to specify an index
        name: OsString,
        /// The refresh rate in hertz
        #[clap(short('r'), long)]
        refresh_rate: Option<u32>,
        /// The resolution width in pixels
        #[clap(short, long)]
        width: Option<u32>,
        /// The resolution height in pixels
        #[clap(short, long)]
        height: Option<u32>,
        /// The x position of the display
        #[clap(short, long, requires = "y")]
        x: Option<i32>,
        /// The y position of the display
        #[clap(short, long, requires = "x")]
        y: Option<i32>,
    },
}
