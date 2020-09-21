#![no_std]
#![feature(unsafe_block_in_unsafe_fn)]
#![deny(unsafe_op_in_unsafe_fn)]

use volatile::Volatile;

pub mod io_apic;
pub mod registers;

pub struct ApicBase {
    base_addr: *mut u8,
}

impl ApicBase {
    /// base address must have 'static lifetime
    pub const unsafe fn new(base_addr: *mut ()) -> Self {
        Self {
            base_addr: base_addr.cast(),
        }
    }

    pub fn id(&mut self) -> Volatile<&mut registers::Id> {
        unsafe { self.offset(Offset::Id) }
    }

    pub fn version(&mut self) -> Volatile<&mut registers::Version> {
        unsafe { self.offset(Offset::Version) }
    }

    pub fn extended_apic_feature(&mut self) -> Volatile<&mut registers::ExtendedApicFeature> {
        unsafe { self.offset(Offset::ExtendedApicFeature) }
    }

    pub fn extended_apic_control(&mut self) -> Volatile<&mut registers::ExtendedApicControl> {
        unsafe { self.offset(Offset::ExtendedApicControl) }
    }

    pub fn spurious_interrupt_vector(
        &mut self,
    ) -> Volatile<&mut registers::SpuriousInterruptVector> {
        unsafe { self.offset(Offset::SpuriousInterruptVector) }
    }

    pub fn timer_local_vector_table_entry(
        &mut self,
    ) -> Volatile<&mut registers::TimerLocalVectorTableEntry> {
        unsafe { self.offset(Offset::TimerLocalVectorTableEntry) }
    }

    pub fn timer_initial_count(&mut self) -> Volatile<&mut registers::TimerInitialCount> {
        unsafe { self.offset(Offset::TimerInitialCount) }
    }

    pub fn timer_divide_configuration(
        &mut self,
    ) -> Volatile<&mut registers::TimerDivideConfiguration> {
        unsafe { self.offset(Offset::TimerDivideConfiguration) }
    }

    pub fn end_of_interrupt(&self) -> &'static registers::EndOfInterrupt {
        let ptr = self.offset_ptr(Offset::EndOfInterrupt).cast();
        unsafe { &*ptr }
    }

    unsafe fn offset<T>(&mut self, offset: Offset) -> Volatile<&mut T> {
        let ptr = self.offset_ptr(offset).cast();
        Volatile::new(unsafe { &mut *ptr })
    }

    fn offset_ptr(&self, offset: Offset) -> *mut u8 {
        self.base_addr.wrapping_add(offset as usize)
    }
}

#[repr(usize)]
pub enum Offset {
    Id = 0x20,
    Version = 0x30,
    TaskPriority = 0x80,
    ArbitrationPriority = 0x90,
    ProcessorPriority = 0xa0,
    EndOfInterrupt = 0xb0,
    RemoteRead = 0xc0,
    LocalDestination = 0xd0,
    DestinationFormat = 0xe0,
    SpuriousInterruptVector = 0xf0,
    InService = 0x100,
    TriggerMode = 0x180,
    InterruptRequest = 0x200,
    ErrorStatus = 0x280,
    InterruptCommand = 0x300,
    TimerLocalVectorTableEntry = 0x320,
    ThermalLocalVectorTableEntry = 0x330,
    PerformanceCounterLocalVectorTableEntry = 0x340,
    LocalInterrupt0VectorTableEntry = 0x350,
    LocalInterrupt1VectorTableEntry = 0x360,
    ErrorVectorTableEntry = 0x370,
    TimerInitialCount = 0x380,
    TimerCurrentCount = 0x390,
    TimerDivideConfiguration = 0x3e0,
    ExtendedApicFeature = 0x400,
    ExtendedApicControl = 0x410,
    SpecificEndOfInterrupt = 0x420,
    InterruptEnable = 0x480,
    ExtendedInterruptLocalVectorTable = 0x500,
}
