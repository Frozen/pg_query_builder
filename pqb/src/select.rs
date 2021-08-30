use crate::count::Count;
use crate::filter::{Condition, Filter};
use crate::order::Order;
use crate::qb::QueryBuilder;
use crate::table::{Fields, Table};
use crate::Op;
use postgres;
use postgres::Row;
use std::ops::Deref;
use tokio_postgres::types::ToSql;

pub struct Select<'a> {
    filter: Filter<'a>,
    order: Order,
    table_name: String,
    fields: Vec<&'static str>,
    limit: Option<usize>,
    offset: Option<usize>,
}

impl<'a> Select<'a> {
    pub fn filter(mut self, e: Condition<'a>) -> Select<'a> {
        self.filter.push(e);
        self
    }

    pub fn order_desc(mut self, e: Op) -> Select<'a> {
        self.order.push_desc(e);
        self
    }

    pub fn order_asc(mut self, e: Op) -> Select<'a> {
        self.order.push_asc(e);
        self
    }

    pub fn limit(mut self, limit: usize) -> Select<'a> {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: usize) -> Select<'a> {
        self.offset = Some(offset);
        self
    }

    pub fn from<T: Table + Fields>() -> Select<'a> {
        Select::new(T::table_name(), T::fields())
    }

    pub fn new<T: Into<String>>(table_name: T, fields: Vec<&'static str>) -> Select<'a> {
        Select {
            table_name: table_name.into(),
            filter: Filter::new(),
            order: Order::new(),
            limit: None,
            offset: None,
            fields,
        }
    }

    pub fn count(self) -> Count<'a> {
        Count::new(self)
    }

    pub(crate) fn build(self) -> (String, Vec<Box<(dyn ToSql + Sync + Send + 'a)>>) {
        let st = &*QueryBuilder::select(
            &self.table_name,
            &self.fields,
            &self.filter.conditions,
            &self.order.into_direction(),
            self.limit,
            self.offset,
        );
        let p = self.filter.collect().params();
        (st.to_string(), p)
    }

    #[cfg(feature = "with-postgres")]
    pub fn query(self, c: &mut postgres::Client) -> Result<Vec<Row>, postgres::Error> {
        c.query(
            &*QueryBuilder::select(
                &self.table_name,
                &self.fields,
                &self.filter.conditions,
                &self.order.into_direction(),
                self.limit,
                self.offset,
            ),
            &*self.filter.collect(),
        )
    }

    #[cfg(feature = "with-tokio-postgres")]
    pub async fn query(self, c: &mut tokio_postgres::Client) -> Result<Vec<Row>, postgres::Error> {
        let st = &*QueryBuilder::select(
            &self.table_name,
            &self.fields,
            &self.filter.conditions,
            &self.order.into_direction(),
            self.limit,
            self.offset,
        );

        let mut out: Vec<&(dyn ToSql + Sync)> = vec![];
        let p = self.filter.collect().params();
        for v in &p {
            out.push(v.deref());
        }

        c.query(st, &out[..]).await
    }
}
