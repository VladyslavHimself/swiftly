// EcmaScript-262 notes
// 13.2.3 Literals
// 12.9.4 String Literals
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),   // For numbers (For JS everything is f64) :- 6.1.6.1 The Number Type
    Boolean(bool), // For booleans :- 6.1.3 The Boolean Type
    StringLiteral(String), // For string literals :- 12.9.4 String Literals
    Identifier(String), // identifiers names for vars and functions :- BindingIdentifier

    Let, // "let" keyword
    If,  // "if" keyword
    Else, // "else" keyword
    While, // "while" keyword

    Assign,    // Operator =
    Semicolon, // semicolon - ;

    LCURLBRACE,
    RCURLBRACE,

    LPARENBRACKET, // (
    RPARENBRACKET, // )

    Not,  // !
    Plus, // +
    Minus, // -
    Star, // *
    Slash, // /
    Percent, // %

    // 13.11 Equality Operators
    EqualityOpEqual,    // ==
    EqualityOpNotEqual, // !=

    RelationOpMore,        // >
    RelationOpLess,        // <
    RelationOpMoreOrEqual, // >=
    RelationOpLessOrEqual, // <=
    RelationOpIn,          // in // not implemented -<

    EOF, // End of file
}
