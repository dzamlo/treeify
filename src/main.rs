extern crate clap;

use std::io::{self, BufRead};

mod cli;
mod filetree;

fn main() {
    let matches = cli::build_cli().get_matches();
    let stdin = io::stdin();
    let trees = if matches.is_present("null") {
        let mut input = stdin.lock()
            .split(0)
            .map(|l| String::from_utf8_lossy(&*l.unwrap()).into_owned());
        filetree::make_trees(&mut input)
    } else {
        let mut input = stdin.lock().lines().map(|l| l.unwrap());
        filetree::make_trees(&mut input)
    };

    let mut stdout = io::stdout();
    let mut v = Vec::new();
    for tree in trees {
        tree.print(&mut stdout, &mut v).unwrap();
    }
}
