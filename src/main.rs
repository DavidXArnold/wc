// #[macro_use]
extern crate clap;
use clap::load_yaml;
use clap::App;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let _matches = App::from_yaml(yaml).get_matches();

    // Same as previous examples...
}