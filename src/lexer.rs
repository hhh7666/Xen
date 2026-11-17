#[derive(Debug)]
#[allow(dead_code)]
pub enum Token {
    CellMembrane,
    IonChannel,
    MembranePotential,
    Trigger,
    ActionPotential,
    Identifier(String),
    Integer(i32),
    End,
    If,
    Else,
    While,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
            continue;
        }

        if ch.is_alphabetic() || ch == '_' {
            let mut word = String::new();

            while let Some(&ch) = chars.peek() {
                if ch.is_alphanumeric() || ch == '_' {
                    word.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }

            let token = match word.as_str() {
                "cell_membrane" => Token::CellMembrane,
                "ion_channel" => Token::IonChannel,
                "membrane_potential" => Token::MembranePotential,
                "trigger" => Token::Trigger,
                "action_potential" => Token::ActionPotential,
                "if" => Token::If,
                "else" => Token::Else,
                "while" => Token::While,
                "end" => Token::End,
                _ => Token::Identifier(word),
            };

            tokens.push(token);
            continue;
        }

        if ch.is_ascii_digit() || ch == '-' {
            let mut number = String::new();

            if ch == '-' {
                number.push(ch);
                chars.next();
            }

            while let Some(&ch) = chars.peek() {
                if ch.is_ascii_digit() {
                    number.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }

            if let Ok(value) = number.parse::<i32>() {
                tokens.push(Token::Integer(value));
            }

            continue;
        }

        // Skip symbols that do not have tokens yet, such as ':', '{' and '}'.
        chars.next();
    }

    tokens
}
