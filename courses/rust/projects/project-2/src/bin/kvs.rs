#[macro_use]
extern crate clap;
use clap::App;
use kvs::{KvStore, Result};

fn app() -> Result<()> {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    if matches.is_present("version") {
        println!("kvs version: {}", std::env!("CARGO_PKG_VERSION"))
    }

    let mut store = KvStore::open("test.db")?;

    match matches.subcommand() {
        ("open", Some(sub_cmd)) => {}
        ("get", Some(sub_cmd)) => {
            let key = sub_cmd.value_of("key").unwrap();

            if let Ok(res) = store.get(key.to_owned()) {
                if let Some(r) = res {
                    println!("{}", r);
                } else {
                    println!("Key not found");
                }
            }
        }
        ("set", Some(sub_cmd)) => {
            let key = sub_cmd.value_of("key").unwrap();
            let val = sub_cmd.value_of("value").unwrap();

            store.set(key.to_owned(), val.to_owned()).unwrap();
        }
        ("rm", Some(sub_cmd)) => {
            let key = sub_cmd.value_of("key").unwrap();
            store.remove(key.to_owned())?;
        }
        _ => {
            unimplemented!();
        }
    };
    Ok(())
}

fn main() {
    std::process::exit(match app() {
        Ok(_) => 0,
        Err(e) => {
            print!("{}", e);
            1
        }
    })
}
