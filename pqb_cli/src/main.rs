mod cli;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::process::abort;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
struct Field(String);

impl Field {
    // rust representations
    fn rs(&self) -> &str {
        self.0.split(";").next().unwrap_or("")
    }

    // type
    pub fn tp(&self) -> &str {
        let mut x = self.0.split(";");
        x.next();
        x.next().unwrap_or("").trim()
    }

    // storage name
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

            println!("#![allow(dead_code)]");
            println!("use pqb::{{Op, Table, Fields, Update}};\n");
            println!("use std::borrow::Borrow;\n");
            println!("use super::Client;\n");
            println!("use super::DbError;\n");
            println!("use serde::Serialize;\n");

            for v in &tables.tables {
                println!("{}", "#[derive(Debug, Clone, Serialize)]");
                println!("pub struct {} {{", &v.rs());
                for f in &v.fields {
                    println!("\tpub {}: {},", f.rs(), f.tp());
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

                println!(
                    "impl<T: Borrow<tokio_postgres::Row>> From<T> for {} {{",
                    &v.rs()
                );
                println!("\tfn from(r: T) -> Self {{");
                println!("\t\t{} {{", &v.rs());
                for f in &v.fields {
                    println!("\t\t\t{}: r.borrow().get(\"{}\"),", f.rs(), f.db());
                }
                println!("\t\t}}");
                println!("\t}}");
                println!("}}\n");

                let mut s = String::new();
                insert(&mut s, &v);
                update(&mut s, &v);
                print!("{}", s);
            }
        }
        _ => {
            println!("{}", matches.usage());
            abort()
        }
    }
}

fn insert(s: &mut String, t: &Table) {
    s.push_str(&format!("impl {} {{\n", t.rs()));
    s.push_str(&format!("\tpub async fn insert("));
    let mut fields: Vec<String> = vec!["client: &mut Client".to_string()];
    for f in &t.fields {
        fields.push(format!(
            "{}: &{}",
            f.rs(),
            if f.tp() == "String" {
                "str".to_string()
            } else {
                f.tp().to_string()
            }
        ));
    }
    s.push_str(&format!(
        "{}) -> Result<u64, DbError> {{\n",
        &fields.join(", ")
    ));

    let mut ss1 = vec![];
    for v in 1..=t.fields.len() {
        ss1.push(format!("${}", v));
    }
    let mut ss2 = vec![];
    for v in &t.fields {
        ss2.push(format!("&{}", v.rs()));
    }
    s.push_str("\t\t");
    s.push_str(&format!(
        r#"client.execute("INSERT INTO {} VALUES({})", &[{}]).await"#,
        t.db(),
        ss1.join(", "),
        ss2.join(", ")
    ));
    s.push_str("\n\t}");
    s.push_str("\n");

    s.push_str("\tpub async fn create(&self, mut client: &mut Client) -> Result<u64, DbError> {\n");

    let mut ss3 = vec![];
    for v in &t.fields {
        ss3.push(format!("&self.{}", v.rs()));
    }
    s.push_str(&format!(
        "\t\t{}::insert(&mut client, {}).await",
        t.rs(),
        ss3.join(", ")
    ));
    drop(ss3);

    s.push_str("\n");
    s.push_str("\t}\n");

    s.push_str(&format!(
        "\tpub fn into_update(self) -> Update{} {{\n",
        &t.rs()
    ));
    s.push_str(&format!("\t\tUpdate{} {{\n", &t.rs()));
    for v in &t.fields {
        s.push_str(&format!("\t\t\t{}: Some(self.{}),\n", v.rs(), v.rs()));
    }
    s.push_str(&format!("\t\t}}\n"));
    s.push_str("\t}\n");

    s.push_str("}\n\n");
}

fn update(s: &mut String, t: &Table) {
    s.push_str("#[derive(Debug, Default)]\n");
    s.push_str(&format!("pub struct Update{} {{\n", t.rs()));
    for f in &t.fields {
        s.push_str(&format!("\tpub {}: Option<{}>,\n", f.rs(), f.tp()));
    }
    s.push_str("}\n");
    s.push_str("\n");

    s.push_str(&format!("impl {} {{\n", t.rs()));
    s.push_str("\tpub fn update<'a>() -> Update<'a> {");
    s.push_str(&format!("\t\tUpdate::new(\"{}\")", t.db()));
    s.push_str("\t}\n");
    s.push_str("}\n");
    s.push_str("\n");

    s.push_str(&format!("impl Update{} {{\n", t.rs()));
    s.push_str(&format!("\tpub fn into_filter<'a>(self) -> Update<'a>{{\n"));
    s.push_str(&format!("\t\tlet mut q = {}::update();\n", t.rs()));
    for f in &t.fields {
        s.push_str(&format!("\t\tif let Some(v) = self.{} {{\n", f.rs()));
        s.push_str(&format!("\t\t\t q = q.set({}::{}(), v);\n", t.rs(), f.rs()));
        s.push_str("\t\t}\n");
    }
    s.push_str("\t\tq\n");
    s.push_str("\t}\n");
    s.push_str("}\n");

    return;
}

#[cfg(test)]
mod tests {
    use crate::{insert, Field, Table};

    #[test]
    fn test_insert() {
        let t = Table {
            name: "User".to_string(),
            fields: vec![Field {
                0: "id; String; id".to_string(),
            }],
        };
        let mut s = String::new();
        insert(&mut s, &t);
    }
}
