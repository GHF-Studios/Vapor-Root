#!/usr/bin/env -S rust-script --force

//! Install or reconcile the app-root player tooling layer.
//!
//! ```cargo
//! [package]
//! edition = "2021"
//! ```

#[path = "../../_shared/vapor_tools.rs"]
mod vapor_tools;

fn main() {
    let _cache_buster = "app-root-discovery-v2";
    vapor_tools::exit(vapor_tools::setup_player_main());
}
