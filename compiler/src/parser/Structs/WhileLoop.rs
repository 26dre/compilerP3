struct WhileLoop {
    condition: Expression,
    body: Block,
}

impl WhileLoop {
    fn new(condition: Expression, body: Block) -> Self {
        Self { condition, body }
    }
}

impl Display for WhileLoop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "while ({}) {{ {} }}", self.condition, self.body)
    }
}

impl WhileLoop {
    fn parse(tokens: &[Token]) -> Result<Self, String> {
        let mut tokens = tokens.iter();
        let condition = Expression::parse(&mut tokens)?;
        let body = Block::parse(&mut tokens)?;
        Ok(Self { condition, body })
    }
}