#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}
