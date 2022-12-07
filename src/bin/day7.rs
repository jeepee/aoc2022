use std::{collections::HashMap, str, fmt::Display};
use aoc2022::{run_and_print, Input, parse_pair};

#[derive(Default)]
struct Directory {
    dirs: HashMap<String,Directory>,
    files: HashMap<String,File>
}

struct File(usize);

struct Builder {
    current: (String,Directory),
    parents: Vec<(String,Directory)>,
}

enum Cmd {
    Root,
    Parent,
    Child(String),
    ListDir(String),
    ListFile(String, usize),
}

impl Cmd {
    fn parse(s: String) -> Option<Self> {
        if let Some(s) = s.strip_prefix("$ ") {
            if s == "cd .." {
                Some(Cmd::Parent)
            } else if s == "cd /" {
                Some(Cmd::Root)
            } else if let Some(s) = s.strip_prefix("cd ") {
                Some(Cmd::Child(s.to_owned()))
            } else {
                None
            }
        } else if let Some(s) = s.strip_prefix("dir ") {
            Some(Cmd::ListDir(s.to_owned()))
        } else {
            let (size, name) = parse_pair(&s, " ");
            Some(Cmd::ListFile(name, size))
        }
    }
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            current: ("/".to_owned(), Directory::default()),
            parents: Vec::new(),
        }
    }

    fn run_commands(mut self, cmds: impl Iterator<Item=Cmd>) -> Self {
        for cmd in cmds {
            match cmd {
                Cmd::Root => {
                    self.cd_root();
                },
                Cmd::Parent => {
                    self.cd_parent();
                },
                Cmd::Child(name) => {
                    if let Some(child) = self.current.1.dirs.remove(&name) {
                        self.parents.push(std::mem::replace(&mut self.current, (name, child)))
                    }
                }
                Cmd::ListDir(name) => {
                    self.current.1.dirs.insert(name, Directory::default());
                }
                Cmd::ListFile(name, size) => {
                    self.current.1.files.insert(name, File(size));
                },
            }
        }

        self
    }

    fn cd_root(&mut self) {
        while !self.parents.is_empty() {
            self.cd_parent();
        }
    }

    fn cd_parent(&mut self) {
        if let Some((name,parent)) = self.parents.pop() {
            let (name,child) = std::mem::replace(&mut self.current, (name, parent));
            self.current.1.dirs.insert(name, child);
        }
    }

    fn into_root(mut self) -> Directory {
        self.cd_root();
        self.current.1
    }
}

fn reconstruct_tree_from_input(input: Input) -> Directory {
    Builder::new()
        .run_commands(input.filter_map(Cmd::parse))
        .into_root()
}

impl Directory {
    pub fn walk<W: DirWalker>(&self, name: &str, walker: &mut W) {
        walker.enter(name);
        self.dirs.iter().for_each(|(name,dir)| dir.walk(name, walker));
        self.files.iter().for_each(|(name,File(size))| walker.file(name, *size));
        walker.exit(name);
    }
}

trait DirWalker {
    fn enter(&mut self, name: &str);
    fn exit(&mut self, name: &str);
    fn file(&mut self, name: &str, size: usize);
}

struct DisplayWalker<'a,'b> {
    level: usize,
    f: &'a mut std::fmt::Formatter<'b>,
}

impl<'a,'b> DirWalker for DisplayWalker<'a,'b> {
    fn enter(&mut self, name: &str) {
        writeln!(self.f, "{}- {} (dir)", "  ".repeat(self.level), name).unwrap();
        self.level += 1;
    }

    fn exit(&mut self, _name: &str) {
        self.level -= 1;
    }

    fn file(&mut self, name: &str, size: usize) {
        writeln!(self.f, "{}- {} (file, size={})", "  ".repeat(self.level), name, size).unwrap();
    }
}

impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut walker = DisplayWalker { level: 0, f };
        self.walk("/", &mut walker);
        Ok(())
    }
}

#[derive(Default)]
struct CollectAllSizes {
    sizes: Vec<usize>,
    parents: Vec<usize>,
    current: usize,
}

impl DirWalker for CollectAllSizes {
    fn enter(&mut self, _name: &str) {
        self.parents.push(self.current);
        self.current = 0;
    }

    fn exit(&mut self, _name: &str) {
        self.sizes.push(self.current);
        self.current += self.parents.pop().unwrap();
    }

    fn file(&mut self, _name: &str, size: usize) {
        self.current += size;
    }
}

fn run(input: Input) -> (usize, usize) {
    let root = reconstruct_tree_from_input(input);

    let mut collector = CollectAllSizes::default();
    root.walk("/", &mut collector);
    collector.sizes.sort();

    let part1 = collector.sizes
        .iter()
        .take_while(|v| **v <= 100000)
        .sum();

    let free = 70000000 - collector.current;
    let required = 30000000;
    let part2 = *collector.sizes
        .iter()
        .find(|size| free + **size >= required)
        .unwrap();

    (part1, part2)
}

fn main() {
    run_and_print(run);
}

#[cfg(test)]
mod test {
    use aoc2022::test::{test_example, test_puzzle};

    #[test]
    fn example() {
        test_example(crate::run, (95437, 24933642))
    }

    #[test]
    fn puzzle() {
        test_puzzle(crate::run, (1723892, 8474158))
    }
}