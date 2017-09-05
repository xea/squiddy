pub use self::encoder::*;
pub use self::decoder::*;

pub mod encoder;
pub mod decoder;

/// The fixed length of an opcode in bytes
const OPCODE_LENGTH: usize = 2;
/// Client names MUST NOT be longer than this limit
const MAX_CLIENT_NAME_LENGTH: u8 = 32;

/// Provides a list of possible opcodes describing individual protocol messages.
enum OpCode {
    ServerHello = 0x01,
    ClientHello = 0x02,
    /// Denotes an invalid opcode, mostly used for decoding a message that has an unrecognisable opcode
    Invalid
}

impl From<u16> for OpCode {
    fn from(opcode: u16) -> OpCode {
        match opcode {
            0x01 => OpCode::ServerHello,
            0x02 => OpCode::ClientHello,
            _ => OpCode::Invalid
        }
    }
}
