pub(crate) mod ast {
    use crate::environment::JsValue;

    #[derive(Debug)]
    // 13.3 Binary Operators
    // !Reference to Annex A.2 - Expressions
    pub enum Expression {
        Assignment {
            name: String,
            value: Box<Expression>,
        },
        Literal(JsValue),
        Identifier(String),
        Binary {
            left: Box<Expression>,
            operator: String,
            right: Box<Expression>,
        },
    }

    // Reference to Annex A.3 - Statements
    #[derive(Debug)]
    pub enum Statement {
        // let <id> = <expr>;
        VariableDeclaration { id: String, init: Expression },
        Block(Vec<Statement>),
        Expression(Expression),
    }

    // Root of the program
    pub struct Program {
        pub body: Vec<Statement>,
    }
}
