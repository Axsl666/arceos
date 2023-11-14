pub const PCIE_GEN: u32 = 2;

pub const MEGABYTE: u64 = 0x100000;
pub const GIGABYTE: u64 = 0x40000000;

// PCIe memory range (outbound)
pub const MEM_PCIE_RANGE_START: u64 = 0x600000000;
pub const MEM_PCIE_RANGE_SIZE: u64 = 0x4000000;
pub const MEM_PCIE_RANGE_PCIE_START: u64 = 0xF8000000; // mapping on PCIe side
pub const MEM_PCIE_RANGE_START_VIRTUAL: u64 = 0xFA000000;
pub const MEM_PCIE_RANGE_END_VIRTUAL: u64 = MEM_PCIE_RANGE_START_VIRTUAL + 2 * MEGABYTE - 1;

// PCIe memory range (inbound)
pub const MEM_PCIE_DMA_RANGE_START: u64 = 0;
pub const MEM_PCIE_DMA_RANGE_SIZE: u64 = 0x100000000;
pub const MEM_PCIE_DMA_RANGE_PCIE_START: u64 = 0; // mapping on PCIe side

pub const PCI_EXP_LNKCAP_SLS: u32 = 0x000_0000;