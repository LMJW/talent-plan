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

    match matches.subcommand() {
        ("open", Some(sub_cmd)) => {
            let store = KvStore::open("test.db")?;
        }
        ("get", Some(sub_cmd)) => {
            let key = sub_cmd.value_of("key").unwrap();
            let store = KvStore::open("test.db")?;
            eprintln!("{:#?}", store);

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

            let mut store = KvStore::open("test.db")?;
            eprintln!("{:#?}", store);

            store.set(key.to_owned(), val.to_owned()).unwrap();
            eprintln!("{:#?}", store);
        }
        ("rm", Some(sub_cmd)) => {
            let key = sub_cmd.value_of("key").unwrap();
            let mut store = KvStore::open("test.db")?;
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
