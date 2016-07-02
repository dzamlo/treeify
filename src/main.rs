extern crate clap;

use std::ffi::{OsString, OsStr};
use std::io::{self, BufRead, Write};
use std::path::Path;

mod cli;

const VERTICAL_CHAR: char = '│';
const HORIZONTAL_STR: &'static str = "├──";
const LAST_HORIZONTAL_STR: &'static str = "└──";
const REPLACEMENT_CHAR: char = '?';

#[derive(Debug,PartialEq,Eq)]
struct FileTree {
    name: OsString,
    childs: Vec<FileTree>,
}

fn print_line<W: Write>(output: &mut W, lasts: &[bool], name: &OsStr) -> io::Result<()> {
    let name: String = name.to_string_lossy()
                           .chars()
                           .map(|c| {
                               if c.is_control() {
                                   REPLACEMENT_CHAR
                               } else {
                                   c
                               }
                           })
                           .collect();

    if lasts.len() > 0 {
        for last in &lasts[..lasts.len() - 1] {
            let c = if *last {
                ' '
            } else {
                VERTICAL_CHAR
            };
            try!(write!(output, "{}   ", c));
        }
        if *lasts.last().unwrap() {
            try!(write!(output, "{} ", LAST_HORIZONTAL_STR));
        } else {
            try!(write!(output, "{} ", HORIZONTAL_STR,));
        }
    }

    try!(writeln!(output, "{}", name));

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

fn main() {
    let matches = cli::build_cli().get_matches();
    let stdin = io::stdin();
    let trees = if matches.is_present("null") {
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

#[cfg(test)]
mod tests {
    use super::{FileTree, make_trees, print_line};
    use std::ffi::{OsString, OsStr};

    fn test_single_tree_creation(lines: &[&str], expected_tree: FileTree) {
        let trees = make_trees(&mut lines.iter()).unwrap();
        assert_eq!(1, trees.len());
        assert_eq!(expected_tree, trees[0]);
    }

    #[test]
    fn test_tree_creation1() {
        let lines = ["a", "a/b", "a/b/c/d", "a/b/e"];
        let e = FileTree {
            name: OsString::from("e"),
            childs: vec![],
        };
        let d = FileTree {
            name: OsString::from("d"),
            childs: vec![],
        };
        let c = FileTree {
            name: OsString::from("c"),
            childs: vec![d],
        };
        let b = FileTree {
            name: OsString::from("b"),
            childs: vec![c, e],
        };
        let expected_tree = FileTree {
            name: OsString::from("a"),
            childs: vec![b],
        };

        test_single_tree_creation(&lines, expected_tree);
    }

    #[test]
    fn test_tree_creation2() {
        let lines = ["a", "a/b/e", "a/b", "a/b/c/d"];
        let e = FileTree {
            name: OsString::from("e"),
            childs: vec![],
        };
        let d = FileTree {
            name: OsString::from("d"),
            childs: vec![],
        };
        let c = FileTree {
            name: OsString::from("c"),
            childs: vec![d],
        };
        let b = FileTree {
            name: OsString::from("b"),
            childs: vec![e, c],
        };
        let expected_tree = FileTree {
            name: OsString::from("a"),
            childs: vec![b],
        };

        test_single_tree_creation(&lines, expected_tree);
    }

    #[test]
    fn test_trees_creation() {
        let lines = ["a", "a/b", "c/d"];
        let d = FileTree {
            name: OsString::from("d"),
            childs: vec![],
        };
        let c = FileTree {
            name: OsString::from("c"),
            childs: vec![d],
        };
        let b = FileTree {
            name: OsString::from("b"),
            childs: vec![],
        };
        let a = FileTree {
            name: OsString::from("a"),
            childs: vec![b],
        };

        let trees = make_trees(&mut lines.iter()).unwrap();
        assert_eq!(2, trees.len());
        assert_eq!(a, trees[0]);
        assert_eq!(c, trees[1]);
    }

    #[test]
    fn test_print_line() {
        let name = OsStr::new("abc\ndef");

        let mut output1 = vec![];
        print_line(&mut output1, &[], name).unwrap();
        assert_eq!(b"abc?def\n", &*output1);

        let mut output2 = vec![];
        print_line(&mut output2, &[true, false, true], name).unwrap();
        assert_eq!("    │   └── abc?def\n".as_bytes(), &*output2);

        let mut output3 = vec![];
        print_line(&mut output3, &[true, false, false], name).unwrap();
        assert_eq!("    │   ├── abc?def\n".as_bytes(), &*output3);

    }

}
