extern crate clap;
use clap::{load_yaml, App};
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Hash, Eq, PartialEq, Debug)]
struct ObjectAttributes {
    bytes: Option<u64>,
    characters: Option<u64>,
    lines: Option<u64>,
    words: Option<u64>,
    pos: usize,
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
    let characters = arg_matches.is_present("chars");

    if !bytes && !lines && !words && !characters {
        bytes = true;
        lines = true;
        words = true;
    }
    return Conf {
        bytes: bytes,
        characters: characters,
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

    if filenames.len() > 1 {
        let sum_totals: u64 = totals.iter().sum();

        if sum_totals > 0 {
            totals
                .as_ref()
                .iter()
                .filter(|x| **x != 0)
                .for_each(|x| print!("\t{} ", x));
            println!("total")
        }
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
        return Some(count);
    } else {
        return None;
    }
}

fn characters(conf: &Conf, contents: &std::string::String) -> Option<u64> {
    if conf.characters {
        return Some(contents.chars().count() as u64);
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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use std::env;
    use std::fs::File;
    extern crate scopeguard;

    #[test]
    fn test_bytes() {
        let string = String::from("test all the things");
        let config = Conf {
            bytes: true,
            characters: true,
            lines: true,
            files_from: false,
            max_line_length: 0,
            words: true,
        };
        let option_value: Option<u64> = Some(19);
        assert_eq!(bytes(&config, &string), option_value)
    }
    #[test]
    fn test_lines() {
        let string = String::from("test all the things\nand other things");
        let config = Conf {
            bytes: false,
            characters: false,
            lines: true,
            files_from: false,
            max_line_length: 0,
            words: false,
        };
        let option_value: Option<u64> = Some(2);
        assert_eq!(lines(&config, &string), option_value)
    }
    #[test]
    fn test_characters() {
        let string = String::from("test all the things\nand other things");
        let config = Conf {
            bytes: false,
            characters: true,
            lines: false,
            files_from: false,
            max_line_length: 0,
            words: false,
        };
        let option_value: Option<u64> = Some(36);
        assert_eq!(characters(&config, &string), option_value)
    }
    #[test]
    fn test_words() {
        let string = String::from("test all the things\nand other things");
        let config = Conf {
            bytes: false,
            characters: false,
            lines: false,
            files_from: false,
            max_line_length: 0,
            words: true,
        };
        let option_value: Option<u64> = Some(7);
        assert_eq!(words(&config, &string), option_value)
    }
    #[test]
    fn test_read_files() {
        use scopeguard::defer;
        use std::fs;
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        let mut dir = env::temp_dir();
        dir.push(rand_string);
        let f = File::create(&dir);
        defer! {
            fs::remove_file(&dir).unwrap();
        }
        f.unwrap().write_all(b"sup,\n wc?").unwrap();

        let config = Conf {
            bytes: false,
            characters: false,
            lines: false,
            files_from: false,
            max_line_length: 0,
            words: true,
        };

        let filenames: Vec<&str> = vec![&dir.to_str().unwrap()];
        read_files(&config, filenames).unwrap();
    }
}
