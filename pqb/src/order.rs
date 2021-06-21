use crate::Op;

#[derive(Debug)]
pub(crate) enum Direction {
    Asc(Op),
    Desc(Op),
}

pub(crate) struct Order {
    orders: Vec<Direction>,
}

impl Order {
    pub(crate) fn new() -> Order {
        Order { orders: vec![] }
    }

    pub fn push_asc(&mut self, op: Op) {
        self.orders.push(Direction::Asc(op))
    }

    pub fn push_desc(&mut self, op: Op) {
        self.orders.push(Direction::Desc(op))
    }

    pub fn into_direction(self) -> Vec<Direction> {
        self.orders
    }
}
