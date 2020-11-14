#[macro_use]
extern crate clap;
use clap::App;
use kvs::Result;

fn main() -> Result<()> {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    if matches.is_present("version") {
        println!("kvs version: {}", std::env!("CARGO_PKG_VERSION"))
    }

    match matches.subcommand() {
        ("get", Some(sub_cmd)) => todo!("unimplemented"),
        ("set", Some(sub_cmd)) => todo!("unimplemented"),
        ("rm", Some(sub_cmd)) => todo!("unimplemented"),
        _ => unimplemented!(),
    };

    Ok(())
}
