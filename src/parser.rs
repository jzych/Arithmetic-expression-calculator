use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(i64),
    BinaryOp {
        op: Token,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEndOfInput,
    InvalidExpression,
}

pub fn parse(tokens: &[Token]) -> Result<Expr, ParseError> {
    let mut pos = 0;

    fn parse_expression(tokens: &[Token], pos: &mut usize) -> Result<Expr, ParseError> {
        let mut node = parse_term(tokens, pos)?;

        while let Some(op) = peek(tokens, *pos) {
            match op {
                Token::Plus | Token::Minus => {
                    *pos += 1;
                    let right = parse_term(tokens, pos)?;
                    node = Expr::BinaryOp {
                        op: op.clone(),
                        left: Box::new(node),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_term(tokens: &[Token], pos: &mut usize) -> Result<Expr, ParseError> {
        let mut node = parse_factor(tokens, pos)?;

        while let Some(op) = peek(tokens, *pos) {
            match op {
                Token::Asterisk | Token::Slash => {
                    *pos += 1;
                    let right = parse_factor(tokens, pos)?;
                    node = Expr::BinaryOp {
                        op: op.clone(),
                        left: Box::new(node),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_factor(tokens: &[Token], pos: &mut usize) -> Result<Expr, ParseError> {
        match peek(tokens, *pos) {
            Some(Token::Number(n)) => {
                *pos += 1;
                Ok(Expr::Number(*n))
            }
            Some(Token::LeftParenthesis) => {
                *pos += 1;
                let expr = parse_expression(tokens, pos)?;
                match peek(tokens, *pos) {
                    Some(Token::RightParenthesis) => {
                        *pos += 1;
                        Ok(expr)
                    }
                    _ => Err(ParseError::UnexpectedToken(tokens[*pos].clone())),
                }
            }
            Some(tok) => Err(ParseError::UnexpectedToken(tok.clone())),
            None => Err(ParseError::UnexpectedEndOfInput),
        }
    }

    fn peek(tokens: &[Token], pos: usize) -> Option<&Token> {
        tokens.get(pos)
    }

    let ast = parse_expression(tokens, &mut pos)?;
    if pos != tokens.len() {
        Err(ParseError::InvalidExpression)
    } else {
        Ok(ast)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Token;

    #[test]
    fn test_empty_input() {
        let tokens: Vec<Token> = Vec::new();
        let result = parse(&tokens);

        assert!(result.is_err());
        assert_eq!(result, Err(ParseError::UnexpectedEndOfInput));
    }

    #[test]
    fn test_simple_expression() {
        let input = vec![Token::Number(2), Token::Plus, Token::Number(3)]; // 2 + 3
        let result = parse(&input).unwrap();

        assert_eq!(
            result,
            Expr::BinaryOp {
                op: Token::Plus,
                left: Box::new(Expr::Number(2)),
                right: Box::new(Expr::Number(3))
            }
        );
    }

    #[test]
    fn test_parentheses() {
        let tokens = vec![
            Token::LeftParenthesis,
            Token::Number(2),
            Token::Plus,
            Token::Number(3),
            Token::RightParenthesis,
            Token::Asterisk,
            Token::Number(4),
        ]; // (2 + 3) * 4
        let ast = parse(&tokens).unwrap();

        assert_eq!(
            ast,
            Expr::BinaryOp {
                op: Token::Asterisk,
                left: Box::new(Expr::BinaryOp {
                    op: Token::Plus,
                    left: Box::new(Expr::Number(2)),
                    right: Box::new(Expr::Number(3)),
                }),
                right: Box::new(Expr::Number(4)),
            }
        );
    }

    #[test]
    fn test_nested_parentheses() {
        let tokens = vec![
            Token::Number(2),
            Token::Asterisk,
            Token::LeftParenthesis,
            Token::Number(3),
            Token::Plus,
            Token::LeftParenthesis,
            Token::Number(4),
            Token::Minus,
            Token::Number(1),
            Token::RightParenthesis,
            Token::RightParenthesis,
        ]; //2 * (3 + (4 - 1))
        let ast = parse(&tokens).unwrap();

        assert_eq!(
            ast,
            Expr::BinaryOp {
                op: Token::Asterisk,
                left: Box::new(Expr::Number(2)),
                right: Box::new(Expr::BinaryOp {
                    op: Token::Plus,
                    left: Box::new(Expr::Number(3)),
                    right: Box::new(Expr::BinaryOp {
                        op: Token::Minus,
                        left: Box::new(Expr::Number(4)),
                        right: Box::new(Expr::Number(1)),
                    }),
                }),
            }
        );
    }

    #[test]
    fn test_invalid_input_unexpected_token() {
        let tokens = vec![
            Token::Number(2),
            Token::Plus,
            Token::RightParenthesis,
            Token::Number(3),
        ]; // 2 + ) 3
        let result = parse(&tokens);

        assert!(result.is_err());
        assert_eq!(
            result,
            Err(ParseError::UnexpectedToken(Token::RightParenthesis))
        );
    }

    #[test]
    fn test_invalid_input_expression() {
        let tokens = vec![
            Token::Number(2),
            Token::Plus,
            Token::Number(3),
            Token::Number(4),
        ]; // 2 + 3 4
        let result = parse(&tokens);

        assert!(result.is_err());
        assert_eq!(result, Err(ParseError::InvalidExpression));
    }
}
