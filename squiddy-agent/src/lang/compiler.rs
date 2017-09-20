use super::ast::{Node, Processor};
use super::super::state::State;
use super::SquiddyProgram;

pub struct Compiler {

}

impl Processor for Compiler {
    type NodeType = Node;
    type ResultType = Result<SquiddyProgram, CompilerError>;

    fn process(&mut self, node: Self::NodeType) -> Self::ResultType {
        let mut state = State::default();

        match node {
            Node::VariableDefinition { var_name, var_type } => println!("{}", var_name)
        }

        Ok(SquiddyProgram::default())
    }
}

pub enum CompilerError {

}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    pub fn compile(ast: Node) -> Result<SquiddyProgram, CompilerError> {
        let mut processor = Compiler::new(); 

        processor.process(ast)
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn asdfasdf() {

    }
}
