use crate::expr::Expr;

#[derive(Debug, PartialEq)]
pub struct Block {
    pub exprs: Vec<Expr>,
}

impl Block {
  pub fn new(s: &str) -> Result<(&str, Self), String> {
    let s = utils::tag("{", s)?;
    let (s, _) = utils::extract_whitespace(s);
    let s = utils::tag("}", s)?;

    Ok((s, Block { stmts: Vec::new() }))
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_block() {
        assert_eq!(Block::new("{}"), Ok(("", Block { exprs: Vec::new() })));
    }

    #[test]
    fn parse_empty_block_with_whitespace() {
        assert_eq!(Block::new("{   }"), Ok(("", Block { stmts: Vec::new() })));
    }
}