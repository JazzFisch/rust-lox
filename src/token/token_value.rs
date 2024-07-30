#[derive(Debug, Default, Clone, PartialEq)]
pub enum TokenValue {
    #[default]
    None,
    Number(f64),
    String(String),
    Identifier(String),
}
