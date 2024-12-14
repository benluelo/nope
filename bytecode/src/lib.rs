pub struct Bytecode {
    opcodes: Vec<Opcode>,
}

impl Bytecode {
    /// Magic bytes that all *NopeVM* bytecode starts with.
    const MAGIC: [u8; 4] = *b"nope";

    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { opcodes: vec![] }
    }

    /// Push an opcode to this bytecode object.
    pub fn push(&mut self, opcode: Opcode) {
        self.opcodes.push(opcode);
    }

    pub fn opcodes(&self) -> &[Opcode] {
        &self.opcodes
    }

    /// Compile this bytecode object into raw *NopeVM* object format.
    pub fn compile(self) -> Vec<u8> {
        Self::MAGIC
            .into_iter()
            .chain(self.opcodes.into_iter().map(|opcode| opcode as u8))
            .collect()
    }

    /// Parse raw compiled *NopeVM* bytecode into opcodes.
    pub fn parse(raw: impl AsRef<[u8]>) -> Result<Self, BytecodeParseError> {
        let raw = raw.as_ref();

        if raw.get(0..4).is_none_or(|magic| magic != Self::MAGIC) {
            Err(BytecodeParseError::InvalidMagic)
        } else {
            let mut bytecode = Self::new();

            let mut raw = &raw[4..];

            loop {
                match Opcode::eat(&mut raw) {
                    Ok(Some(opcode)) => bytecode.push(opcode),
                    Ok(None) => break,
                    Err(err) => return Err(BytecodeParseError::InvalidOpcode(err)),
                }
            }

            Ok(bytecode)
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BytecodeParseError {
    #[error("invalid header magic bytes")]
    InvalidMagic,
    #[error(transparent)]
    InvalidOpcode(#[from] InvalidOpcodeError),
}

pub enum Opcode {
    Noop,
}

impl Opcode {
    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            Opcode::Noop => vec![0x00],
        }
    }

    /// Attempt to "eat" an opcode off of the front of the provided input,
    /// returning the parsed opcode on success. The input is then pointed to the
    /// start of the next opcode.
    pub fn eat(bytes: &mut &[u8]) -> Result<Option<Self>, InvalidOpcodeError> {
        match bytes {
            [] => Ok(None),
            [0x00, ..] => {
                *bytes = &bytes[1..];
                Ok(Some(Self::Noop))
            }
            [start_byte, ..] => Err(InvalidOpcodeError {
                start_byte: *start_byte,
            }),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("invalid opcode (first invalid byte: {start_byte:x})")]
pub struct InvalidOpcodeError {
    pub start_byte: u8,
}
