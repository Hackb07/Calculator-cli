use std::io::{self, Write};

fn main() {
    println!("Rust Calculator Demo");
    println!("==================");
    
    let test_expressions = vec![
        "2+3",
        "10-4", 
        "3*4",
        "8/2",
        "2+3*4",
        "(2+3)*4",
        "2.5+1.5",
        "-5+3",
        "(2+3)*(4-2)/3"
    ];
    
    for expr in test_expressions {
        match calculate(expr) {
            Ok(result) => println!("{} = {}", expr, result),
            Err(e) => println!("{} Error: {}", expr, e),
        }
    }
    
    println!("\nInteractive mode (enter expressions, 'quit' to exit):");
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                
                if input == "quit" {
                    break;
                }
                
                if !input.is_empty() {
                    match calculate(input) {
                        Ok(result) => println!("= {}", result),
                        Err(e) => println!("Error: {}", e),
                    }
                }
            }
            Err(_) => break,
        }
    }
}

fn calculate(expression: &str) -> Result<f64, String> {
    let expression = expression.replace(" ", "");
    
    if expression.is_empty() {
        return Err("Empty expression".to_string());
    }
    
    let tokens = tokenize(&expression)?;
    let result = evaluate_tokens(&tokens)?;
    
    Ok(result)
}

fn tokenize(expression: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = expression.chars().peekable();
    
    while let Some(&ch) = chars.peek() {
        match ch {
            '+' => {
                tokens.push(Token::Operator(Operator::Add));
                chars.next();
            }
            '-' => {
                tokens.push(Token::Operator(Operator::Subtract));
                chars.next();
            }
            '*' => {
                tokens.push(Token::Operator(Operator::Multiply));
                chars.next();
            }
            '/' => {
                tokens.push(Token::Operator(Operator::Divide));
                chars.next();
            }
            '(' => {
                tokens.push(Token::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                chars.next();
            }
            '0'..='9' | '.' => {
                let mut number = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_digit(10) || ch == '.' {
                        number.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                number.parse::<f64>()
                    .map(|n| tokens.push(Token::Number(n)))
                    .map_err(|_| format!("Invalid number: {}", number))?;
            }
            _ => return Err(format!("Invalid character: {}", ch)),
        }
    }
    
    Ok(tokens)
}

fn evaluate_tokens(tokens: &[Token]) -> Result<f64, String> {
    let mut tokens = tokens.to_vec();
    let result = parse_expression(&mut tokens)?;
    
    if !tokens.is_empty() {
        return Err("Unexpected tokens".to_string());
    }
    
    Ok(result)
}

fn parse_expression(tokens: &mut Vec<Token>) -> Result<f64, String> {
    parse_term(tokens)
}

fn parse_term(tokens: &mut Vec<Token>) -> Result<f64, String> {
    let mut left = parse_factor(tokens)?;
    
    while !tokens.is_empty() {
        match tokens[0] {
            Token::Operator(Operator::Add) => {
                tokens.remove(0);
                let right = parse_factor(tokens)?;
                left = left + right;
            }
            Token::Operator(Operator::Subtract) => {
                tokens.remove(0);
                let right = parse_factor(tokens)?;
                left = left - right;
            }
            _ => break,
        }
    }
    
    Ok(left)
}

fn parse_factor(tokens: &mut Vec<Token>) -> Result<f64, String> {
    let mut left = parse_primary(tokens)?;
    
    while !tokens.is_empty() {
        match tokens[0] {
            Token::Operator(Operator::Multiply) => {
                tokens.remove(0);
                let right = parse_primary(tokens)?;
                left = left * right;
            }
            Token::Operator(Operator::Divide) => {
                tokens.remove(0);
                let right = parse_primary(tokens)?;
                if right == 0.0 {
                    return Err("Division by zero".to_string());
                }
                left = left / right;
            }
            _ => break,
        }
    }
    
    Ok(left)
}

fn parse_primary(tokens: &mut Vec<Token>) -> Result<f64, String> {
    if tokens.is_empty() {
        return Err("Unexpected end of expression".to_string());
    }
    
    match tokens.remove(0) {
        Token::Number(n) => Ok(n),
        Token::LeftParen => {
            let result = parse_expression(tokens)?;
            if tokens.is_empty() || !matches!(tokens.remove(0), Token::RightParen) {
                return Err("Missing closing parenthesis".to_string());
            }
            Ok(result)
        }
        Token::Operator(Operator::Subtract) => {
            let value = parse_primary(tokens)?;
            Ok(-value)
        }
        _ => Err("Unexpected token".to_string()),
    }
}

#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    Operator(Operator),
    LeftParen,
    RightParen,
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_addition() {
        assert_eq!(calculate("2+3").unwrap(), 5.0);
    }

    #[test]
    fn test_basic_subtraction() {
        assert_eq!(calculate("10-4").unwrap(), 6.0);
    }

    #[test]
    fn test_basic_multiplication() {
        assert_eq!(calculate("3*4").unwrap(), 12.0);
    }

    #[test]
    fn test_basic_division() {
        assert_eq!(calculate("8/2").unwrap(), 4.0);
    }

    #[test]
    fn test_operator_precedence() {
        assert_eq!(calculate("2+3*4").unwrap(), 14.0);
        assert_eq!(calculate("2*3+4").unwrap(), 10.0);
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(calculate("(2+3)*4").unwrap(), 20.0);
        assert_eq!(calculate("2*(3+4)").unwrap(), 14.0);
    }

    #[test]
    fn test_nested_parentheses() {
        assert_eq!(calculate("(2+(3*4))*2").unwrap(), 28.0);
    }

    #[test]
    fn test_decimal_numbers() {
        assert_eq!(calculate("2.5+1.5").unwrap(), 4.0);
        assert_eq!(calculate("3.14*2").unwrap(), 6.28);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(calculate("-5+3").unwrap(), -2.0);
        assert_eq!(calculate("2*-3").unwrap(), -6.0);
    }

    #[test]
    fn test_complex_expression() {
        assert_eq!(calculate("(2+3)*(4-2)/3").unwrap(), 10.0/3.0);
    }

    #[test]
    fn test_division_by_zero() {
        assert!(calculate("5/0").is_err());
    }

    #[test]
    fn test_invalid_expression() {
        assert!(calculate("").is_err());
        assert!(calculate("2+*3").is_err());
        assert!(calculate("(2+3").is_err());
    }
}