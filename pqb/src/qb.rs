use crate::filter::Condition;
use crate::order::Direction;

#[derive(Debug, Default)]
struct Index {
    dx: i32,
}

impl Index {
    fn next(&mut self) -> i32 {
        self.dx += 1;
        self.dx
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
        for v in conditions.iter() {
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
}

#[cfg(test)]
mod tests {
    use crate::order::Direction;
    use crate::qb::QueryBuilder;
    use crate::{Filter, Op};

    #[cfg(any(feature = "with-postgres", feature = "with-tokio-postgres"))]
    #[test]
    fn test() {
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
}
