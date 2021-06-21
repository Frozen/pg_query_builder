mod cli;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::process::abort;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
struct Field(String);

impl Field {
    fn rs(&self) -> &str {
        self.0.split(";").next().unwrap_or("")
    }

    pub fn tp(&self) -> &str {
        let mut x = self.0.split(";");
        x.next();
        x.next().unwrap_or("").trim()
    }

    pub fn db(&self) -> &str {
        let mut x = self.0.split(";");
        x.next();
        x.next();
        x.next().unwrap_or("").trim()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
struct Table {
    name: String,
    fields: Vec<Field>,
}

impl Table {
    pub fn rs(&self) -> &str {
        self.name.split(";").next().unwrap_or("").trim()
    }

    pub fn db(&self) -> &str {
        let mut x = self.name.split(";");
        x.next();
        x.next().unwrap_or("").trim()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
struct Tables {
    tables: Vec<Table>,
}

fn main() {
    let matches = cli::build_cli().get_matches();

    match matches.value_of("input") {
        Some(input) => {
            let mut s = File::open(input).unwrap();

            let tables: Tables = serde_yaml::from_reader(&mut s).unwrap();

            println!("use pqb::{{Op, Table, Fields}};\n");

            for v in &tables.tables {
                println!("{}", "#[derive(Debug, Clone)]");
                println!("pub(crate) struct {} {{", &v.rs());
                for f in &v.fields {
                    println!("\tpub(crate) {}: {},", f.rs(), f.tp());
                }

                println!("}}\n");

                println!("impl Table for {} {{", &v.rs());
                println!("\tfn table_name() -> &'static str {{");
                println!("\t\t\"{}\"", &v.db());
                println!("\t}}");
                println!("}}\n");

                println!("impl Fields for {} {{", &v.rs());
                println!("\tfn fields() -> Vec<&'static str> {{");
                println!("\t\tvec![");
                for f in &v.fields {
                    println!("\t\t\t\"{}\",", f.db());
                }
                println!("\t\t]");
                println!("\t}}");
                println!("}}\n");

                println!("impl {} {{", &v.rs());
                for f in &v.fields {
                    println!("\tpub fn {}() -> Op {{", f.rs());
                    println!("\t\tOp::new(\"{}\")", f.db());
                    println!("\t}}")
                }
                println!("}}\n");
            }
        }
        _ => {
            println!("{}", matches.usage());
            abort()
        }
    }
}
