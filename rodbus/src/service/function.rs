use std::fmt::{Display, Formatter};

use crate::error::*;
use crate::service::traits::Serialize;
use crate::util::cursor::WriteCursor;

mod constants {
    pub const READ_COILS: u8 = 1;
    pub const READ_DISCRETE_INPUTS: u8 = 2;
    pub const READ_HOLDING_REGISTERS: u8 = 3;
    pub const READ_INPUT_REGISTERS: u8 = 4;
    pub const WRITE_SINGLE_COIL: u8 = 5;
    pub const WRITE_SINGLE_REGISTER: u8 = 6;
    /*
    pub const WRITE_MULTIPLE_COILS: u8 = 15;
    pub const WRITE_MULTIPLE_REGISTERS: u8 = 16;
    */
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum FunctionCode {
    ReadCoils = constants::READ_COILS,
    ReadDiscreteInputs = constants::READ_DISCRETE_INPUTS,
    ReadHoldingRegisters = constants::READ_HOLDING_REGISTERS,
    ReadInputRegisters = constants::READ_INPUT_REGISTERS,
    WriteSingleCoil = constants::WRITE_SINGLE_COIL,
    WriteSingleRegister = constants::WRITE_SINGLE_REGISTER,
    /*
    WriteMultipleCoils = constants::WRITE_MULTIPLE_COILS,
    WriteMultipleRegisters = constants::WRITE_MULTIPLE_REGISTERS
    */
}

impl Display for FunctionCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            FunctionCode::ReadCoils => f.write_str("READ COILS"),
            FunctionCode::ReadDiscreteInputs => f.write_str("READ DISCRETE INPUTS"),
            FunctionCode::ReadHoldingRegisters => f.write_str("READ HOLDING REGISTERS"),
            FunctionCode::ReadInputRegisters => f.write_str("READ INPUT REGISTERS"),
            FunctionCode::WriteSingleCoil => f.write_str("WRITE SINGLE COIL"),
            FunctionCode::WriteSingleRegister => f.write_str("WRITE SINGLE REGISTERS"),
        }
    }
}

impl FunctionCode {
    pub const fn get_value(self) -> u8 {
        self as u8
    }

    pub const fn as_error(self) -> u8 {
        self.get_value() | 0x80
    }

    pub fn get(value: u8) -> Option<Self> {
        match value {
            constants::READ_COILS => Some(FunctionCode::ReadCoils),
            constants::READ_DISCRETE_INPUTS => Some(FunctionCode::ReadDiscreteInputs),
            constants::READ_HOLDING_REGISTERS => Some(FunctionCode::ReadHoldingRegisters),
            constants::READ_INPUT_REGISTERS => Some(FunctionCode::ReadInputRegisters),
            constants::WRITE_SINGLE_COIL => Some(FunctionCode::WriteSingleCoil),
            constants::WRITE_SINGLE_REGISTER => Some(FunctionCode::WriteSingleRegister),
            _ => None,
        }
    }
}

pub struct ADU<'a, T>
where
    T: Serialize,
{
    function: u8,
    body: &'a T,
}

impl<'a, T> ADU<'a, T>
where
    T: Serialize,
{
    pub fn new(function: u8, body: &'a T) -> Self {
        ADU { function, body }
    }
}

impl<'a, T> Serialize for ADU<'a, T>
where
    T: Serialize,
{
    fn serialize(&self, cursor: &mut WriteCursor) -> Result<(), Error> {
        cursor.write_u8(self.function)?;
        self.body.serialize(cursor)
    }
}
