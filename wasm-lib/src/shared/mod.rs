// --------------------------------------------------------------------------------
// Program Module
// --------------------------------------------------------------------------------
pub mod program {

    use wasm_bindgen::prelude::*;
    use serde::{Serialize, Deserialize};

    // --------------------------------------------------------------------------------
    // Common Registers
    // --------------------------------------------------------------------------------
    pub enum Register {
        R0,
        R1,
        R2,
        R3,
        R4,
        R5,
        R6,
        R7,
        R8,
        R9,
        R10,
        R11,
        R12,
        R13,
        R14,
        R15
    }

    impl Register {
        /// get the Register enum instance from either the Rx notation or the alias
        fn from_str(s: &str) -> Register {
            // TODO: s to lowercase

            match s {
                // R0
                "r0" => Register::R0,
                "a0" => Register::R0,
                // R1
                "r1" => Register::R1,
                "a1" => Register::R1,
                // R2
                "r2" => Register::R2,
                "a2" => Register::R2,
                // R3
                "r3" => Register::R3,
                "a3" => Register::R3,
                //R4
                "r4" => Register::R4,
                "v1" => Register::R4
                // R5
                "r5" => Register::R5,
                "v2" => Register::R5,
                // R6
                "r6" => Register::R6,
                "v3" => Register::R6,
                // R7
                "r7" => Register::R7,
                "v4" => Register::R7,
                // R8
                "r8" => Register::R8,
                "v5" => Register::R8,
                // R9
                "r9" => Register::R9,
                "v6" => Register::R9,
                // R10
                "r10" => Register::R10,
                "v7" => Register::R10,
                // R11
                "r11" => Register::R11,
                "v8" => Register::R11,
                "fp" => Register::R11,
                // R12
                "r12" => Register::R12,
                "ip" => Register::R12,
                // R13
                "r13" => Register::R13,
                "sp" => Register::R13,
                // R14
                "r14" => Register::R14,
                "lr" => Register::R14,
                // R15
                "r15" => Register::R15,
                "pc" => Register::R15,
                _ => panic!("Invalid register name: {}", s)
            }
        }

        fn to_string(&self) -> String {
            match self {
                Register::R0 => "r0".to_string(),
                Register::R1 => "r1".to_string(),
                Register::R2 => "r2".to_string(),
                Register::R3 => "r3".to_string(),
                Register::R4 => "r4".to_string(),
                Register::R5 => "r5".to_string(),
                Register::R6 => "r6".to_string(),
                Register::R7 => "r7".to_string(),
                Register::R8 => "r8".to_string(),
                Register::R9 => "r9".to_string(),
                Register::R10 => "r10".to_string(),
                Register::R11 => "r11".to_string(),
                Register::R12 => "r12".to_string(),
                Register::R13 => "r13".to_string(),
                Register::R14 => "r14".to_string(),
                Register::R15 => "r15".to_string(),
            }
        }
    }

    // --------------------------------------------------------------------------------
    // Status Registers
    // --------------------------------------------------------------------------------
    pub enum StatusRegister {
        CPSR,
        SPSR
    }

    impl StatusRegister {
        fn from_str(s: &str) -> StatusRegister {
            match s {
                "cpsr" => StatusRegister::CPSR,
                "spsr" => StatusRegister::SPSR,
                _ => panic!("Invalid special register name: {}", s)
            }
        }

        fn to_string(&self) -> String {
            match self {
                StatusRegister::CPSR => "cpsr".to_string(),
                StatusRegister::SPSR => "spsr".to_string(),
            }
        }
    }

    /// Temporary memory state struct used to represent a change
    /// Instruction: input -> execution -> output
    /// where input and output is a MemoryState
    pub struct DataState {
        /// register
        pub register: Register,
        pub registerValue: u32,

        /// status register
        pub statusRegister: StatusRegister,
        pub statusRegisterValue: u32

        /// memory
        pub memAdr: u32 // 2^32 memory space
        pub memValue: u32
    }

    #[derive(Serialize, Deserialize)]
    pub struct SerializableDataState {
        pub register: String,
        pub registerValue: u32
        pub statusRegister: String,
        pub statusRegisterValue: u32,
        pub memAdr: u32,
        pub memValue: u32
    }

    impl DataState {
        pub fn from_serializable(&self, state: &SerializableDataState) -> DataState {
            DataState {
                register: Register.from_str(state.register),
                registerValue: state.registerValue,
                statusRegister: StatusRegister.from_str(state.statusRegister),
                statusRegisterValue: state.statusRegisterValue,
                memAdr: state.memAdr,
                memValue: state.memValue
            }
        }

        pub fn to_serializable(&self) -> SeriazableDataState {
            StatusRegister {
                register: self.register.to_string(),
                registerValue: self.registerValue,
                statusRegister: self.statusRegister.to_string(),
                statusRegisterValue: self.statusRegisterValue,
                memAdr: self.memAdr,
                memValue: self.memValue
            }
        }
    }

    pub struct Instruction {
        pub opcode: String,
        pub condFlag: String?,
        pub cpsrFlag: String?,
        pub dest: Register,
        
    }
}
