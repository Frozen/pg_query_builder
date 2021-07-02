use crate::filter::Condition;
use crate::qb::{Params, QueryBuilder};
use crate::Op;
use std::borrow::Borrow;
use std::ops::Deref;
use tokio_postgres::types::ToSql;

pub struct Update<'a> {
    table_name: String,
    fields: Vec<Condition<'a>>,
    filter: Vec<Condition<'a>>,
}

impl<'a> Update<'a> {
    pub fn new(table_name: &str) -> Update<'a> {
        Update {
            table_name: table_name.to_string(),
            fields: vec![],
            filter: vec![],
        }
    }

    pub fn set<T: ToSql + Sync + Send + 'a>(mut self, op: impl Borrow<Op>, value: T) -> Update<'a> {
        self.fields.push(Condition::Eq(
            op.borrow().name().to_string(),
            Box::new(value),
        ));
        self
    }

    pub fn filter(mut self, e: Condition<'a>) -> Update<'a> {
        self.filter.push(e);
        self
    }

    pub async fn execute(self, c: &mut tokio_postgres::Client) -> Result<u64, postgres::Error> {
        let (q, f) = QueryBuilder::update(&self.table_name, self.fields, self.filter);

        let mut out = vec![];
        let p = f.params();
        for v in &p {
            out.push(v.deref());
        }

        c.execute(&*q, &out[..]).await
    }

    pub(crate) fn build(self) -> (String, Params<'a>) {
        QueryBuilder::update(&self.table_name, self.fields, self.filter)
    }
}
