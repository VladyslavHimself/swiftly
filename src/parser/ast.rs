pub(crate) mod ast {
    use crate::Token;

    #[derive(Debug)]
    // Reference to Annex A.2 - Expressions
    pub enum Expression {
        Literal(Token),
        Identifier(String),
    }

    // Reference to Annex A.3 - Statements
    #[derive(Debug)]
    pub enum Statement {
        // let <id> = <expr>;
        VariableDeclaration { id: String, init: Expression },
    }

    // Root of the program
    pub struct Program {
        pub body: Vec<Statement>,
    }
}
