extern crate clap;
use clap::{load_yaml, App};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Hash, Eq, PartialEq, Debug)]
struct ObjectAttributes {
    bytes: Option<u64>,
    characters: Option<u64>,
    lines: Option<u64>,
    words: Option<u64>,
    pos: usize,
}

impl Iterator for ObjectAttributes {
    type Item = Option<u64>;
    fn next(&mut self) -> Option<Option<u64>> {
        if self.pos > 4 {
            None
        } else {
            self.pos += 1;
            match self.pos {
                0 => Some(self.bytes),
                1 => Some(self.characters),
                2 => Some(self.lines),
                3 => Some(self.words),
                _ => Option::None,
            }
        }
    }
}

// Config is a struct representing configured runtime flags / options
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Conf {
    bytes: bool,
    characters: bool,
    lines: bool,
    files_from: bool,
    max_line_length: u32,
    words: bool,
}

fn config(arg_matches: &clap::ArgMatches) -> Conf {
    let mut bytes = arg_matches.is_present("bytes");
    let mut lines = arg_matches.is_present("lines");
    let mut words = arg_matches.is_present("words");

    if !bytes && !lines && !words {
        bytes = true;
        lines = true;
        words = true;
    }
    return Conf {
        bytes: bytes,
        characters: arg_matches.is_present("chars"),
        lines: lines,
        files_from: arg_matches.is_present("files_from"),
        // @TODO implement max_line_length
        max_line_length: 0,
        words: words,
    };
}

fn main() -> std::io::Result<()> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let conf = config(&matches);
    let filenames: Vec<&str> = matches.values_of("files").unwrap_or_default().collect();
    if filenames.len() > 0 {
        read_files(&conf, filenames).unwrap_or_default();
    } else {
        read_stream().unwrap_or_default();
    }
    Ok(())
}

fn read_files(conf: &Conf, filenames: Vec<&str>) -> std::io::Result<()> {
    let mut objects: HashMap<&str, ObjectAttributes> = HashMap::new();
    for filename in filenames.iter() {
        let file = File::open(filename);
        match file {
            Ok(f) => {
                let mut contents = String::new();
                let mut buf_reader = BufReader::new(f);
                let _l = buf_reader.read_to_string(&mut contents);
                let attrib = ObjectAttributes {
                    bytes: bytes(&conf, &contents),
                    lines: lines(&conf, &contents),
                    characters: characters(&conf, &contents),
                    words: words(&conf, &contents),
                    pos: 0,
                };
                objects.insert(filename, attrib);
            }
            Err(e) => {
                println!("wc: {} {}", filename, e);
            }
        }
    }

    let mut totals: [u64; 4] = [0, 0, 0, 0];

    for (k, a) in objects.iter() {
        if a.bytes.is_some() {
            print!("\t{} ", a.bytes.unwrap());
            totals[0] += a.bytes.unwrap()
        }
        if a.characters.is_some() {
            print!("\t{} ", a.characters.unwrap());
            totals[1] += a.characters.unwrap()
        }
        if a.lines.is_some() {
            print!("\t{} ", a.lines.unwrap());
            totals[2] += a.lines.unwrap()
        }
        if a.words.is_some() {
            print!("\t{} ", a.words.unwrap());
            totals[3] += a.words.unwrap()
        }
        print!("{}\n", k);
    }

    let sum_totals: u64 = totals.iter().sum();

    if sum_totals > 0 {
        totals
            .as_ref()
            .iter()
            .filter(|x| **x != 0)
            .for_each(|x| print!("\t{} ", x));
        println!("total")
    }
    Ok(())
}

fn bytes(conf: &Conf, contents: &std::string::String) -> Option<u64> {
    if conf.bytes {
        return Some(contents.len() as u64);
    } else {
        return None;
    }
}

fn lines(conf: &Conf, contents: &std::string::String) -> Option<u64> {
    if conf.lines {
        let mut count = 0u64;
        for _line in contents.lines() {
            count += 1;
        }
        return Some(count as u64);
        // total += contents.len() as u32;
    } else {
        return None;
    }
}

fn characters(conf: &Conf, contents: &std::string::String) -> Option<u64> {
    if conf.characters {
        let count = contents.unicode_words().collect::<Vec<&str>>();

        return Some(count.len() as u64);
    } else {
        return None;
    }
}

fn words(conf: &Conf, contents: &std::string::String) -> Option<u64> {
    if conf.words {
        let count = contents.unicode_words().collect::<Vec<&str>>();

        return Some(count.len() as u64);
    } else {
        return None;
    }
}

fn read_stream() -> std::io::Result<()> {
    println! {"not implemented"}
    Ok(())
}
