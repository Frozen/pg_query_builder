use crate::qb::Params;
#[cfg(any(feature = "with-postgres", feature = "with-tokio-postgres"))]
use postgres::types::ToSql;
use std::ops::Deref;

#[cfg(any(feature = "with-postgres", feature = "with-tokio-postgres"))]
pub enum Condition<'a> {
    Eq(String, Box<dyn ToSql + Sync + Send + 'a>),
    Gt(String, Box<dyn ToSql + Sync + Send + 'a>),
    Gte(String, Box<dyn ToSql + Sync + Send + 'a>),
    Lt(String, Box<dyn ToSql + Sync + Send + 'a>),
    Lte(String, Box<dyn ToSql + Sync + Send + 'a>),
    Or(Box<Condition<'a>>, Box<Condition<'a>>),
}

impl<'a> Condition<'a> {
    pub(crate) fn conds(c: Condition<'a>) -> Vec<Box<(dyn ToSql + Sync + 'a)>> {
        match c {
            Condition::Eq(_, v) => vec![v],
            Condition::Gt(_, v) => vec![v],
            Condition::Gte(_, v) => vec![v],
            Condition::Lt(_, v) => vec![v],
            Condition::Lte(_, v) => vec![v],
            Condition::Or(l, r) => {
                let mut v1 = Condition::conds(*l);
                let v2 = Condition::conds(*r);
                v1.extend(v2);
                v1
            }
        }
    }
}

pub struct Filter<'a> {
    pub(crate) conditions: Vec<Condition<'a>>,
}

impl<'a> Filter<'a> {
    pub fn new() -> Filter<'a> {
        Filter { conditions: vec![] }
    }

    pub(crate) fn push(&mut self, c: Condition<'a>) {
        self.conditions.push(c);
    }

    pub(crate) fn collect(self) -> Params<'a> {
        Params::new(
            self.conditions
                .into_iter()
                .map(|e| Condition::conds(e))
                .flatten()
                .collect::<Vec<_>>(),
        )
    }

    pub fn or(c1: Condition<'a>, c2: Condition<'a>) -> Condition<'a> {
        Condition::Or(Box::new(c1), Box::new(c2))
    }
}

// //
// pub struct Filters<'a>(pub Filter<'a>);
//
// impl<'a> Filters<'a> {
//
//     #[cfg(feature = "with-postgres")]
//     pub fn as_slice(&self) -> &[dyn ToSql + Sync + 'a] {
//         // &*self.0
//     }
// }
