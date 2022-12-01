use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

fn get_input_file() -> PathBuf {
    let exe = env::current_exe().unwrap();
    let mut path = PathBuf::from("inputs");
    path.push(exe.file_name().unwrap());

    if let Some(ext) = env::args().find(|x| x.starts_with("test")) {
        path.set_extension(ext);
    }

    path
}

pub fn read_lines() -> impl Iterator<Item = String> {
    BufReader::new(File::open(get_input_file()).unwrap())
        .lines()
        .filter_map(|x| x.ok())
}

pub fn parse_input<T>() -> impl Iterator<Item = T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    BufReader::new(File::open(get_input_file()).unwrap())
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| x.parse::<T>().unwrap())
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
