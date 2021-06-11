use crate::filter::{Condition, Filter};
use crate::qb::QueryBuilder;
use crate::table::{Fields, Table};
use postgres;
use postgres::Row;

pub struct Select<'a> {
    filter: Filter<'a>,
    table_name: String,
    fields: Vec<&'static str>,
}

impl<'a> Select<'a> {
    pub fn filter(mut self, e: Condition<'a>) -> Select<'a> {
        self.filter.push(e);
        self
    }

    pub fn from<T: Table + Fields>() -> Select<'a> {
        Select::new(T::table_name(), T::fields())
    }

    pub fn new<T: Into<String>>(table_name: T, fields: Vec<&'static str>) -> Select<'a> {
        Select {
            table_name: table_name.into(),
            filter: Filter::new(),
            fields,
        }
    }

    #[cfg(feature = "with-postgres")]
    pub fn query(self, c: &mut postgres::Client) -> Result<Vec<Row>, postgres::Error> {
        c.query(
            &*QueryBuilder::select(&self.table_name, &self.fields, &self.filter.conditions),
            &*self.filter.collect(),
        )
    }

    #[cfg(feature = "with-tokio-postgres")]
    pub async fn query(self, c: &mut tokio_postgres::Client) -> Result<Vec<Row>, postgres::Error> {
        c.query(
            &*QueryBuilder::select(&self.table_name, &self.fields, &self.filter.conditions),
            &*self.filter.collect(),
        )
        .await
    }
}
