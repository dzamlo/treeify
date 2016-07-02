extern crate clap;

use clap::Shell;
use std::env;

include!("src/cli.rs");

fn main() {
    let mut app = build_cli();
    app.gen_completions("treeify", Shell::Bash, env::var("OUT_DIR").unwrap());
}
