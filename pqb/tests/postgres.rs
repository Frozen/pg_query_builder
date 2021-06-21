#![cfg(feature = "with-postgres")]

use postgres::{Client, Row};
use pqb::Op;
use pqb::Select;
use pqb::Table;
use pqb::{Fields, Filter};
use std::ops::Deref;

#[derive(Default, Debug, Clone)]
struct Person {
    id: i64,
    name: String,
}

#[derive(Debug, Clone)]
struct Rows<T>(T);

impl<T> Deref for Rows<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Person {
    fn insert(&self, client: &mut Client) -> Result<u64, postgres::Error> {
        client.execute(
            "INSERT INTO person (\"id\", \"user\") values($1, $2)",
            &[&self.id, &self.name],
        )
    }
}

impl From<postgres::Row> for Person {
    fn from(r: postgres::Row) -> Self {
        Person {
            id: r.get("id"),
            name: r.get("name"),
        }
    }
}

impl From<&postgres::Row> for Person {
    fn from(r: &postgres::Row) -> Self {
        Person {
            id: r.get("id"),
            name: r.get("name"),
        }
    }
}

impl From<Vec<postgres::Row>> for Rows<Vec<Person>> {
    fn from(r: Vec<postgres::Row>) -> Self {
        let x: Vec<Person> = r.iter().map(|r| r.into()).collect();
        Rows(x)
    }
}

impl Table for Person {
    fn table_name() -> &'static str {
        "person"
    }
}

impl Fields for Person {
    fn fields() -> Vec<&'static str> {
        vec!["id", "name"]
    }
}

impl Person {
    fn id() -> Op {
        Op::new("id")
    }
    fn name() -> Op {
        Op::new("name")
    }
}

#[test]
fn test_add() {
    use postgres::{Client, NoTls};
    let mut client =
        Client::connect("host=127.0.0.1 port=5436 user=pg password=pg", NoTls).unwrap();

    client
        .batch_execute(
            "
        CREATE TABLE IF NOT EXISTS person (
            id      INT NOT NULL,
            name    TEXT NOT NULL
        );
    ",
        )
        .unwrap();

    // client
    //     .query("SELECT * FROM person where id = $1", &[&1i64])
    //     .unwrap();

    Select::from::<Person>()
        .filter(Person::id().eq(5i64))
        .query(&mut client)
        // .map(|e| )
        .unwrap();

    // Person {
    //     id: 1,
    //     name: "bla".to_string(),
    // }
    // .insert(&mut client)
    // .unwrap();

    // Insert::into::<Article>.value(Article::id().eq(5));

    //
    let name: i64 = 5;
    let data = "bla";
    client
        .execute(
            "INSERT INTO person (id, name) VALUES ($1, $2)",
            &[&name, &data],
        )
        .unwrap();

    Select::from::<Person>()
        .filter(Person::id().eq(5i64))
        .filter(Person::name().eq("bla"))
        .query(&mut client)
        // .map(|e| )
        .unwrap();

    let rs: Rows<Vec<Person>> = Select::from::<Person>()
        .filter(Filter::or(Person::name().eq("bla"), Person::id().eq(5i64)))
        .filter(Person::name().eq("bla"))
        .query(&mut client)
        .map(|e| e.into())
        .unwrap();

    let rs2: Vec<Person> = rs
        .iter()
        .map(|e| Person {
            id: e.id * 2,
            name: e.name.clone(),
        })
        .collect();

    // let rs: Vec<Person> = Select::from::<Person>()
    //     .filter(Filter::or(Person::name().eq("bla"), Person::id().eq(5i64)))
    //     .filter(Person::name().eq("bla"))
    //     .query(&mut client)
    //     .flat_map(|e| )
    //     .unwrap();

    println!("{:?}", rs2)
}
