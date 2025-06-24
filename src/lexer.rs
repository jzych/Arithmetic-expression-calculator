use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParenthesis,
    RightParenthesis,
}

pub fn tokenize(input: &String) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '0'..='9' => {
                tokens.push(Token::Number(parse_digits(&mut chars)));
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Asterisk);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Slash);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LeftParenthesis);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParenthesis);
                chars.next();
            }
            ' ' | '\t' | '\n' => {
                chars.next();
            }
            other => {
                return Err(format!("Invalid character: {other}"));
            }
        }
    }

    Ok(tokens)
}

fn parse_digits(chars: &mut std::iter::Peekable<Chars>) -> i64 {
    let mut num_str = String::new();

    while let Some(&digit) = chars.peek() {
        if digit.is_ascii_digit() {
            num_str.push(digit);
            chars.next();
        } else {
            break;
        }
    }
    num_str.parse::<i64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_expression() {
        let input = String::from("2 + (3 * 4)");
        let result = tokenize(&input).unwrap();

        assert_eq!(
            result,
            vec![
                Token::Number(2),
                Token::Plus,
                Token::LeftParenthesis,
                Token::Number(3),
                Token::Asterisk,
                Token::Number(4),
                Token::RightParenthesis,
            ]
        );
    }

    #[test]
    fn test_expression_with_long_digits() {
        let input = String::from("12 - 456 / 1234");
        let result = tokenize(&input).unwrap();

        assert_eq!(
            result,
            vec![
                Token::Number(12),
                Token::Minus,
                Token::Number(456),
                Token::Slash,
                Token::Number(1234),
            ]
        );
    }

    #[test]
    fn test_invalid_character() {
        let input = String::from("2 & 4");
        let result = tokenize(&input);

        assert_eq!(result, Err("Invalid character: &".to_string()));
    }
}
