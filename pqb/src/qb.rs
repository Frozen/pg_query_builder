use crate::filter::Condition;
use crate::order::Direction;
use std::ops::Deref;
use tokio_postgres::types::ToSql;

#[derive(Debug, Default)]
struct Index {
    dx: usize,
}

impl Index {
    fn next(&mut self) -> usize {
        self.dx += 1;
        self.dx
    }
    pub fn new(idx: usize) -> Index {
        Index { dx: idx }
    }
}

pub(crate) struct QueryBuilder {}

impl QueryBuilder {
    fn push(out: &mut Vec<String>, i: &mut Index, v: &Condition) {
        match v {
            Condition::Eq(name, _) => out.push(format!("{} = ${}", name, i.next())),
            Condition::Gt(name, _) => out.push(format!("{} > ${}", name, i.next())),
            Condition::Gte(name, _) => out.push(format!("{} >= ${}", name, i.next())),
            Condition::Lt(name, _) => out.push(format!("{} < ${}", name, i.next())),
            Condition::Lte(name, _) => out.push(format!("{} <= ${}", name, i.next())),
            Condition::Or(l, r) => {
                let mut collect = vec![];
                QueryBuilder::push(&mut collect, i, l);
                QueryBuilder::push(&mut collect, i, r);
                out.push(format!("( {} )", collect.join(" OR ")));
            }
        }
    }

    pub fn select(
        name: &str,
        fields: &[&'static str],
        conditions: &[Condition],
        orders: &[Direction],
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> String {
        let mut s: Vec<String> = vec![];
        let mut ind = Index::default();
        for v in conditions {
            QueryBuilder::push(&mut s, &mut ind, v);
        }

        let mut q = if fields.len() > 0 {
            format!("SELECT {} FROM \"{}\"", fields.join(", "), name)
        } else {
            format!("SELECT {} FROM \"{}\"", "*", name)
        };

        if s.len() > 0 {
            q.push_str(" WHERE ");
            q.push_str(&s.join(" AND "));
        }

        if orders.len() > 0 {
            q.push_str(" ORDER BY ");
            let s = orders
                .iter()
                .map(|v| match v {
                    Direction::Asc(op) => {
                        let mut s = op.name().to_string();
                        s.push_str(" asc");
                        s
                    }
                    Direction::Desc(op) => {
                        let mut s = op.name().to_string();
                        s.push_str(" desc");
                        s
                    }
                })
                .collect::<Vec<String>>()
                .join(", ");
            q.push_str(&s);
        }

        if let Some(v) = limit {
            q.push_str(" LIMIT ");
            q.push_str(&v.to_string());
        }

        if let Some(v) = offset {
            q.push_str(" OFFSET ");
            q.push_str(&v.to_string());
        }

        return q;
    }

    pub fn update<'a>(
        name: &str,
        fields: Vec<Condition<'a>>,
        conditions: Vec<Condition<'a>>,
    ) -> (String, Params<'a>) {
        let mut s = format!("UPDATE {} SET ", name);
        let mut wheres: Vec<Box<(dyn ToSql + Sync)>> = vec![];
        let mut names = vec![];
        for v in fields {
            match v {
                Condition::Eq(name, value) => {
                    names.push(format!("{} = ${}", name, names.len() + 1));
                    wheres.push(value);
                }
                _ => unreachable!(),
            }
        }

        s.push_str(&format!("{}", names.join(", ")));

        if wheres.len() + conditions.len() == 0 {
            return (s, Params::empty());
        }

        let mut q = vec![];
        let mut ind = Index::new(wheres.len());
        for v in conditions {
            QueryBuilder::push(&mut q, &mut ind, &v);
            for e in Condition::conds(v) {
                wheres.push(e);
            }
        }

        if q.len() > 0 {
            s.push_str(" WHERE ");
            s.push_str(&q.join(" AND "));
        }
        return (s, Params(wheres));
    }
}

pub(crate) struct Params<'a>(Vec<Box<(dyn ToSql + Sync + 'a)>>);

impl<'a> Params<'a> {
    pub(crate) fn params(self) -> Vec<Box<(dyn ToSql + Sync + 'a)>> {
        self.0
    }

    fn empty() -> Params<'a> {
        Params(vec![])
    }

    pub(crate) fn new(v: Vec<Box<(dyn ToSql + Sync + 'a)>>) -> Params {
        Params(v)
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::order::Direction;
    use crate::qb::{Params, QueryBuilder};
    use crate::update::Update;
    use crate::{Filter, Op};

    #[cfg(any(feature = "with-postgres", feature = "with-tokio-postgres"))]
    #[test]
    fn test_select() {
        assert_eq!(
            r#"SELECT id, name FROM "users" ORDER BY id asc"#,
            QueryBuilder::select(
                "users",
                &["id", "name"],
                &[],
                &[Direction::Asc(Op::new("id"))],
                None,
                None,
            )
        );
        assert_eq!(
            r#"SELECT * FROM "users""#,
            QueryBuilder::select("users", &[], &[], &[], None, None)
        );
        assert_eq!(
            r#"SELECT id, name FROM "users""#,
            QueryBuilder::select("users", &["id", "name"], &[], &[], None, None)
        );
        assert_eq!(
            r#"SELECT * FROM "users" WHERE id = $1"#,
            QueryBuilder::select("users", &[], &[Op::new("id").eq(5)], &[], None, None)
        );
        assert_eq!(
            r#"SELECT * FROM "users" WHERE ( id = $1 OR name = $2 )"#,
            QueryBuilder::select(
                "users",
                &[],
                &[Filter::or(
                    Op::new("id").eq(5),
                    Op::new("name").eq("anything")
                )],
                &[],
                None,
                None
            )
        );
        assert_eq!(
            r#"SELECT id FROM "users" ORDER BY id desc LIMIT 10 OFFSET 20"#,
            QueryBuilder::select(
                "users",
                &["id"],
                &[],
                &[Direction::Desc(Op::new("id"))],
                10.into(),
                20.into(),
            )
        );
    }

    #[cfg(any(feature = "with-postgres", feature = "with-tokio-postgres"))]
    #[test]
    fn test_update() {
        let u = Update::new("users")
            .set(Op::new("id"), 5)
            .filter(Op::new("id").eq(10))
            .build();

        assert_eq!("UPDATE users SET id = $1 WHERE id = $2", u.0);
        assert_eq!(2, u.1.len());
    }
}
