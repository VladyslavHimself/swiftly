pub(crate) mod ast {
    use crate::Token;

    #[derive(Debug)]
    pub enum Expression {
        Literal(Token),
        Identifier(String),
    }

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
