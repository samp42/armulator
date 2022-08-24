pub mod memory {

    use wasm_bindgen::prelude::*;

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
        fn from_str(s: &str) -> Register {
            match s {
                "r0" => Register::R0,
                "r1" => Register::R1,
                "r2" => Register::R2,
                "r3" => Register::R3,
                "r4" => Register::R4,
                "r5" => Register::R5,
                "r6" => Register::R6,
                "r7" => Register::R7,
                "r8" => Register::R8,
                "r9" => Register::R9,
                "r10" => Register::R10,
                "r11" => Register::R11,
                "r12" => Register::R12,
                "r13" => Register::R13,
                "r14" => Register::R14,
                "r15" => Register::R15,
                _ => panic!("Invalid register name: {}", s)
            }
        }

        fn from_alias(s: &str) -> Register {
            match s {
                "a0" => Register::R0,
                "a1" => Register::R1,
                "a2" => Register::R2,
                "a3" => Register::R3,
                "v1" => Register::R4,
                "v2" => Register::R5,
                "v3" => Register::R6,
                "v4" => Register::R7,
                "v5" => Register::R8,
                "v6" => Register::R9,
                "v7" => Register::R10,
                "v8" => Register::R11,
                "fp" => Register::R11,
                "ip" => Register::R12,
                "sp" => Register::R13,
                "lr" => Register::R14,
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
    // Special Registers
    // --------------------------------------------------------------------------------
    pub enum SpecialRegister {
        CPSR,
        SPSR
    }

    impl SpecialRegister {
        fn from_str(s: &str) -> SpecialRegister {
            match s {
                "cpsr" => SpecialRegister::CPSR,
                "spsr" => SpecialRegister::SPSR,
                _ => panic!("Invalid special register name: {}", s)
            }
        }

        fn to_string(&self) -> String {
            match self {
                SpecialRegister::CPSR => "cpsr".to_string(),
                SpecialRegister::SPSR => "spsr".to_string(),
            }
        }
    }

    // --------------------------------------------------------------------------------
    // Instruction Memory Fields
    // --------------------------------------------------------------------------------
    struct InsructionMemFields {
        pub register: Register,
        pub value: u32,
        pub specialRegister: SpecialRegister,
        pub specialValue: u32,
        // assumming 4 GB memory space (might want to reduce it)
        pub memAdr: u32,
        pub memValue: u32,
    }

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
