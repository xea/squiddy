use super::ast::Node;
use super::SquiddyProgram;

pub struct Compiler {

}

pub enum CompilerError {

}

impl Compiler {
    pub fn compile(ast: Node) -> Result<SquiddyProgram, CompilerError> {
        let mut program = SquiddyProgram::default();

        program.state.register_u32();

        Ok(program)
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn asdfasdf() {

    }
}
