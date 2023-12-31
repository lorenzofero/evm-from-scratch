use std::{error::Error, fmt};

use primitive_types::U256;

use super::types::ExecutionContext;

pub enum EVMError {
    NoBytecodeError(ExecutionContext),
    FromStrRadixError(String, ExecutionContext),
    NoOpcodeError(u8, ExecutionContext),
    EmptyStackError(ExecutionContext),
    U256ToUSizeError(U256, ExecutionContext),
    U256ToU8Error(U256, ExecutionContext),
    InvalidJumpdestError(usize, ExecutionContext),
}

impl fmt::Display for EVMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EVMError::EmptyStackError(_) => {
                write!(f, "cannot pop from empty stack")
            }
            EVMError::NoOpcodeError(op, _) => {
                write!(f, "cannot find opcode {:x?}", op)
            }
            EVMError::NoBytecodeError(ctx) => {
                write!(f, "cannot find code at pc {}", ctx.machine_state.pc)
            }
            EVMError::FromStrRadixError(..) => {
                write!(f, "cannot parse string to hex")
            }
            EVMError::U256ToUSizeError(val, _) => {
                write!(f, "cannot convert from U256 {:x?} to usize", val)
            }
            EVMError::U256ToU8Error(val, _) => {
                write!(f, "cannot convert from U256 {:x?} to u8", val)
            }
            EVMError::InvalidJumpdestError(val, _) => {
                write!(f, "cannot jump to opcode {:x?}", val)
            }
        }
    }
}

impl fmt::Debug for EVMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EVMError::NoBytecodeError(ctx) => {
                write!(f, "NoBytecodeError\n    ctx: {:x?}", ctx)
            }
            EVMError::FromStrRadixError(str, ctx) => {
                write!(
                    f,
                    "FromStrRadixError\n    str: {},\n    ctx: {:x?}",
                    str, ctx
                )
            }
            EVMError::NoOpcodeError(op, ctx) => {
                write!(f, "NoOpcodeError\n    op: {:x},\n    ctx: {:x?}", op, ctx)
            }
            EVMError::EmptyStackError(ctx) => {
                write!(f, "EmptyStackError\n    ctx: {:x?}", ctx)
            }
            EVMError::U256ToUSizeError(val, ctx) => {
                write!(
                    f,
                    "U256ToUSizeError\n    val: {:x}\n    ctx: {:x?}",
                    val, ctx
                )
            }
            EVMError::U256ToU8Error(val, ctx) => {
                write!(f, "U256ToU8Error\n    val: {:x}\n    ctx: {:x?}", val, ctx)
            }
            EVMError::InvalidJumpdestError(val, ctx) => {
                write!(
                    f,
                    "InvalidJumpdestError\n    val: {:x}\n    ctx: {:x?}",
                    val, ctx
                )
            }
        }
    }
}

impl Error for EVMError {}
