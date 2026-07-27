#!/usr/bin/env -S rust-script --force

//! Create a Vapor SuperWorkspace shape from the installed app root.
//!
//! ```cargo
//! [package]
//! edition = "2021"
//! ```

#[path = "../../_shared/vapor_tools.rs"]
mod vapor_tools;

fn main() {
    let _cache_buster = "superworkspace-create-v1";
    vapor_tools::exit(vapor_tools::create_superworkspace_main());
}
