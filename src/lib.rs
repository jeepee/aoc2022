use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::PathBuf;
use std::str::FromStr;

pub struct Input(Lines<BufReader<File>>);

impl Iterator for Input {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                None => return None,
                Some(Ok(x)) => return Some(x),
                Some(Err(_)) => {},
            }
        }
    }
}

pub fn get_input_file(ext: Option<&str>) -> PathBuf {
    let exe = env::current_exe().unwrap();

    // optionally strip the dynamic part of the executable that is added by the test-framework
    let filename = exe.file_name().unwrap().to_string_lossy();
    let filename = match filename.split_once("-") {
        Some((x, _)) => x,
        None => &filename,
    };

    let mut path = PathBuf::from("inputs");
    path.push(filename);

    if let Some(ext) = ext {
        path.set_extension(ext);
    }

    path
}

pub fn read_lines(file: PathBuf) -> Input {
    Input(BufReader::new(File::open(file).unwrap()).lines())
}

pub fn parse_input<T>(input: Input) -> impl Iterator<Item = T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input.map(|x| x.parse::<T>().unwrap())
}

pub fn parse_pair<A,B>(line: &str, sep: &str) -> (A, B)
where A: FromStr,
      B: FromStr,
      <A as FromStr>::Err: Debug,
      <B as FromStr>::Err: Debug,
{
    let (a, b) = line.split_once(sep).unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

pub struct OwnedChars {
    s: String,
    i: usize,
}

impl Iterator for OwnedChars {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.s[self.i..].chars().next()?;

        self.i += c.len_utf8();

        Some(c)
    }
}

impl OwnedChars {
    pub fn from_string(string: String) -> OwnedChars {
        OwnedChars {
            s: string,
            i: 0,
        }
    }
}

pub fn slice_mut_twice<T>(arr: &mut [T], a: usize, b: usize) -> (&mut T, &mut T) {
    // could be written using split_at_mut, but would be more complicated in favor of not using unsafe directly
    assert!(a != b);
    unsafe {(
        &mut *(&mut arr[a] as *mut _),
        &mut *(&mut arr[b] as *mut _)
    )}
}

pub fn run_and_print<F,R>(f: F)
where
    F: FnOnce(Input) -> R,
    R: Debug
{
    println!("{:?}", f(read_lines(get_input_file(None))));
}

pub mod test {
    use crate::{get_input_file,read_lines,Input};

    pub fn test_file<F,R>(ext: Option<&str>, f: F, expected: R) 
    where
        F: FnOnce(Input) -> R,
        R: PartialEq,
        R: std::fmt::Debug
    {
        let result = f(read_lines(get_input_file(ext)));
        assert_eq!(result, expected);
    }

    pub fn test_example<F,R>(f: F, expected: R) 
    where
        F: FnOnce(Input) -> R,
        R: PartialEq,
        R: std::fmt::Debug
    {
        test_file(Some("test"), f, expected)
    }

    pub fn test_puzzle<F,R>(f: F, expected: R) 
    where
        F: FnOnce(Input) -> R,
        R: PartialEq,
        R: std::fmt::Debug
    {
        test_file(None, f, expected)
    }
}