#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CharacterType {
    NewLine,
    Space,
    Tab,
    ParanthesisStart,
    ParanthesisEnd,
    SquareParanthesisStart,
    SquareParanthesisEnd,
    Colon,
    Comma,
    SemiColon,
    EndOfFile,
    Value,
}
