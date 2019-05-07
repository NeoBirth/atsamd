//! DMA controller (DMAC) support

pub mod register;

/// DMAC descriptor registers
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(align(16))]
pub struct Descriptor {
    /// Block transfer control
    pub btctrl: u16,

    /// Block transfer count
    pub btcnt: u16,

    /// Source address for transfer
    pub srcaddr: u32,

    /// Destination address for transfer
    pub dstaddr: u32,

    /// Address of next descriptor
    pub descaddr: u32,
}
