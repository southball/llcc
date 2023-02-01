#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenData {
    Reserved(String),
    Ident(String),
    Num(i32),
    Eof,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub data: TokenData,
    pub position: usize,
}
