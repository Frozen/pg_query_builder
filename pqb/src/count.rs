use crate::{Filter, Select};
use postgres::types::ToSql;
use std::ops::Deref;

pub struct Count<'a> {
    select: Select<'a>,
}

impl<'a> Count<'a> {
    pub(crate) fn new(select: Select<'a>) -> Count<'a> {
        Count { select }
    }

    pub async fn query(self, c: &mut tokio_postgres::Client) -> Result<usize, postgres::Error> {
        let (s, yy) = self.select.build();

        let mut out: Vec<&(dyn ToSql + Sync)> = vec![];
        let p = yy; //self.filter.collect().params();
        for v in &p {
            out.push(v.deref());
        }

        let s: &str = &format!("SELECT COUNT(*) FROM ({}) as cnt", s);

        let rs = c.query_one(s, &out[..]).await?;

        let cnt: i64 = rs.get(0);
        Ok(cnt as usize)
    }
}
