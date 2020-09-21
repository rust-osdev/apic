use bit_field::BitField;
use core::convert::{TryFrom, TryInto};

#[derive(Debug, Copy, Clone)]
pub struct Version(u32);

impl Version {
    pub(crate) fn from_raw(value: u32) -> Self {
        Self(value)
    }

    pub fn max_redirection_entry(&self) -> u8 {
        self.0.get_bits(16..24).try_into().unwrap()
    }

    pub fn apic_version(&self) -> u8 {
        self.0.get_bits(0..8).try_into().unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Arbitration(u32);

impl Arbitration {
    pub(crate) fn from_raw(value: u32) -> Self {
        Self(value)
    }

    pub(crate) fn into_raw(self) -> u32 {
        self.0
    }

    pub fn new(arbitration_id: u8) -> Self {
        let mut value = 0;
        value.set_bits(24..28, arbitration_id.into());
        Self(value)
    }

    pub fn arbitration_id(&self) -> u8 {
        self.0.get_bits(24..28).try_into().unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RedirectionTableEntry {
    low: u32,
    high: u32,
}

impl RedirectionTableEntry {
    pub(crate) fn from_raw(low: u32, high: u32) -> Self {
        Self { low, high }
    }

    pub(crate) fn into_raw(self) -> (u32, u32) {
        (self.low, self.high)
    }

    pub fn vector(&self) -> u8 {
        self.low.get_bits(0..8).try_into().unwrap()
    }

    pub fn set_vector(&mut self, vector: u8) {
        self.low.set_bits(0..8, vector.into());
    }

    pub fn delivery_mode(&self) -> DeliveryMode {
        self.low.get_bits(8..11).try_into().unwrap()
    }

    pub fn set_delivery_mode(&mut self, mode: DeliveryMode) {
        let raw: u8 = mode.into();
        self.low.set_bits(8..11, raw.into());
    }

    pub fn destination_mode_logical(&self) -> bool {
        self.low.get_bit(11)
    }

    pub fn set_destination_mode_logical(&mut self, logical: bool) {
        self.low.set_bit(11, logical);
    }

    pub fn delivery_status_send_pending(&self) -> bool {
        self.low.get_bit(12)
    }

    pub fn polarity_low_active(&self) -> bool {
        self.low.get_bit(13)
    }

    pub fn set_polarity_low_active(&mut self, low_active: bool) {
        self.low.set_bit(13, low_active);
    }

    pub fn remote_irr(&self) -> bool {
        self.low.get_bit(14)
    }

    pub fn trigger_mode_level(&self) -> bool {
        self.low.get_bit(15)
    }

    pub fn set_trigger_mode_level(&mut self, level_sensitive: bool) {
        self.low.set_bit(15, level_sensitive);
    }

    pub fn masked(&self) -> bool {
        self.low.get_bit(16)
    }

    pub fn set_masked(&mut self, masked: bool) {
        self.low.set_bit(16, masked);
    }

    pub fn destination(&self) -> u8 {
        self.high.get_bits(24..32).try_into().unwrap()
    }

    pub fn set_destination(&mut self, destination: u8) {
        self.high.set_bits(24..32, destination.into());
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DeliveryMode {
    Fixed,
    LowestPriority,
    SystemManagementInterrupt,
    Reserved1,
    NonMaskableInterrupt,
    Init,
    Reserved2,
    ExtInt,
}

impl TryFrom<u8> for DeliveryMode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b000 => Ok(DeliveryMode::Fixed),
            0b001 => Ok(DeliveryMode::LowestPriority),
            0b010 => Ok(DeliveryMode::SystemManagementInterrupt),
            0b011 => Ok(DeliveryMode::Reserved1),
            0b100 => Ok(DeliveryMode::NonMaskableInterrupt),
            0b101 => Ok(DeliveryMode::Init),
            0b110 => Ok(DeliveryMode::Reserved2),
            0b111 => Ok(DeliveryMode::ExtInt),
            _other => Err(()),
        }
    }
}

impl TryFrom<u32> for DeliveryMode {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let value = u8::try_from(value).map_err(|_err| ())?;
        DeliveryMode::try_from(value)
    }
}

impl Into<u8> for DeliveryMode {
    fn into(self) -> u8 {
        match self {
            DeliveryMode::Fixed => 0b000,
            DeliveryMode::LowestPriority => 0b001,
            DeliveryMode::SystemManagementInterrupt => 0b010,
            DeliveryMode::Reserved1 => 0b011,
            DeliveryMode::NonMaskableInterrupt => 0b100,
            DeliveryMode::Init => 0b101,
            DeliveryMode::Reserved2 => 0b110,
            DeliveryMode::ExtInt => 0b111,
        }
    }
}
