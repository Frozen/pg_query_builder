use crate::filter::Condition;
#[cfg(any(feature = "with-postgres", feature = "with-tokio-postgres"))]
use postgres::types::ToSql;

pub struct Op {
    field: String,
}

#[cfg(any(feature = "with-postgres", feature = "with-tokio-postgres"))]
impl Op {
    pub fn new<T: Into<String>>(name: T) -> Op {
        Op { field: name.into() }
    }

    pub fn eq<'a, T: ToSql + Sync + 'a>(&self, v: T) -> Condition<'a> {
        Condition::Eq(self.field.clone(), Box::new(v))
    }

    pub fn gt<'a, T: ToSql + Sync + 'a>(&self, v: T) -> Condition<'a> {
        Condition::Gt(self.field.clone(), Box::new(v))
    }

    pub fn gte<'a, T: ToSql + Sync + 'a>(&self, v: T) -> Condition<'a> {
        Condition::Gte(self.field.clone(), Box::new(v))
    }

    pub fn lt<'a, T: ToSql + Sync + 'a>(&self, v: T) -> Condition<'a> {
        Condition::Lt(self.field.clone(), Box::new(v))
    }

    pub fn lte<'a, T: ToSql + Sync + 'a>(&self, v: T) -> Condition<'a> {
        Condition::Lte(self.field.clone(), Box::new(v))
    }
}
