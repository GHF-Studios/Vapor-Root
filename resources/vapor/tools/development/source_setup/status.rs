#!/usr/bin/env -S rust-script --force

//! Show the local source checkout state for the super-workspace.
//!
//! ```cargo
//! [package]
//! edition = "2021"
//! ```

#[path = "../../_shared/vapor_tools.rs"]
mod vapor_tools;

fn main() {
    let _cache_buster = "app-root-discovery-v2";
    vapor_tools::exit(vapor_tools::source_status_main());
}
