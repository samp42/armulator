// --------------------------------------------------------------------------------
// Memory Module
// --------------------------------------------------------------------------------
pub mod memory {

    use wasm_bindgen::prelude::*;
    use serde::{Serialize, Deserialize};


    // --------------------------------------------------------------------------------
    // Instruction Memory Fields
    // --------------------------------------------------------------------------------
    struct InsructionMemFields {
        pub register: shared::memory::Register,
        pub value: u32,
        pub specialRegister: shared::memory::SpecialRegister,
        pub specialValue: u32,
        // assumming 4 GB memory space (might want to reduce it)
        pub memAdr: u32,
        pub memValue: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SerializableInstructionMemFields {
        pub register: String,
        pub value: u32,
        pub specialRegister: String,
        pub specialValue: u32,
        pub memAdr: u32,
        pub memValue: u32,
    }

    impl InsructionMemFields {
        pub fn to_serializable(&self) -> SerializableInstructionMemFields {
            SerializableInstructionMemFields {
                register: self.register.to_string(),
                value: self.value,
                specialRegister: self.specialRegister.to_string(),
                specialValue: self.specialValue,
                memAdr: self.memAdr,
                memValue: self.memValue,
            }
        }
    }

}

// --------------------------------------------------------------------------------
// Instruction Module
// --------------------------------------------------------------------------------
pub mod instruction {
    // --------------------------------------------------------------------------------
    // Opcodes
    // --------------------------------------------------------------------------------

    trait Execute {
        // get the opcode of the instruction
        fn mnemonic(&self) -> String;
        // execute the instruction
        fn execute(&self, memory: &mut memory::Memory);
    }

    // Mnemonic: move
    pub impl 
}
