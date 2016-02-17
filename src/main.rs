extern crate docopt;
extern crate rustc_serialize;

use std::ffi::{OsString, OsStr};
use std::io::{self, BufRead, Write};
use std::path::Path;

use docopt::Docopt;

const VERTICAL_CHAR: char = '│';
const HORIZONTAL_STR: &'static str = "├──";
const LAST_HORIZONTAL_STR: &'static str = "└──";

const USAGE: &'static str = "
treeify converts the output of a command that lists files in a tree representation similar to the output of the command tree.

Usage:
  treeify [-0]
  treeify (-h | --help)

Options:
  -h --help  Display this message
  -0         Paths are separated by null characters instead of new lines
";

struct FileTree {
    name: OsString,
    childs: Vec<FileTree>,
}

fn print_line<W: Write>(output: &mut W, lasts: &[bool], name: &OsStr) -> io::Result<()> {
    let name = format!("{:?}", name);
    // Remove the quotes
    let name = &name[1..name.len()-1];

    if lasts.len() == 0 {
        try!(writeln!(output, "{}", name));
    } else {
        for last in &lasts[..lasts.len() - 1] {
            let c = if *last {
                ' '
            } else {
                VERTICAL_CHAR
            };
            try!(write!(output, "{}   ", c));
        }
        if *lasts.last().unwrap() {
            try!(writeln!(output, "{} {}", LAST_HORIZONTAL_STR, name));
        } else {
            try!(writeln!(output, "{} {}", HORIZONTAL_STR, name));
        }
    }
    Ok(())
}

impl FileTree {
    fn print<W: Write>(&self, out: &mut W, lasts: &mut Vec<bool>) -> io::Result<()> {
        try!(print_line(out, &lasts[..], &*self.name));
        lasts.push(false);
        for (i, child) in self.childs.iter().enumerate() {
            if i + 1 == self.childs.len() {
                *lasts.last_mut().unwrap() = true;
            }
            try!(child.print(out, lasts));
        }
        lasts.pop();
        Ok(())
    }

    fn add<'a, I: Iterator<Item = &'a OsStr>>(&mut self, name_iter: &mut I) {
        if let Some(c) = name_iter.next() {
            let mut found = false;
            for child in &mut self.childs {
                if &*child.name == c {
                    child.add(name_iter);
                    found = true;
                    break;
                }
            }

            if !found {
                let new_child = FileTree {
                    name: c.to_os_string(),
                    childs: vec![],
                };
                self.childs.push(new_child);
                self.childs.last_mut().unwrap().add(name_iter);
            }
        }
    }
}

fn make_trees<I, O>(input: &mut I) -> io::Result<Vec<FileTree>>
    where I: Iterator<Item = O>,
          O: AsRef<OsStr>
{
    let mut pseudo_root = FileTree {
        name: OsString::new(),
        childs: vec![],
    };

    for line in input {
        let path = Path::new(&line);
        let mut bar = path.components().map(|c| c.as_os_str());
        pseudo_root.add(&mut bar);
    }

    Ok(pseudo_root.childs)
}

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_0: bool,
}

fn main() {

    let args: Args = Docopt::new(USAGE)
                         .and_then(|d| d.decode())
                         .unwrap_or_else(|e| e.exit());
    let stdin = io::stdin();
    let trees = if args.flag_0 {
        let mut input = stdin.lock()
                             .split(0)
                             .map(|l| String::from_utf8_lossy(&*l.unwrap()).into_owned());
        make_trees(&mut input).unwrap()
    } else {
        let mut input = stdin.lock().lines().map(|l| l.unwrap());
        make_trees(&mut input).unwrap()
    };

    let mut stdout = io::stdout();
    let mut v = Vec::new();
    for tree in trees {
        tree.print(&mut stdout, &mut v).unwrap();
    }
}
