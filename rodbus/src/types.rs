use crate::error::details::{ADUParseError, ExceptionCode, InvalidRequest};
use crate::service::function::FunctionCode;
use std::ops::Range;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct UnitId {
    id: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct AddressRange {
    pub start: u16,
    pub count: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct RegisterValue {
    pub value: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct Indexed<T> {
    pub index: u16,
    pub value: T,
}

mod constants {
    pub const ON: u16 = 0xFF00;
    pub const OFF: u16 = 0x0000;
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum CoilState {
    On = constants::ON,
    Off = constants::OFF,
}

impl CoilState {
    pub fn from_bool(value: bool) -> Self {
        if value {
            CoilState::On
        } else {
            CoilState::Off
        }
    }

    pub fn from_u16(value: u16) -> Result<Self, ADUParseError> {
        match value {
            constants::ON => Ok(CoilState::On),
            constants::OFF => Ok(CoilState::Off),
            _ => Err(ADUParseError::UnknownCoilState(value)),
        }
    }

    pub fn to_u16(self) -> u16 {
        self as u16
    }
}

impl RegisterValue {
    pub fn new(value: u16) -> Self {
        RegisterValue { value }
    }
}

impl AddressRange {
    pub const MAX_REGISTERS: u16 = 125;
    pub const MAX_BINARY_BITS: u16 = 2000;

    pub fn new(start: u16, count: u16) -> Self {
        AddressRange { start, count }
    }

    fn check_validity(&self, max_count: u16) -> Result<(), InvalidRequest> {
        // a count of zero is never valid
        if self.count == 0 {
            return Err(InvalidRequest::CountOfZero);
        }

        // check that start/count don't overflow u16
        let last_address = (self.start as u32) + (self.count as u32 - 1);
        if last_address > (std::u16::MAX as u32) {
            return Err(InvalidRequest::AddressOverflow(self.start, self.count));
        }

        if self.count > max_count {
            return Err(InvalidRequest::CountTooBigForType(self.count, max_count));
        }

        Ok(())
    }

    pub fn check_validity_for_bits(&self) -> Result<(), InvalidRequest> {
        self.check_validity(Self::MAX_BINARY_BITS)
    }

    pub fn check_validity_for_registers(&self) -> Result<(), InvalidRequest> {
        self.check_validity(Self::MAX_REGISTERS)
    }
}

impl<T> Indexed<T> {
    pub fn new(index: u16, value: T) -> Self {
        Indexed { index, value }
    }
}

impl UnitId {
    pub fn new(unit_id: u8) -> Self {
        Self { id: unit_id }
    }

    pub fn default() -> Self {
        Self { id: 0xFF }
    }

    pub fn to_u8(self) -> u8 {
        self.id
    }
}