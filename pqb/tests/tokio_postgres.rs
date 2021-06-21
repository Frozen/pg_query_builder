#![cfg(feature = "with-tokio-postgres")]

use pqb::{Fields, Filter, Op, Select, Table};
use std::ops::Deref;
use tokio_postgres::{Client, Error, NoTls};

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
    async fn insert(&self, client: &mut Client) -> Result<u64, postgres::Error> {
        client
            .execute(
                "INSERT INTO person (\"id\", \"user\") values($1, $2)",
                &[&self.id, &self.name],
            )
            .await
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

#[tokio::test]
async fn test() {
    let (mut client, connection) =
        tokio_postgres::connect("host=127.0.0.1 port=5436 user=pg password=pg", NoTls)
            .await
            .unwrap();

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rs: Rows<Vec<Person>> = Select::from::<Person>()
        .filter(Filter::or(Person::name().eq("bla"), Person::id().eq(5i64)))
        .filter(Person::name().eq("bla"))
        .query(&mut client)
        .await
        .map(|e| e.into())
        .unwrap();

    println!("{:?}", rs)
    // And then check that we got back the same string we sent over.
    // let value: &str = rows[0].get(0);
    // assert_eq!(value, "hello world");
}
