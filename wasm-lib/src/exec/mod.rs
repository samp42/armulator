// --------------------------------------------------------------------------------
// Instruction Module
// --------------------------------------------------------------------------------
pub mod instruction {

    /// A generic trait used to indicate the operation to execute for each opcode
    trait Execute {
        /// execute the instruction
        fn execute(&self, dest: &mut shared::memory::DataState);
    }

    // --------------------------------------------------------------------------------
    // Opcodes
    // --------------------------------------------------------------------------------
    /// Mnemonic: move
    pub impl MOV for Execute {
        fn execute(&self, state: &mut shared::memory::DataState) {
            
        }
    } 
}
