use super::token::Token;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyInput
}

/// YourLanguage.parse("1 + 2 * 3").should ==
/// s(:add,
///  s(:integer, 1),
///   s(:multiply,
///    s(:integer, 2),
///    s(:integer, 3)))
#[derive(Debug, PartialEq)]
pub enum Node {
    VariableDefinition { var_name: String, var_type: VarType },
}

#[derive(Debug, PartialEq)]
pub enum VarType {
    UnsignedInt
}

pub struct Parser {

}

impl Parser {

    pub fn parse(ast: &Vec<Token>) -> Result<Node, ParseError> {
        if ast.is_empty() {
            Err(ParseError::EmptyInput)
        } else {
            Ok(Node::VariableDefinition { var_name: String::from("global_counter"), var_type: VarType::UnsignedInt })
        }
    }
}

pub trait Processor {
    type NodeType;
    type ResultType;

    fn process(&mut self, node: Self::NodeType) -> Self::ResultType;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_an_empty_token_list_results_in_an_empty_input_error() {
        assert_eq!(Err(ParseError::EmptyInput), Parser::parse(&vec![]));
    }
}
