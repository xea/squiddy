use bytes::{Bytes, BytesMut, BufMut};

pub struct Tokenizer;

impl Tokenizer {
    pub fn tokenize(bytes: Bytes) -> Vec<Token> {
        bytes.iter().fold((vec![], None, BytesMut::with_capacity(64)), |mut acc: (Vec<Token>, Option<LongTokenType>, BytesMut), current_byte| {
            match acc.1 {
                Some(token_type) => (),
                // New token
                None => {
                    acc.1 = match *current_byte {
                        0...32 => Some(LongTokenType::Whitespace),
                        // Symbols
                        33...47 => None,
                        // Digits
                        48...57 => Some(LongTokenType::Number),
                        // Symbols
                        58...64 => None,
                        // Upper case letters
                        65...90 => Some(LongTokenType::Word),
                        // Symbols
                        91...96 => None,
                        // Lower case letters
                        97...122 => Some(LongTokenType::Word),
                        // Symbols
                        123...126 => None,
                        _ => Some(LongTokenType::Unknown)
                    };

                    acc.2.put_u8(*current_byte);
                }
            }

            acc
        }).0
    }
}

#[derive(Copy, Clone)]
enum LongTokenType {
    Word,
    Whitespace,
    Number,
    Unknown
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Word(String),
    Unknown(u8)
//    Whitespace(usize),
//    Indentation(usize)
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn a_single_word_is_tokenised_into_a_word_token() {
        let input = "hello";
        let tokenizer = Tokenizer;

        assert_eq!(vec![ Token::Word(String::from("hello")) ], tokenizer.tokenize(Bytes::from(input)));
    }

    #[test]
    fn leading_whitespaces_are_recognised_tokens() {
        let input = "  hello";
        let tokenizer = Tokenizer;

        assert_eq!(vec![ Token::Whitespace(2) ], tokenizer.tokenize(Bytes::from(input)));
    }
}
