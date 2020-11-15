#[macro_use]
extern crate clap;
use clap::App;
use kvs::{KvStore, Result};

fn main() -> Result<()> {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    if matches.is_present("version") {
        println!("kvs version: {}", std::env!("CARGO_PKG_VERSION"))
    }

    match matches.subcommand() {
        ("open", Some(sub_cmd)) => {
            let mut store = KvStore::open("test.db")?;
            eprintln!("{:#?}", store);
        }
        ("get", Some(sub_cmd)) => todo!("unimplemented"),
        ("set", Some(sub_cmd)) => {
            let key = sub_cmd.value_of("key").unwrap();
            let val = sub_cmd.value_of("value").unwrap();
            let mut store = KvStore::open("test.db")?;
            store.set(key.to_owned(), val.to_owned()).unwrap();
        }
        ("rm", Some(sub_cmd)) => todo!("unimplemented"),
        _ => {
            unimplemented!();
        }
    };

    Ok(())
}
