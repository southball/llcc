use crate::{
    node::Node,
    token::{Token, TokenData},
};

pub struct TokenizerError<'a> {
    pub source: &'a str,
    pub position: usize,
    pub message: String,
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, TokenizerError> {
    let chars = source.chars().collect::<Vec<_>>();
    let mut i = 0;
    let mut tokens = vec![];

    while i < chars.len() {
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }

        if chars[i].is_alphabetic() {
            // Keyword detection
            if i + 6 < chars.len() {
                let maybe_return = chars[i..i + 6].iter().collect::<String>();
                if maybe_return == "return" {
                    tokens.push(Token {
                        data: TokenData::Return,
                        position: i,
                    });
                    i += 6;
                    continue;
                }
            }

            let mut j = i;
            while j < chars.len() && chars[j].is_alphabetic() {
                j += 1;
            }
            tokens.push(Token {
                data: TokenData::Ident(chars[i..j].iter().collect::<String>()),
                position: i,
            });
            i = j;
            continue;
        }

        if i + 1 < chars.len() {
            let op = chars[i..=i + 1].iter().collect::<String>();
            if op == "==" || op == "!=" || op == "<=" || op == ">=" {
                tokens.push(Token {
                    data: TokenData::Reserved(op),
                    position: i,
                });
                i += 2;
                continue;
            }
        }

        {
            let op = chars[i];
            if op == '+'
                || op == '-'
                || op == '*'
                || op == '/'
                || op == '('
                || op == ')'
                || op == '<'
                || op == '>'
                || op == ';'
                || op == '='
            {
                tokens.push(Token {
                    data: TokenData::Reserved([op].into_iter().collect::<String>()),
                    position: i,
                });
                i += 1;
                continue;
            }
        }

        if chars[i].is_digit(10) {
            let mut j = i;
            while j < chars.len() && chars[j].is_digit(10) {
                j += 1;
            }
            let Ok(parsed_num) = chars[i..j].iter().collect::<String>().parse::<i32>() else {
                return Err(TokenizerError {
                    source, position: i, message: "Invalid number.".to_string()
                });
            };
            tokens.push(Token {
                data: TokenData::Num(parsed_num),
                position: i,
            });
            i = j;
            continue;
        }

        return Err(TokenizerError {
            source,
            position: i,
            message: "Failed to parse.".to_string(),
        });
    }

    tokens.push(Token {
        data: TokenData::Eof,
        position: i,
    });

    Ok(tokens)
}
