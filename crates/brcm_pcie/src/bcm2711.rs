use core::{marker::PhantomData, ptr::NonNull};

use tock_registers::{
    interfaces::{ReadWriteable, Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

use crate::consts::*;
use crate::BCM2711Hal;

register_bitfields![
    u16,
    // 0x00dc
    PCI_EXP_LNKCTL2 [
        LNKCTL OFFSET(0) NUMBITS(16) [],
    ],
];

register_bitfields![
    u32,
    // /* BRCM_PCIE_CAP_REGS - Offset for the mandatory capability config regs */
    // 0x00ac
    //BRCM_PCIE_CAP_REGS [],

    // 0X00b8 0x0b0ac + 12
    PCI_EXP_LNKCAP [
        LNKCAP OFFSET(0) NUMBITS(32) [],
    ],
    //
    //  Broadcom STB PCIe Register Offsets
    // 0x0188
    RC_CFG_VENDOR_VENDOR_SPECIFIC_REG1 [
        LITTLE_ENDIAN OFFSET(0) NUMBITS(1) [],
        ENDIAN_MODE_BAR2 OFFSET(2) NUMBITS(2) [],
    ],

    // 0x043c
    RC_CFG_PRIV1_ID_VAL3 [
        CLASS_ID  OFFSET(0) NUMBITS(24) [
            pcie_pcie_bridge = 0x060400
        ],
    ],
    // 0x04dc
    RC_CFG_PRIV1_LINK_CAPABILITY [
        BIT10_11 OFFSET(10) NUMBITS(2)[]
    ],


    // 0x1100
    RC_DL_MDIO_ADDR [
        ADDR OFFSET(0) NUMBITS(32) [
            // SET_ADDR_OFFSET to be written
            set_addr_offset = 0x1f,
            // SSC_CNTL_OFFSET to be read
            ssc_cntl_offset = 0x100002
        ]
    ],

    // 0x1104
    RC_DL_MDIO_WR_DATA [
        DATA OFFSET(0) NUMBITS(32) [
            init_val = 0x80001100
        ]
    ],
    // 0x1108
    // RC_DL_MDIO_RD_DATA

    // 0x4008
    MISC_MISC_CTRL [
        SCB2_SIZE OFFSET(0) NUMBITS(5) [],
        SCB_ACCESS_EN OFFSET(12) NUMBITS(1) [],
        CFG_READ_UR_MODE OFFSET(13) NUMBITS(1) [],
        MAX_BURST_SIZE OFFSET(20) NUMBITS(2) [],
        SCB1_SIZE OFFSET(22) NUMBITS(5) [],
        SCB0_SIZE OFFSET(27) NUMBITS(5) [
            init_val = 0b10001,
        ],
    ],
    // 0x400c
    MISC_CPU_2_PCIE_MEM_WIN0_LO [
        MEM_WIN0_LO OFFSET(0) NUMBITS(32) [
            // TODO
            init_val = 0xc000_0000
        ],
    ],
    // 0x4010
    MISC_CPU_2_PCIE_MEM_WIN0_HI [
        MEM_WIN0_HI OFFSET(0) NUMBITS(32) [
            init_val = 0x0000_0000
        ],
    ],

    // 0x4204


    // 0x402C
    MISC_RC_BAR1_CONFIG_LO [
        MEM_WIN OFFSET(0) NUMBITS(5)[]
    ],

    // 0x4034
    MISC_RC_BAR2_CONFIG_LO [
        VALUE_LO OFFSET(0) NUMBITS(32)[
            init_val = 0x11,
        ]
    ],
    // 0x4038
    MISC_RC_BAR2_CONFIG_HI [
        VALUE_HI OFFSET(0) NUMBITS(32)[
            init_val = 0x4,
        ]
    ],
    // 0x403C
    MISC_RC_BAR3_CONFIG_LO [
        MEM_WIN OFFSET(0) NUMBITS(5)[]
    ],

    // 0x4044
    MISC_MSI_BAR_CONFIG_LO [
        LO OFFSET(0) NUMBITS(32) [
            // lower 32 bits of msi target address with bit 0 set => msi enable
            init_val = 0xffff_fffd
        ]
    ],
    //0x4048
    MISC_MSI_BAR_CONFIG_HI [
        HI OFFSET(0) NUMBITS(32) [
            init_val = 0x0
        ]
    ],
    // 0x404c
    MISC_MSI_DATA_CONFIG [
        DATA_CONFIG OFFSET(0) NUMBITS(32) [
            /*
            * ffe0 -- least sig 5 bits are 0 indicating 32 msgs
            * 6540 -- this is our arbitrary unique data value
            */
            init_val = 0xffe06540
        ]
    ],
    // 0x4060
    // MISC_EOI_CTRL
    // 0x4064
    // MISC_PCIE_CTRL
    // 0x4068
    MISC_PCIE_STATUS [
        CHECK_BITS OFFSET(4) NUMBITS(2)[],
        RC_MODE OFFSET(7) NUMBITS(1)[],
    ],
    // 0x406c
    MISC_REVISION [
        MAJMIN OFFSET(0) NUMBITS(16) [],
    ],

    // 0x4070
    MISC_CPU_2_PCIE_MEM_WIN0_BASE_LIMIT [
        BIT4_15 OFFSET(4) NUMBITS(12)[],
        BIT20_29 OFFSET(20) NUMBITS(10)[],
        BIT30_31 OFFSET(30) NUMBITS(2)[],
    ],
    // // 0x4070
    // MISC_CPU_2_PCIE_MEM_WIN0_BASE_LIMIT [
    //     MEM_WIN0_BASE_LIMIT OFFSET(0) NUMBITS(32)[
    //         // TODO
    //         init_val = 0x3ff00000
    //     ]
    // ],
    // 0x4080
    MISC_CPU_2_PCIE_MEM_WIN0_BASE_HI [
        MEM_WIN0_BASE_HI OFFSET(0) NUMBITS(32)[
            init_val = 0x6
        ]
    ],
    // 0x4084
    MISC_CPU_2_PCIE_MEM_WIN0_LIMIT_HI [
        MEM_WIN0_LIMIT_HI OFFSET(0) NUMBITS(32)[
            init_val = 0x6
        ]
    ],
    // 0x4204
    MISC_HARD_PCIE_HARD_DEBUG [
        CLKREQ_DEBUG_ENABLE OFFSET(1) NUMBITS(1) [],
        CLKREQ_L1SS_ENABLE OFFSET(21) NUMBITS(1) [],
        SERDES_IDDQ OFFSET(27) NUMBITS(1) [],
    ],

    // 0x4300 INTR2_CPU_BASE
    INTR2_CPU_STATUS [
        INTR_STATUS OFFSET(0) NUMBITS(32) [],
    ],
    // 0x4304 0x4300 + 0x4
    INTR2_CPU_SET [
        INTR_SET OFFSET(0) NUMBITS(32) [],
    ],
    // 0x4308 0x4300 + 0x8
    INTR2_CPU_CLR [
        INTR_CLR OFFSET(0) NUMBITS(32) []
    ],
    // 0x430c 0x4300 + 0x0c
    INTR2_CPU_MASK_STATUS [
        INTR_MASK_STATUS OFFSET(0) NUMBITS(32) []
    ],
    // 0x4310 0x4300 + 0x10
    INTR2_CPU_MASK_SET [
        INTR_MASK_SET OFFSET(0) NUMBITS(32) []
    ],
    // 0x4314 0x4500 + 0x14
    INTR2_CPU_MASK_CLR [
        INTR_MASK_CLR OFFSET(0) NUMBITS(32) []
    ],
    // 0x4500 MSI_INTR2_BASE
    MSI_INTR2_STATUS [
        INTR_STATUS OFFSET(0) NUMBITS(32) [],
    ],
    // 0x4504 0x4500 + 0x4
    MSI_INTR2_SET [
        INTR_SET OFFSET(0) NUMBITS(32) [],
    ],
    // 0x4508 0x4500 + 0x8
    MSI_INTR2_CLR [
        INTR_CLR OFFSET(0) NUMBITS(32) []
    ],
    // 0x450c 0x4500 + 0x0c
    MSI_INTR2_MASK_STATUS [
        INTR_MASK_STATUS OFFSET(0) NUMBITS(32) []
    ],
    // 0x4510 0x4500 + 0x10
    MSI_INTR2_MASK_SET [
        INTR_MASK_SET OFFSET(0) NUMBITS(32) []
    ],
    // 0x4514 0x4500 + 0x14
    MSI_INTR2_MASK_CLR [
        INTR_MASK_CLR OFFSET(0) NUMBITS(32) []
    ],


    // 0x8000
    // EXT_CFG_DATA
    // 0x9000
    // EXT_CFG_INDEX

    // 0x9210
    RGR1_SW_INIT_1 [
        PCIE_RGR1_SW_INTI_1_PERST OFFSET(0) NUMBITS(1) [],
        RGR1_SW_INTI_1_GENERIC OFFSET(1) NUMBITS(1) [],
    ],

];

register_structs! {
    /// Pl011 registers.
    BCM2711PCIeHostBridgeRegs {
        (0x00 => _rsvd1),
        (0xb8 => pci_exp_lnkcap: ReadWrite<u32,PCI_EXP_LNKCAP::Register>),
        (0xbc => hole01),
        (0xbe => cap: ReadWrite<u16>),
        (0xc0 => hole11),
        (0xdc => pci_exp_lnkctl2: ReadWrite<u16,PCI_EXP_LNKCTL2::Register>),
        (0xde => hole13),
        (0x0188 => rc_cfg_vendor_vendor_specific_reg1: ReadWrite<u32,RC_CFG_VENDOR_VENDOR_SPECIFIC_REG1::Register>),
        (0x018c => hole12),
        (0x043c => rc_cfg_priv1_id_val3: ReadWrite<u32,RC_CFG_PRIV1_ID_VAL3::Register>),
        (0x0440 => _rsvdd2),
        (0x04dc => rc_cfg_priv1_link_capability: ReadWrite<u32, RC_CFG_PRIV1_LINK_CAPABILITY::Register>),
        (0x04e0 => hole1),
        (0x1100 => rc_dl_mdio_addr),
        (0x1104 => rc_dl_mdio_wr_data),
        (0x1108 => rc_dl_mdio_rd_data),
        (0x4008 => misc_misc_ctrl: ReadWrite<u32, MISC_MISC_CTRL::Register>),
        (0x400C => misc_cpu_2_pcie_mem_win0_lo: ReadWrite<u32,MISC_CPU_2_PCIE_MEM_WIN0_LO::Register>),
        (0x4010 => misc_cpu_2_pcie_mem_win0_hi: ReadWrite<u32,MISC_CPU_2_PCIE_MEM_WIN0_HI::Register>),
        (0x4014 => _rsvd22),
        (0x4028 => _rsvd2),
        (0x402C => misc_rc_bar1_config_lo: ReadWrite<u32,MISC_RC_BAR1_CONFIG_LO::Register>),
        (0x4030 => _rsvdd),
        (0x4034 => misc_rc_bar2_config_lo: ReadWrite<u32,MISC_RC_BAR2_CONFIG_LO::Register>),
        (0x4038 => misc_rc_bar2_config_hi: ReadWrite<u32,MISC_RC_BAR2_CONFIG_HI::Register>),
        (0x403C => misc_rc_bar3_config_lo: ReadWrite<u32,MISC_RC_BAR3_CONFIG_LO::Register>),
        (0x4040 => _rsvddd),
        (0x4044 => misc_msi_bar_config_lo: ReadWrite<u32,MISC_MSI_BAR_CONFIG_LO::Register>),
        (0x4048 => misc_msi_bar_config_hi: ReadWrite<u32,MISC_MSI_BAR_CONFIG_HI::Register>),
        (0x404c => misc_msi_data_config: ReadWrite<u32,MISC_MSI_DATA_CONFIG::Register>	),
        (0x4050 => hole33),
        (0x4060 => misc_eoi_ctrl),
        (0x4064 => misc_pcie_ctrl),
        (0x4068 => misc_pcie_status: ReadOnly<u32,MISC_PCIE_STATUS::Register>),
        (0x406C => misc_revision: ReadWrite<u32,MISC_REVISION::Register>),
        (0x4070 => misc_cpu_2_pcie_mem_win0_base_limit: ReadWrite<u32, MISC_CPU_2_PCIE_MEM_WIN0_BASE_LIMIT::Register>),
        (0x4074 => hole),
        (0x4080 => misc_cpu_2_pcie_mem_win0_base_hi: ReadWrite<u32,MISC_CPU_2_PCIE_MEM_WIN0_BASE_HI::Register>),
        (0x4084 => misc_cpu_2_pcie_mem_win0_limit_hi: ReadWrite<u32,MISC_CPU_2_PCIE_MEM_WIN0_LIMIT_HI::Register>),
        (0x4088 => hole2),
        (0x4204 => misc_hard_pcie_hard_debug: ReadWrite<u32,MISC_HARD_PCIE_HARD_DEBUG::Register>),
        (0x4208 => _rsvd3),
        /// cpu intr
        (0x4300 => intr2_cpu_status:        ReadWrite<u32,INTR2_CPU_STATUS::Register>),
        (0x4304 => intr2_cpu_set:           ReadWrite<u32,INTR2_CPU_SET::Register>),
        (0x4308 => intr2_cpu_clr:           ReadWrite<u32,INTR2_CPU_CLR::Register>),
        (0x430C => intr2_cpu_mask_status:   ReadWrite<u32,INTR2_CPU_MASK_STATUS::Register>),
        (0x4310 => intr2_cpu_mask_set:      ReadWrite<u32,INTR2_CPU_MASK_SET::Register>),
        (0x4314 => intr2_cpu_mask_clr:      ReadWrite<u32,INTR2_CPU_MASK_CLR::Register>),
        (0x4318 => hole3),
        /// msi intr
        (0x4500 => msi_intr2_status:        ReadWrite<u32,MSI_INTR2_STATUS::Register>),
        (0x4504 => msi_intr2_set:           ReadWrite<u32,MSI_INTR2_SET::Register>),
        (0x4508 => msi_intr2_clr:           ReadWrite<u32,MSI_INTR2_CLR::Register>),
        (0x450C => msi_intr2_mask_status:   ReadWrite<u32,MSI_INTR2_MASK_STATUS::Register>),
        (0x4510 => msi_intr2_mask_set:      ReadWrite<u32,MSI_INTR2_MASK_SET::Register>),
        (0x4514 => msi_intr2_mask_clr:      ReadWrite<u32,MSI_INTR2_MASK_CLR::Register>),
        (0x4518 => hole4),
        /// Interrupt Clear Register.
        (0x9210 => rgr1_sw_init: ReadWrite<u32,RGR1_SW_INIT_1::Register>),
        (0x9214 => _rsvd4),
        (0x9310 => @END),
    }
}

struct PCIeMemoryWindow {
    pcie_addr: u64,
    cpu_addr: u64,
    size: u64,
}

type PCIeMSIHandler = fn(usize, usize);

struct PCIeMSIData {
    base: usize,
    target_addr: u64,
    intr_base: usize,
    handler: PCIeMSIHandler,
    param_ptr: usize,
}

pub struct BCM2711PCIeHostBridge<H: BCM2711Hal> {
    /// PCIe MMIO base address
    base: NonNull<BCM2711PCIeHostBridgeRegs>,
    /// Controller revision
    rev: u32,
    /// Outbound window
    out_wins: Option<PCIeMemoryWindow>,
    /// inbound winodws
    dma_ranges: Option<PCIeMemoryWindow>, //  Vec<PCIeMemoryWindow>
    scb_size: Option<u64>,
    //
    _phantom: PhantomData<H>,
}

unsafe impl<H: BCM2711Hal> Send for BCM2711PCIeHostBridge<H> {}
unsafe impl<H: BCM2711Hal> Sync for BCM2711PCIeHostBridge<H> {}

impl<H: BCM2711Hal> BCM2711PCIeHostBridge<H> {
    /// Constrcut a new BCM2711 PCIe host bridge instance from the base address.
    pub const fn new(base: usize) -> Self {
        Self {
            base: NonNull::new(base as *mut u8).unwrap().cast(),
            rev: 0,
            out_wins: None,
            dma_ranges: None,
            scb_size: None,
            _phantom: PhantomData,
        }
    }

    pub fn init(&mut self)   {
        self.pcie_probe();
        self.enable_bridge();

    }

    pub fn enable_device(&self) {}

    fn pcie_probe(&mut self) {
        // TODO? DMA target addresses >= 0xC0000000 may need bounce support
        self.set_pci_ranges();
        self.set_dma_ranges();
        self.setup();
    }

    fn enable_bridge(&self) {
        // TODO
    }

    const fn regs(&self) -> &BCM2711PCIeHostBridgeRegs {
        unsafe { self.base.as_ref() }
    }

    fn bridge_sw_init_set(&self, val: u32) {
        self.regs()
            .rgr1_sw_init
            .modify(RGR1_SW_INIT_1::RGR1_SW_INTI_1_GENERIC.val(val))
    }

    fn perst_set(&self, val: u32) {
        self.regs()
            .rgr1_sw_init
            .modify(RGR1_SW_INIT_1::PCIE_RGR1_SW_INTI_1_PERST.val(val))
    }

    fn pcie_link_up(&self) -> bool {
        self.regs()
            .misc_pcie_status
            .read(MISC_PCIE_STATUS::CHECK_BITS)
            == 0x3
    }

    /// The controller is capable of serving in both RC and EP roles
    fn is_rc_mode(&self) -> bool {
        self.regs().misc_pcie_status.read(MISC_PCIE_STATUS::RC_MODE) == 0x1
    }

    fn enable_msi(&self) {

    }

    // TODO: msidata
    fn msi_set_regs(&self) {
        // assert!(self.rev >= BRCM_PCIE_HW_REV_33) // BRCM_PCIE_HW_REV_33 = 0x0303
        /*
         * Make sure we are not masking MSIs. Note that MSIs can be masked,
         * but that occurs on the PCIe EP device
         */
        let regs = self.regs();

        /*
         * The 0 bit of PCIE_MISC_MSI_BAR_CONFIG_LO is repurposed to MSI
         * enable, which we set to 1.
         */
        regs.msi_intr2_mask_clr
            .write(MSI_INTR2_MASK_CLR::INTR_MASK_CLR::SET);
        regs.misc_msi_bar_config_lo
            .write(MISC_MSI_BAR_CONFIG_LO::LO::init_val);
        regs.misc_msi_bar_config_hi
            .write(MISC_MSI_BAR_CONFIG_HI::HI::init_val);
        /*
         * ffe0 -- least sig 5 bits are 0 indicating 32 msgs
         * 6540 -- this is our arbitrary unique data value
         */
        regs.misc_msi_data_config
            .write(MISC_MSI_DATA_CONFIG::DATA_CONFIG::init_val);
    }

    fn set_pci_ranges(&mut self) {
        assert!(self.out_wins.is_none());
        self.out_wins = Some(PCIeMemoryWindow {
            pcie_addr: MEM_PCIE_RANGE_PCIE_START,
            cpu_addr: MEM_PCIE_RANGE_START,
            size: MEM_PCIE_RANGE_SIZE,
        })
    }
    fn set_dma_ranges(&mut self) {
        assert!(self.dma_ranges.is_none());
        self.out_wins = Some(PCIeMemoryWindow {
            pcie_addr: MEM_PCIE_DMA_RANGE_PCIE_START,
            cpu_addr: MEM_PCIE_DMA_RANGE_START,
            size: MEM_PCIE_DMA_RANGE_SIZE,
        })
    }

    // this function is simplified
    fn set_outbound_win(&self) {
        // Set the m_base of the pcie_addr window
        self.regs()
            .misc_cpu_2_pcie_mem_win0_lo
            .write(MISC_CPU_2_PCIE_MEM_WIN0_LO::MEM_WIN0_LO::init_val);
        self.regs()
            .misc_cpu_2_pcie_mem_win0_hi
            .write(MISC_CPU_2_PCIE_MEM_WIN0_HI::MEM_WIN0_HI::init_val);

        //  Write the addr m_base low register
        self.regs().misc_cpu_2_pcie_mem_win0_base_limit.modify(
            MISC_CPU_2_PCIE_MEM_WIN0_BASE_LIMIT::BIT30_31::CLEAR
                + MISC_CPU_2_PCIE_MEM_WIN0_BASE_LIMIT::BIT20_29::SET
                + MISC_CPU_2_PCIE_MEM_WIN0_BASE_LIMIT::BIT4_15::CLEAR,
        );
        // Write the addr limit low register

        // Write the cpu addr high register
        self.regs()
            .misc_cpu_2_pcie_mem_win0_base_hi
            .write(MISC_CPU_2_PCIE_MEM_WIN0_BASE_HI::MEM_WIN0_BASE_HI::init_val);
        // Write the cpu limit high register
        self.regs()
            .misc_cpu_2_pcie_mem_win0_limit_hi
            .write(MISC_CPU_2_PCIE_MEM_WIN0_LIMIT_HI::MEM_WIN0_LIMIT_HI::init_val);
    }

    fn set_gen(&self, gen: u32) {
        let mut lnkcap = self.regs().pci_exp_lnkcap.read(PCI_EXP_LNKCAP::LNKCAP);
        let mut lnkctl2 = self.regs().pci_exp_lnkctl2.read(PCI_EXP_LNKCTL2::LNKCTL);

        lnkcap = (lnkcap & !PCI_EXP_LNKCAP_SLS) | gen;
        lnkctl2 = (lnkctl2 & !0xf) | gen as u16;

        self.regs()
            .pci_exp_lnkcap
            .write(PCI_EXP_LNKCAP::LNKCAP.val(lnkcap));
        self.regs()
            .pci_exp_lnkctl2
            .write(PCI_EXP_LNKCTL2::LNKCTL.val(lnkctl2));
    }

    pub fn setup(&mut self) {
        // Reset the bridge
        self.bridge_sw_init_set(1);
        // Ensure that the fundamental reset is asserted
        self.perst_set(1);

        H::sleep(core::time::Duration::from_micros(2));

        // Take the bridge out of reset
        self.bridge_sw_init_set(0);

        H::sleep(core::time::Duration::from_micros(2));

        // Enable SerDes
        self.regs()
            .misc_hard_pcie_hard_debug
            .modify(MISC_HARD_PCIE_HARD_DEBUG::SERDES_IDDQ::CLEAR);

        // Wait for SerDes to be stable
        H::sleep(core::time::Duration::from_micros(2));

        // Get the PCIe hardware revision number
        self.rev = 1;

        // Set SCB_MAX_BURST_SIZE , CFG_READ_UR_MODE, SCB_ACCESS_EN
        self.regs()
            .misc_misc_ctrl
            .modify(MISC_MISC_CTRL::SCB_ACCESS_EN::SET);
        self.regs()
            .misc_misc_ctrl
            .modify(MISC_MISC_CTRL::CFG_READ_UR_MODE::SET);
        self.regs()
            .misc_misc_ctrl
            .modify(MISC_MISC_CTRL::MAX_BURST_SIZE::CLEAR);

        /*
         * Set up inbound memory view for the EP (called RC_BAR2,
         * not to be confused with the BARs that are advertised by
         * the EP).
         *
         * The PCIe host controller by design must set the inbound
         * viewport to be a contiguous arrangement of all of the
         * system's memory.  In addition, its size mut be a power of
         * two.  Further, the MSI target address must NOT be placed
         * inside this region, as the decoding logic will consider its
         * address to be inbound memory traffic.  To further
         * complicate matters, the viewport must start on a
         * pcie-address that is aligned on a multiple of its size.
         * If a portion of the viewport does not represent system
         * memory -- e.g. 3GB of memory requires a 4GB viewport --
         * we can map the outbound memory in or after 3GB and even
         * though the viewport will overlap the outbound memory
         * the controller will know to send outbound memory downstream
         * and everything else upstream.
         */

        assert!(self.dma_ranges.is_some());
        self.scb_size = Some(self.dma_ranges.as_ref().unwrap().size); // must be a power of 2
        let rc_bar2_offset = self.dma_ranges.as_ref().unwrap().pcie_addr;

        assert!(self.scb_size.is_some());
        let total_mem_size = self.scb_size.clone().unwrap();
        let rc_bar2_size = total_mem_size; // must be a power of 2

        /* Verify the alignment is correct */
        assert!((rc_bar2_offset & (rc_bar2_size - 1)) != 0);

        /*
         * Position the MSI target low if possible.
         *
         * TODO: Consider outbound window when choosing MSI target and
         * verifying configuration.
         */

        // rc_bar2_offset (todo)
        // setup inbound memory view
        self.regs()
            .misc_rc_bar2_config_lo
            .write(MISC_RC_BAR2_CONFIG_LO::VALUE_LO::init_val);
        self.regs()
            .misc_rc_bar2_config_hi
            .write(MISC_RC_BAR2_CONFIG_HI::VALUE_HI::init_val);
        // set bits 27-31 of [0xfd504008] to 0b10001 (PCIE_MISC_MISC_CTRL)
        // scb_size[0] ? ilog2(scb_size) - 15 : 0xf;
        self.regs()
            .misc_misc_ctrl
            .modify(MISC_MISC_CTRL::SCB0_SIZE::init_val);

        // disable the PCIe->GISB memory window (RC_BAR1)
        self.regs()
            .misc_rc_bar1_config_lo
            .modify(MISC_RC_BAR1_CONFIG_LO::MEM_WIN::CLEAR);
        // disable the PCIe->SCB memory window (RC_BAR3)
        self.regs()
            .misc_rc_bar3_config_lo
            .modify(MISC_RC_BAR3_CONFIG_LO::MEM_WIN::CLEAR);

        // clear any interrupts we find on boot
        self.regs()
            .intr2_cpu_clr
            .write(INTR2_CPU_CLR::INTR_CLR::SET);
        // Mask all interrupts since we are not handling any yet
        self.regs()
            .intr2_cpu_mask_set
            .write(INTR2_CPU_MASK_SET::INTR_MASK_SET::SET);

        // set gen
        self.set_gen(PCIE_GEN);
        // Unassert the fundamental reset
        self.perst_set(0);

        /*
         * Give the RC/EP time to wake up, before trying to configure RC.
         * Intermittently check status for link-up, up to a total of 100ms
         * when we don't know if the device is there, and up to 1000ms if
         * we do know the device is there.
         */
        for _ in 0..100 {
            if self.pcie_link_up() {
                break;
            }
            H::sleep(core::time::Duration::from_micros(5));
        }

        if self.pcie_link_up() {
            panic!("PCIe link is down");
        }

        // check if controller is running in root complex mode. if bit 7 is not set, and error
        if self.is_rc_mode() {
            panic!("PCIe controller is not running in root complex mode");
        }
        log::debug!("PCIe controller is running in root complex mode");

        self.set_outbound_win();

        /*
         * For config space accesses on the RC, show the right class for
         * a PCIe-PCIe bridge (the default setting is to be EP mode).
         */
        // set proper class Id
        self.regs()
            .rc_cfg_priv1_id_val3
            .modify(RC_CFG_PRIV1_ID_VAL3::CLASS_ID::pcie_pcie_bridge);
        self.regs()
            .rc_cfg_priv1_link_capability
            .modify(RC_CFG_PRIV1_LINK_CAPABILITY::BIT10_11::SET);

        let lnksta = self.regs().cap.get();
        let cls = lnksta & 0x000f;
        let nlw = (lnksta & 0x03f0) >> 4;
        log::debug!("PCIe link up, {} Gbps x{}", cls, nlw);

        // PCIe->SCB endian mode for BAR
        // field ENDIAN_MODE_BAR2 = DATA_ENDIAN
        self.regs()
            .rc_cfg_vendor_vendor_specific_reg1
            .modify(RC_CFG_VENDOR_VENDOR_SPECIFIC_REG1::ENDIAN_MODE_BAR2::CLEAR);

        // Enable SSC (spread spectrum clocking) steps
        {
            // write SET_ADDR_OFFSET to register
        }

        /*
         * Refclk from RC should be gated with CLKREQ# input when ASPM L0s,L1
         * is enabled =>  setting the CLKREQ_DEBUG_ENABLE field to 1.
         */
        self.regs().misc_hard_pcie_hard_debug.modify(
            MISC_HARD_PCIE_HARD_DEBUG::CLKREQ_DEBUG_ENABLE::SET,
            // + MISC_HARD_PCIE_HARD_DEBUG::CLKREQ_L1SS_ENABLE::CLEAR,
        );

    }
}
