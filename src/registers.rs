use core::{convert::TryInto, sync::atomic::AtomicU32, sync::atomic::Ordering};

use bit_field::BitField;

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Id(u32);

impl Id {
    /// Returns the unique APIC ID value assigned to this specific CPU core
    pub fn id(&self) -> u8 {
        self.0.get_bits(24..).try_into().unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Version(u32);

impl Version {
    /// Indicates  the  version  number  of  the  APIC implementation
    pub fn version(&self) -> u8 {
        self.0.get_bits(..8).try_into().unwrap()
    }

    /// Specifies the number of entries in the localvector table minus one
    pub fn max_lvt_entries(&self) -> u8 {
        self.0.get_bits(16..24).try_into().unwrap()
    }

    /// Indicates thepresence of an extended APIC register space
    pub fn extended_apic_register_space_present(&self) -> bool {
        self.0.get_bit(31)
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct ExtendedApicFeature(u32);

impl ExtendedApicFeature {
    /// Specifies the number of extended local vector tableregisters in the local APIC
    pub fn extended_lvt_count(&self) -> u8 {
        self.0.get_bits(16..24).try_into().unwrap()
    }

    /// Indicates  that  the  processor  is  capable  ofsupporting an 8-bit APIC ID
    pub fn extended_apic_id_capability(&self) -> bool {
        self.0.get_bit(2)
    }

    /// Indicates that the Specific End Of Interrupt Register is present
    pub fn specific_end_of_interrupt_capable(&self) -> bool {
        self.0.get_bit(1)
    }

    /// Indicates  that  the  Interrupt  EnableRegisters are present
    pub fn interrupt_enable_register_capable(&self) -> bool {
        self.0.get_bit(1)
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct ExtendedApicControl(u32);

impl ExtendedApicControl {
    pub fn extended_apic_id_enabled(&self) -> bool {
        self.0.get_bit(2)
    }

    pub fn enable_extended_apic_id(&mut self, enable: bool) {
        self.0.set_bit(2, enable);
    }

    pub fn specific_end_of_interrupt_generation_enabled(&self) -> bool {
        self.0.get_bit(1)
    }

    pub fn enable_specific_end_of_interrupt_generation(&mut self, enable: bool) {
        self.0.set_bit(1, enable);
    }

    pub fn interrupt_enable_registers_enabled(&self) -> bool {
        self.0.get_bit(0)
    }

    pub fn enable_interrupt_enable_registers(&mut self, enable: bool) {
        self.0.set_bit(0, enable);
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct SpuriousInterruptVector(u32);

impl SpuriousInterruptVector {
    pub fn vector(&self) -> u8 {
        self.0.get_bits(..8).try_into().unwrap()
    }

    pub fn set_vector(&mut self, vector: u8) {
        self.0.set_bits(..8, vector.into());
    }

    pub fn apic_software_enabled(&self) -> bool {
        self.0.get_bit(8)
    }

    pub fn enable_apic_software(&mut self, enable: bool) {
        self.0.set_bit(8, enable);
    }

    pub fn focus_cpu_core_checking(&self) -> bool {
        self.0.get_bit(9)
    }

    pub fn set_focus_cpu_core_checking(&mut self, value: bool) {
        self.0.set_bit(9, value);
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct TimerLocalVectorTableEntry(u32);

impl TimerLocalVectorTableEntry {
    pub fn vector(&self) -> u8 {
        self.0.get_bits(..8).try_into().unwrap()
    }

    pub fn set_vector(&mut self, vector: u8) {
        self.0.set_bits(..8, vector.into());
    }

    pub fn delivery_status(&self) -> bool {
        self.0.get_bit(12)
    }

    pub fn mask(&self) -> bool {
        self.0.get_bit(16)
    }

    pub fn set_mask(&mut self, disable: bool) {
        self.0.set_bit(16, disable);
    }

    pub fn timer_mode(&self) -> bool {
        self.0.get_bit(17)
    }

    pub fn set_timer_mode(&mut self, periodic: bool) {
        self.0.set_bit(17, periodic);
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct TimerInitialCount(u32);

impl TimerInitialCount {
    pub fn get(&self) -> u32 {
        self.0
    }

    pub fn set(&mut self, inital_count: u32) {
        self.0 = inital_count
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct TimerDivideConfiguration(u32);

impl TimerDivideConfiguration {
    pub fn get(&self) -> TimerDivideConfigurationValue {
        let bit_0 = self.0.get_bit(0);
        let bit_1 = self.0.get_bit(1);
        let bit_3 = self.0.get_bit(3);
        match (bit_3, bit_1, bit_0) {
            (false, false, false) => TimerDivideConfigurationValue::Divide2,
            (false, false, true) => TimerDivideConfigurationValue::Divide4,
            (false, true, false) => TimerDivideConfigurationValue::Divide8,
            (false, true, true) => TimerDivideConfigurationValue::Divide16,
            (true, false, false) => TimerDivideConfigurationValue::Divide32,
            (true, false, true) => TimerDivideConfigurationValue::Divide64,
            (true, true, false) => TimerDivideConfigurationValue::Divide128,
            (true, true, true) => TimerDivideConfigurationValue::Divide1,
        }
    }

    pub fn set(&mut self, value: TimerDivideConfigurationValue) {
        let (bit_3, bit_1, bit_0) = match value {
            TimerDivideConfigurationValue::Divide2 => (false, false, false),
            TimerDivideConfigurationValue::Divide4 => (false, false, true),
            TimerDivideConfigurationValue::Divide8 => (false, true, false),
            TimerDivideConfigurationValue::Divide16 => (false, true, true),
            TimerDivideConfigurationValue::Divide32 => (true, false, false),
            TimerDivideConfigurationValue::Divide64 => (true, false, true),
            TimerDivideConfigurationValue::Divide128 => (true, true, false),
            TimerDivideConfigurationValue::Divide1 => (true, true, true),
        };
        self.0.set_bit(0, bit_0);
        self.0.set_bit(1, bit_1);
        self.0.set_bit(3, bit_3);
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TimerDivideConfigurationValue {
    Divide1,
    Divide2,
    Divide4,
    Divide8,
    Divide16,
    Divide32,
    Divide64,
    Divide128,
}

#[derive(Debug)]
#[repr(transparent)]
pub struct EndOfInterrupt(AtomicU32);

impl EndOfInterrupt {
    pub fn signal(&self) {
        self.0.store(0, Ordering::Release);
    }
}
