use core::convert::TryInto;

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
