use super::chunk::{Chunk, SEP};
use super::Value;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    /// Returns a Constant from the constants in the chunk.
    Return,
    GetConstant,
    GetConstantLong,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    Unit,
    True,
    False,
}

impl OpCode {
    pub fn disassemble(&self, chunk: &Chunk, offset: usize) -> usize {
        let end_offset = offset + self.arity();

        if end_offset > chunk.code.len() - 1 {
            panic!(
                "Error: Expected {} more instruction(s).",
                end_offset - (chunk.code.len() - 1)
            );
        }

        use OpCode::*;
        let repr = match self {
            GetConstant => {
                let index = chunk.code[offset + 1];
                let constant: &Value = chunk.get_constant(index as usize).unwrap();

                Some(format!(" {index} -> {constant}"))
            }
            GetConstantLong => {
                let p1 = chunk.code[offset + 1];
                let p2 = chunk.code[offset + 2];
                let p3 = chunk.code[offset + 3];
                let index: u32 = u32::from_be_bytes([0, p1, p2, p3]);

                let constant: &Value = chunk.get_constant(index as usize).unwrap();

                Some(format!(" {index} -> {constant}"))
            }
            _ => None,
        };

        let repr = repr.unwrap_or("".to_string());
        let line = chunk
            .get_line_no(offset)
            .map(|s| format!("{s:04?}"))
            .unwrap_or("...".to_string());

        println!("{line} {SEP} {self} {repr}");

        end_offset + 1
    }

    pub fn arity(&self) -> usize {
        use OpCode::*;

        match self {
            Return => 0,
            GetConstant => 1,
            GetConstantLong => 3,

            // Binary OpCodes
            Negate | Add | Subtract | Multiply | Divide | Equal | NotEqual | Less | LessEqual
            | Greater | GreaterEqual | LogicalAnd | LogicalOr | LogicalNot => 0,

            // Constant OpCodes
            Unit | True | False => 0,
        }
    }
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04} {SEP} {:?}", *self as u8, self)
    }
}

impl TryFrom<u8> for OpCode {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let res = match value {
            0 => OpCode::Return,
            1 => OpCode::GetConstant,
            2 => OpCode::GetConstantLong,
            3 => OpCode::Negate,
            4 => OpCode::Add,
            5 => OpCode::Subtract,
            6 => OpCode::Multiply,
            7 => OpCode::Divide,
            8 => OpCode::Equal,
            9 => OpCode::NotEqual,
            10 => OpCode::Less,
            11 => OpCode::LessEqual,
            12 => OpCode::Greater,
            13 => OpCode::GreaterEqual,
            14 => OpCode::LogicalAnd,
            15 => OpCode::LogicalOr,
            16 => OpCode::LogicalNot,
            _ => return Err("Invalid OpCode".to_string()),
        };

        Ok(res)
    }
}