pub trait Table {
    fn table_name() -> &'static str;
}

pub trait Fields {
    fn fields() -> Vec<&'static str>;
}
