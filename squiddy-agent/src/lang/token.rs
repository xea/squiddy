use bytes::{Bytes, BytesMut, BufMut};

pub struct Tokenizer;

impl Tokenizer {
    pub fn tokenize(bytes: Bytes) -> Vec<Token> {
        bytes.iter().fold(vec![], |mut acc: Vec<(TokenType, BytesMut)>, current_byte| {
            let token_type = Tokenizer::get_token_type(*current_byte);

            // Remove the last item, if it exists
            if let Some(mut last_item) = acc.pop() {
                if last_item.0 == token_type && token_type.is_long_token() {
                    // If the last item's token type matches the current one and it's a long item, we append the current byte
                    last_item.1.put_u8(*current_byte);

                    acc.push(last_item);
                } else {
                    acc.push(last_item);
                    // If it's either a different type or it wasn't a long token, we create a new token
                    let mut bytes = BytesMut::with_capacity(32);
                    bytes.put_u8(*current_byte);

                    acc.push((token_type, bytes));
                }
            } else {
                // initialising the first item in the list
                let mut bytes = BytesMut::with_capacity(32);
                bytes.put_u8(*current_byte);
                acc.push((token_type, bytes));
            };

            acc
        }).iter().map(|temp_token| {
            Tokenizer::parse_buffer(temp_token.0, temp_token.1.as_ref().to_vec())
        }).collect()
    }

    fn parse_buffer(token_type: TokenType, buffer: Vec<u8>) -> Token {
        // TODO proper error handling
        match token_type {
            TokenType::Number => String::from_utf8(buffer)
                .map(|number| u64::from_str_radix(&number, 10).unwrap_or(0))
                .map(|number| Token::Number(number))
                .unwrap_or(Token::Number(0)),
            TokenType::Word => match String::from_utf8(buffer) {
                Ok(word) => Token::Word(word),
                Err(_) => Token::Unknown(b'e')
            },
            TokenType::Whitespace => Token::Whitespace(buffer.len()),
            TokenType::Tab => Token::Indentation(buffer.len()),
            // TODO make this handle \r\n sequences as a single new line
            TokenType::NewLine => Token::NewLine(buffer.len()),
            TokenType::Symbol => Token::Symbol(buffer[0]),
            _ => Token::Unknown(b'a')
        }
    }

    fn get_token_type(byte: u8) -> TokenType {
        match byte {
            // Control characters
            0...8 => TokenType::Unknown,
            // Horizontal tab
            9 => TokenType::Tab,
            // Carriage return + line feed
            10 | 13 => TokenType::NewLine,
            // More control characters
            11 | 12 | 14...32 => TokenType::Whitespace,
            // Symbols !@#$%^&*()+,./
            33...47 => TokenType::Symbol,
            // Digits
            48...57 => TokenType::Number,
            // Symbols :;<>=?@
            58...64 => TokenType::Symbol,
            // Upper case letters
            65...90 => TokenType::Word,
            // Symbols []\^_`
            91...96 => TokenType::Symbol,
            // Lower case letters
            97...122 => TokenType::Word,
            // Symbols {}|~
            123...126 => TokenType::Symbol,
            _ => TokenType::Unknown
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum TokenType {
    Word,
    Whitespace,
    Tab,
    NewLine,
    Number,
    Symbol,
    Unknown
}

impl TokenType {
    pub fn is_long_token(&self) -> bool {
        match self {
            &TokenType::Symbol => false,
            _ => true
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Word(String),
    Number(u64),
    Symbol(u8),
    Whitespace(usize),
    NewLine(usize),
    Indentation(usize),
    Unknown(u8)
//    Indentation(usize)
}


#[cfg(test)]
mod tests {

    use super::*;
    use bytes::Bytes;

    #[test]
    fn empty_input_is_tokenized_into_empty_vec() {
        assert!(Tokenizer::tokenize(Bytes::from("")).is_empty());
    }

    #[test]
    fn symbols_are_tokenized_individually() {
        let expected_result = vec![
            Token::Symbol(b'#'),
            Token::Symbol(b'!'),
            Token::Symbol(b'$'),
            Token::Symbol(b'"'),
            Token::Symbol(b'\'')
        ];

        assert_eq!(expected_result, Tokenizer::tokenize(Bytes::from("#!$\"'")));
    }

    #[test]
    fn alphanumerical_words_are_tokenized_into_word_tokens() {
        assert_eq!(vec![ Token::Word(String::from("hello")) ], Tokenizer::tokenize(Bytes::from("hello")));
    }

    #[test]
    fn integral_numbers_are_tokenized_into_number_tokens() {
        assert_eq!(vec![ Token::Number(5534) ], Tokenizer::tokenize(Bytes::from("5534")));
    }

    #[test]
    fn mixed_integrals_and_alphanumericals_are_tokenized_separately() {
        assert_eq!(vec![ Token::Word(String::from("hello")), Token::Number(114), Token::Word(String::from("world")) ], Tokenizer::tokenize(Bytes::from("hello114world")));
    }

    #[test]
    fn whitespaces_are_tokenized_collectively() {
        assert_eq!(vec![ Token::Whitespace(3) ], Tokenizer::tokenize(Bytes::from("   ")));
        assert_eq!(vec![ 
            Token::Whitespace(2), 
            Token::Word(String::from("hello")), 
            Token::Whitespace(4),
            Token::Word(String::from("world")),
            Token::Whitespace(3)
            ], Tokenizer::tokenize(Bytes::from("  hello    world   ")));
    }

    #[test]
    fn new_lines_are_parsed_as_new_line_tokens() {
        assert_eq!(vec![
            Token::Word(String::from("hello")),
            Token::NewLine(1),
            Token::Word(String::from("world")),
            Token::NewLine(1)
        ], Tokenizer::tokenize(Bytes::from("hello\nworld\n")))
    }
}
