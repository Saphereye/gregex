/// The `Operator` enum represents the different operations that can be performed on a regular expression.
#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Or,
    Concat,
    Production,
    Plus,
    Question,
}