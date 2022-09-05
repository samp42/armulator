// mod shared;

// pub use crate::shared;

pub fn parse(prog: String) -> String {
    match tokenizer::parse_line(prog) {
        Ok(ok) => return ok,
        Err(err) => return err
    };
}

// --------------------------------------------------------------------------------
// Tokenizer Module
// --------------------------------------------------------------------------------
mod tokenizer {

    pub mod tokenizer_errors {

        pub fn UNKNOWN_OPCODE_ERROR(args: &str) -> String{
            return format!("`{}` is not a valid opcode", args);
        }

    }

    use crate::interpreter::tokenizer::tokenizer_errors::*;


    /// Parse each line
    /// Returns Result with Instruction to add to program if successful or an error if applicable
    pub fn parse_line(s: String) -> Result<String, String> {
        return Err(UNKNOWN_OPCODE_ERROR("OPCODE"));
    }

}
