//! DMAC registers

use core::ptr;

/// ARM Peripheral Bus (APB) base address
pub const APB_BASE_ADDRESS: u32 = 0x4100_a000;

/// Readable DMAC registers
pub trait ReadableRegister {
    /// Type of the underlying volatile value
    type Item: Copy + Sized;

    /// Borrow the inner pointer value
    fn as_ptr(&self) -> *mut Self::Item;

    /// Perform a volatile read of the underlying register value
    fn read(&self) -> Self::Item {
        unsafe { ptr::read_volatile(self.as_ptr()) }
    }
}

/// Writable DMAC registers
pub trait WritableRegister: ReadableRegister {
    /// Perform a volatile write to the underlying register value.
    ///
    /// NOTE: Though volatile, this write is unsynchronized and therefore
    /// not truly `Sync` safe.
    fn write(&self, value: Self::Item) {
        unsafe { ptr::write_volatile(self.as_ptr(), value); }
    }
}

/// DMAC registers
macro_rules! dmac_register {
    ($const:ident, $struct:tt, $type:ty, $off:expr, $doc:expr) => {
        #[doc=$doc]
        pub struct $struct {
            ptr: *mut $type
        }

        impl $crate::dmac::register::ReadableRegister for $struct {
            type Item = $type;

            fn as_ptr(&self) -> *mut $type {
                self.ptr
            }
        }

        #[doc=$doc]
        pub const $const: &$struct = &$struct {
            ptr: ($crate::dmac::register::APB_BASE_ADDRESS + $off) as *mut $type
        };
    }
}

/// Writable DMAC registers (input/output)
macro_rules! dmac_io_register {
    ($const:ident, $struct:tt, $type:ty, $off:expr, $doc:expr) => {
        dmac_register!($const, $struct, $type, $off, $doc);
        impl $crate::dmac::register::WritableRegister for $struct {}

         impl core::ops::BitOrAssign<$type> for $struct {
            fn bitor_assign(&mut self, rhs: $type) {
                self.write(self.read() | rhs)
            }
        }


        impl core::ops::BitAndAssign<$type> for $struct {
            fn bitand_assign(&mut self, rhs: $type) {
                self.write(self.read() & rhs)
            }
        }
    }
}

// Registers common to both SAMD21 and SAMD51

dmac_io_register!(CTRL, Ctrl, u16, 0x00, "Control");
dmac_io_register!(CRCCTRL, CrcCtrl, u16, 0x02, "CRC control");
dmac_io_register!(CRCDATAIN, CrcDataIn, u32, 0x04, "CRC data input");
dmac_io_register!(CRCCHKSUM, CrcChecksum, u32, 0x08, "CRC checksum");
dmac_io_register!(CRCSTATUS, CrcStatus, u8, 0x0c, "CRC status");
dmac_io_register!(DBGCTRL, DebugCtrl, u8, 0x0d, "Debug control");
dmac_io_register!(SWTRIGCTRL, SwTrigCtrl, u32, 0x10, "Software trigger control");
dmac_io_register!(PRICTRL0, PriCtrl0, u32, 0x14, "Priority control 0");
dmac_io_register!(INTPEND, IntPending, u16, 0x20, "Interrupt pending");
dmac_register!(INTSTATUS, IntStatus, u32, 0x24, "Interrupt status");
dmac_register!(BUSYCH, BusyChannels, u32, 0x28, "Busy channels");
dmac_register!(PENDCH, PendingChannels, u32, 0x2c, "Pending channels");
dmac_register!(ACTIVE, ActiveChannel, u32, 0x30, "Active channel");
dmac_io_register!(BASEADDR, BaseAddr, u32, 0x34, "Descriptor base address");
dmac_io_register!(WRITEADDR, WriteAddr, u32, 0x38, "Write-back base address");

/// SAMD21-specific DMAC registers
#[cfg(not(feature = "samd51"))]
mod samd21 {
    /// Number of DMA channels
    pub const NUM_CHANNELS: usize = 1;

    dmac_io_register!(QOSCTRL, QosCtrl, u8, 0x0e, "QOS control");
    dmac_io_register!(CHID, ChannelId, u8, 0x3f, "Channel ID");
    dmac_io_register!(CHCTRLA, ChannelCtrlA, u8, 0x40, "Channel control (A)");
    dmac_io_register!(CHCTRLB, ChannelCtrlB, u32, 0x44, "Channel control (B)");
    dmac_io_register!(CHINTENCLR, ChannelIntEnableClr, u8, 0x4c, "Channel interrupt enable clear");
    dmac_io_register!(CHINTENSET, ChannelIntEnableSet, u8, 0x4d, "Channel interrupt enable set");
    dmac_io_register!(CHINTFLAG, ChannelIntFlag, u8, 0x4e, "Channel interrupt flag");
    dmac_register!(CHSTATUS, ChannelStatus, u8, 0x4f, "Channel status");
}

#[cfg(not(feature = "samd51"))]
pub use self::samd21::*;

/// SAMD51-specific DMAC registers
#[cfg(feature = "samd51")]
mod samd51 {
    use vcell::VolatileCell;
    use core::mem;

    /// Channel identifier
    pub type ChannelId = u8;

    /// Number of DMA channels
    pub const NUM_CHANNELS: usize = 32;

    /// DMAC channel hardware registers
    pub struct ChannelRegisters {
        /// Control (A)
        pub chctrla: VolatileCell<u32>,

        /// Control (B)
        pub chctrlb: VolatileCell<u8>,

        /// Priority level
        pub chprilvl: VolatileCell<u8>,

        /// Event control
        pub chevctrl: VolatileCell<u8>,

        /// Reserved
        _reserved1: u8,

        /// Interrupt enable clear
        pub chintenclr: VolatileCell<u8>,

        /// Interrupt enable set
        pub chintenset: VolatileCell<u8>,

        /// Interrupt flag
        pub chintflag: VolatileCell<u8>,

        /// Status
        pub chstatus: VolatileCell<u8>
    }

    /// Pointer to the channel register table
    const CHANNEL_REGISTERS_PTR: *const [ChannelRegisters; NUM_CHANNELS] =
        (super::APB_BASE_ADDRESS + 0x40) as *const [ChannelRegisters; NUM_CHANNELS];

    /// Get DMAC channel registers for a particular channel.
    ///
    /// Panics if the `ChannelId` is higher than `NUM_CHANNELS` (32).
    pub fn channel(id: ChannelId) -> &'static ChannelRegisters {
        assert!(id < NUM_CHANNELS as u8, "channel ID must be less than {}", NUM_CHANNELS);
        unsafe { mem::transmute(CHANNEL_REGISTERS_PTR.offset(id as isize)) }
    }
}


#[cfg(feature = "samd51")]
pub use self::samd51::*;
