#![no_std]
#![feature(unsafe_block_in_unsafe_fn)]
#![deny(unsafe_op_in_unsafe_fn)]

use volatile::Volatile;

pub mod registers;

pub struct ApicRegisters {
    base_addr: *mut (),
}

impl ApicRegisters {
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

    /*
        pub fn task_priority(&mut self) -> Volatile<&mut registers::TaskPriority> {
            unsafe { self.offset(Offset::TaskPriority) }
        }

        pub fn arbitration_priority(&mut self) -> Volatile<&mut registers::ArbitrationPriority> {
            unsafe { self.offset(Offset::ArbitrationPriority) }
        }

        pub fn processor_priority(&mut self) -> Volatile<&mut registers::ProcessorPriority> {
            unsafe { self.offset(Offset::ProcessorPriority) }
        }

        pub fn end_of_interrupt(&mut self) -> Volatile<&mut registers::EndOfInterrupt> {
            unsafe { self.offset(Offset::EndOfInterrupt) }
        }
    */

    unsafe fn offset<T>(&mut self, offset: Offset) -> Volatile<&mut T> {
        let ptr = self.base_addr.wrapping_add(offset as usize).cast();
        Volatile::new(unsafe { &mut *ptr })
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
