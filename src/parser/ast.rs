pub(crate) mod ast {
    use crate::environment::JsValue;
    use crate::Token;

    #[derive(Debug)]
    // 13.3 Binary Operators
    // Reference to Annex A.2 - Expressions
    pub enum Expression {
        Literal(JsValue),
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
