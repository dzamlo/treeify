use std::ffi::{OsString, OsStr};
use std::io::{self, BufRead, Write};
use std::path::Path;

const VERTICAL_CHAR: char = '│';
const HORIZONTAL_STR: &'static str = "├──";
const LAST_HORIZONTAL_STR: &'static str = "└──";

struct FileTree {
    name: OsString,
    childs: Vec<FileTree>,
}

fn print_line<W: Write>(output: &mut W, lasts: &[bool], name: &OsStr) -> io::Result<()> {
    if lasts.len() == 0 {
        try!(writeln!(output, "{:?}", name));
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
            try!(writeln!(output, "{} {:?}", LAST_HORIZONTAL_STR, name));
        } else {
            try!(writeln!(output, "{} {:?}", HORIZONTAL_STR, name));
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

fn make_trees<R: BufRead>(input: &mut R) -> io::Result<Vec<FileTree>> {
    let mut pseudo_root = FileTree {
        name: OsString::new(),
        childs: vec![],
    };

    for line in input.lines() {
        let line = try!(line);
        let path = Path::new(&*line);
        let mut bar = path.components().map(|c| c.as_os_str());
        pseudo_root.add(&mut bar);

    }

    Ok(pseudo_root.childs)
}


fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut v = Vec::new();
    for tree in make_trees(&mut stdin.lock()).unwrap() {
        tree.print(&mut stdout, &mut v).unwrap();
    }
}
