#!/usr/bin/env -S rust-script --force

//! Run a RustRover workflow through app-root Vapor tools.
//!
//! ```cargo
//! [package]
//! edition = "2024"
//! ```

#[path = "../../_shared/vapor_tools.rs"]
mod vapor_tools;

fn main() {
    let _cache_buster = "ide-run-app-root-tools-v1";
    vapor_tools::exit(vapor_tools::ide_run_main());
}
