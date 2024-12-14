use bytecode::{Bytecode, Opcode};

pub struct Vm {
    gas_meter: GasMeter,
}

impl Vm {
    #[allow(clippy::new_without_default)]
    pub fn new(max_gas: u32) -> Self {
        Self {
            gas_meter: GasMeter::new(0, max_gas),
        }
    }

    pub fn run(&mut self, bytecode: Bytecode) -> Result<(), RunError> {
        for opcode in bytecode.opcodes() {
            match opcode {
                Opcode::Noop => self
                    .gas_meter
                    .consume(1)
                    .map_err(|used| RunError::OutOfGas {
                        used,
                        max: self.gas_meter.max_gas,
                    })?,
            }
        }

        Ok(())
    }

    pub fn gas_used(&self) -> u32 {
        self.gas_meter.gas_used
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RunError {
    #[error("out of gas (max {max}, used {used})")]
    OutOfGas { used: u32, max: u32 },
}

pub struct GasMeter {
    gas_used: u32,
    max_gas: u32,
}

impl GasMeter {
    pub fn new(gas_used: u32, max_gas: u32) -> Self {
        Self { gas_used, max_gas }
    }

    pub fn consume(&mut self, gas: u32) -> Result<(), u32> {
        match self.gas_used.checked_add(gas) {
            Some(gas_used) => {
                if gas_used <= self.max_gas {
                    self.gas_used = gas_used;
                    Ok(())
                } else {
                    Err(gas_used)
                }
            }
            None => Err(u32::MAX),
        }
    }
}
