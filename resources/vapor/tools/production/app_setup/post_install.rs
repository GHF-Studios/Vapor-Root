#!/usr/bin/env -S rust-script --force

//! Record the local Vapor app-root anchor.
//!
//! ```cargo
//! [package]
//! edition = "2021"
//! ```

#[path = "../../_shared/vapor_tools.rs"]
mod vapor_tools;

fn main() {
    let _cache_buster = "app-root-discovery-v2";
    vapor_tools::exit(vapor_tools::post_install_main());
}
