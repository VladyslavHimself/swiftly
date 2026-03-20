// EcmaScript-262 notes
// 13.2.3 Literals
// 12.9.4 String Literals
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64), // For numbers (For JS everything is f64) :- 6.1.6.1 The Number Type
    StringLiteral(String), // For string literals :- 12.9.4 String Literals
    Identifier(String), // identifiers names for vars and functions :- BindingIdentifier
    Let,         // "let" keyword
    Assign,      // Operator =
    Semicolon,   // semicolon - ;

    LCURLBRACE,
    RCURLBRACE,

    Plus,     // +
    Equal,    // ==
    Not,      // !
    NotEqual, // !=
    EOF,      // End of file
}
