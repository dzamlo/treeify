use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new("treeify")
        .version(env!("CARGO_PKG_VERSION"))
        .about("treeify converts the output of a command that lists files in a tree \
                representation similar to the output of the command tree.")
        .arg(Arg::with_name("null")
            .long("null")
            .short("0")
            .help("Paths are separated by null characters instead of new lines"))
}
