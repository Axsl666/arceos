
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
use tock_registers::{register_bitfields, register_structs};


// Register fields and definitions look like this:
register_bitfields![u32,
    // creg_mio_func_sel 
    IC_CON [
        MASTER_MODE  OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],
        SPEED OFFSET(1) NUMBITS(2) [
            Standard = 1,
            Fast = 2,
            FastPlus = 3,
        ],
        IC_10BITADDR_SLAVE  OFFSET(3) NUMBITS(1) [
            Mode7Bit = 0,
            Mode10Bit= 1,
        ],
        IC_10BITADDR_MASTER OFFSET(4) NUMBITS(1) [
            Mode7Bit = 0,
            Mode10Bit = 1,
        ],
        IC_RESTART_EN OFFSET(5) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],
        IC_SLAVE_DISABLE OFFSET(6) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ]
    ],

    IC_TAR [
        IC_TAR OFFSET(0) NUMBITS(10) [],
        GC_OR_START OFFSET(10) NUMBITS(1) [

        ],
        SPECIAL OFFSET(11) NUMBITS(1) [
            UseTar = 0,
            UseGcOrStart = 1,
        ],
        IC_10BITADDR_MASTER OFFSET(12) NUMBITS(1) [
            Mode7Bit = 0,
            Mode10Bit = 1,
        ]
    ],
    IC_SAR [
        IC_SAR OFFSET(0) NUMBITS(10) []
    ],
    IC_HS_MADDR [
        IC_HS_MADDR OFFSET(0) NUMBITS(3) []
    ],
    IC_DATA_CMD [
        DAT OFFSET(0) NUMBITS(8) [],
        CMD OFFSET(8) NUMBITS(1) [],
        STOP OFFSET(9) NUMBITS(1) [],
        RESTART OFFSET(10) NUMBITS(1)[]
    ],
    IC_SS_SCL_HCNT [
        IC_SS_SCL_HCNT OFFSET(0) NUMBITS(16) [],
    ],
    IC_SS_SCL_LCNT [
        IC_SS_SCL_LCNT OFFSET(0) NUMBITS(16) [],
    ],
    IC_FS_SCL_HCNT [
        IC_FS_SCL_HCNT OFFSET(0) NUMBITS(16) [],
    ],
    IC_FS_SCL_LCNT [
        IC_FS_SCL_LCNT OFFSET(0) NUMBITS(16) [],
    ],
    IC_HS_SCL_HCNT [
        IC_HS_SCL_HCNT OFFSET(0) NUMBITS(16) [],
    ],
    IC_HS_SCL_LCNT [
        IC_HS_SCL_LCNT OFFSET(0) NUMBITS(16) [],
    ],
    IC_INTR_STAT [
        R_RX_UNDER OFFSET(0) NUMBITS(1) [],
        R_RX_OVER OFFSET(1) NUMBITS(1) [],
        R_RX_FULL OFFSET(2) NUMBITS(1) [],
        R_TX_OVER OFFSET(3) NUMBITS(1) [],
        R_TX_EMPTY OFFSET(4) NUMBITS(1) [],
        R_RD_REQ OFFSET(5) NUMBITS(1) [],
        R_TX_ABRT OFFSET(6) NUMBITS(1) [],
        R_RX_DONE OFFSET(7) NUMBITS(1) [],
        R_ACTIVITY OFFSET(8) NUMBITS(1) [],
        R_STOP_DET OFFSET(9) NUMBITS(1) [],
        R_START_DET OFFSET(10) NUMBITS(1) [],
        R_GEN_CALL OFFSET(11) NUMBITS(1) [],
    ],
    IC_INTR_MASK [
        M_RX_UNDER OFFSET(0) NUMBITS(1) [],
        M_RX_OVER OFFSET(1) NUMBITS(1) [],
        M_RX_FULL OFFSET(2) NUMBITS(1) [],
        M_TX_OVER OFFSET(3) NUMBITS(1) [],
        M_TX_EMPTY OFFSET(4) NUMBITS(1) [],
        M_RD_REQ OFFSET(5) NUMBITS(1) [],
        M_TX_ABRT OFFSET(6) NUMBITS(1) [],
        M_RX_DONE OFFSET(7) NUMBITS(1) [],
        M_ACTIVITY OFFSET(8) NUMBITS(1) [],
        M_STOP_DET OFFSET(9) NUMBITS(1) [],
        M_START_DET OFFSET(10) NUMBITS(1) [],
        M_GEN_CALL OFFSET(11) NUMBITS(1) [],
    ],
    IC_RAW_INTR_STAT [
        RX_UNDER OFFSET(0) NUMBITS(1) [],
        RX_OVER OFFSET(1) NUMBITS(1) [],
        RX_FULL OFFSET(2) NUMBITS(1) [],
        TX_OVER OFFSET(3) NUMBITS(1) [],
        TX_EMPTY OFFSET(4) NUMBITS(1) [],
        RD_REQ OFFSET(5) NUMBITS(1) [],
        TX_ABRT OFFSET(6) NUMBITS(1) [],
        RX_DONE OFFSET(7) NUMBITS(1) [],
        ACTIVITY OFFSET(8) NUMBITS(1) [],
        STOP_DET OFFSET(9) NUMBITS(1) [],
        START_DET OFFSET(10) NUMBITS(1) [],
        GEN_CALL OFFSET(11) NUMBITS(1) [],
    ],
    IC_RX_TL [
        RX_TL OFFSET(0) NUMBITS(7) [],
    ],
    IC_TX_TL [
        TX_TL OFFSET(0) NUMBITS(7) [],
    ],
    IC_CLR_INTR [
        CLR_INTR OFFSET(0) NUMBITS(1) [],
    ],
    IC_CLR_RX_UNDER [
        CLR_RX_UNDER OFFSET(0) NUMBITS(1) [],
    ],
    IC_CLR_RX_OVER [
        CLR_RX_OVER OFFSET(0) NUMBITS(1) [],
    ],
    IC_CLR_TX_OVER [
        CLR_TX_OVER OFFSET(0) NUMBITS(1) [],
    ],
    IC_CLR_RD_REQ [
        CLR_RD_REQ OFFSET(0) NUMBITS(1) [],
    ],
    IC_CLR_TX_ABRT [
        CLR_TX_ABRT OFFSET(0) NUMBITS(1) [],
    ],
    IC_CLR_RX_DONE [
        CLR_RX_DONE OFFSET(0) NUMBITS(1) [],
    ],
    IC_CLR_ACTIVITY [
        CLR_ACTIVITY OFFSET(0) NUMBITS(1) [],
    ],
    IC_CLR_STOP_DEL [
        CLR_STOP_DEL OFFSET(0) NUMBITS(1) [],
    ],
    IC_CLR_START_DEL [
        CLR_START_DEL OFFSET(0) NUMBITS(1) [],
    ],
    IC_CLR_GEN_CALL [
        CLR_GEN_CALL OFFSET(0) NUMBITS(1) [],
    ],
    IC_ENABLE [
        ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    IC_STATUS [
        ACTIVITY OFFSET(0) NUMBITS(1) [],
        TFNF OFFSET(1) NUMBITS(1) [],
        TFE OFFSET(2) NUMBITS(1) [],
        RFNE OFFSET(3) NUMBITS(1) [],
        RFF OFFSET(4) NUMBITS(1) [],
        MST_ACTIVITY OFFSET(5) NUMBITS(1) [],
        SLV_ACTIVITY OFFSET(6) NUMBITS(1) [],
    ],
    IC_TXFLR [
        TXFLR OFFSET(0) NUMBITS(2) [],
    ],
    IC_RXFLR [
        RXFLR OFFSET(0) NUMBITS(2) [],
    ],
    IC_SDA_HOLD [
        IC_SDA_HOLD OFFSET(0) NUMBITS(16) [],
    ],
    IC_TX_ABRT_SOURCE [
        ABRT_7B_ADDR_NOACK OFFSET(0) NUMBITS(1) [],
        ABRT_10ADDR1_NOACK_NOACK OFFSET(1) NUMBITS(1) [],
        ABRT_10ADDR2_NOACK_NOACK OFFSET(2) NUMBITS(1) [],
        ABRT_TXDATA_NOACK OFFSET(3) NUMBITS(1) [],
        ABRT_GCALL_NOACK OFFSET(4) NUMBITS(1) [],
        ABRT_GCALL_READ OFFSET(5) NUMBITS(1) [],
        ABRT_HS_ACKDET OFFSET(6) NUMBITS(1) [],
        ABRT_SBYTE_ACKDET OFFSET(7) NUMBITS(1) [],
        ABRT_HS_NOSTART OFFSET(8) NUMBITS(1) [],
        ABRT_SBYTE_NOSTART OFFSET(9) NUMBITS(1) [],
        ABRT_10B_RD_NOSTART OFFSET(10) NUMBITS(1) [],
        ABRT_MASTER_DIS OFFSET(11) NUMBITS(1) [],
        ABRT_LOST OFFSET(12) NUMBITS(1) [],
        ABRT_SLVFLUSH_TXFIFO OFFSET(13) NUMBITS(1) [],
        ABRT_SLV_ARBLOST OFFSET(14) NUMBITS(1) [],
        ABRT_SLVRD_INTX OFFSET(15) NUMBITS(1) [],
    ],
    IC_SLV_DATA_NACK_ONLY [
        NACK OFFSET(0) NUMBITS(1) [],
    ],
    IC_DMA_CR [
        RDMAE OFFSET(0) NUMBITS(1) [],
        TDMAE OFFSET(1) NUMBITS(1) [],
    ],
    IC_DMA_TDLR [
        DMATDL OFFSET(0) NUMBITS(1) [],
    ],
    IC_DMA_RDLR [
        DMARDL OFFSET(0) NUMBITS(1) [],
    ],
    IC_SDA_SETUP [
        SDA_SETUP OFFSET(0) NUMBITS(8) [],
    ],
    IC_ACK_GENERAL_CALL [
        ACK_GEN_CALL OFFSET(0) NUMBITS(1) [],
    ],
    IC_ENABLE_STATUS [
        IC_EN OFFSET(0) NUMBITS(1) [],
        SLV_DISABLED_SHILE_BUSY OFFSET(1) NUMBITS(1) [],
        SLV_RX_DATA_LOST OFFSET(2) NUMBITS(1) [],
    ],
    IC_FS_SPKLEN [
        IC_FS_SPKLEN OFFSET(0) NUMBITS(8) [],
    ],
    IC_HS_SPKLEN [
        IC_HS_SPKLEN OFFSET(0) NUMBITS(8) [],
    ],

];

register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x00 => con: ReadWrite<u32,IC_CON::Register>),
        (0x04 => tar: ReadWrite<u32,IC_TAR::Register>),
        (0x08 => sar: ReadWrite<u32,IC_SAR::Register>),
        (0x0C => hs_maddr: ReadWrite<u32,IC_HS_MADDR::Register>),
        (0x10 => data_cmd: WriteOnly<u32,IC_DATA_CMD::Register>),
        (0x14 => ss_scl_hcnt: ReadWrite<u32,IC_SS_SCL_HCNT::Register>),
        (0x18 => ss_scl_lcnt: ReadWrite<u32,IC_SS_SCL_LCNT::Register>),
        (0x1C => fs_scl_hcnt: ReadWrite<u32,IC_FS_SCL_HCNT::Register>),
        (0x20 => fs_scl_lcnt: ReadWrite<u32,IC_FS_SCL_LCNT::Register>),
        (0x24 => hs_scl_hcnt: ReadWrite<u32,IC_HS_SCL_HCNT::Register>),
        (0x28 => hs_scl_lcnt: ReadWrite<u32,IC_HS_SCL_LCNT::Register>),
        (0x2C =>intr_stat: ReadOnly<u32,IC_INTR_STAT::Register>),
        (0x30 =>intr_mask: ReadWrite<u32,IC_INTR_MASK::Register>),
        (0x34 =>raw_intr_stat: ReadOnly<u32,IC_RAW_INTR_STAT::Register>),
        (0x38 => rx_tl: ReadWrite<u32,IC_RX_TL::Register>),
        (0x3C => tx_tl: ReadWrite<u32,IC_TX_TL::Register>),
        (0x40 => clr_intr: ReadOnly<u32,IC_CLR_INTR::Register>),
        (0x44 => clr_rx_under: ReadOnly<u32,IC_CLR_RX_UNDER::Register>),
        (0x48 => clr_rx_over: ReadOnly<u32,IC_CLR_RX_OVER::Register>),
        (0x4C => clr_tx_over: ReadOnly<u32,IC_CLR_TX_OVER::Register>),
        (0x50 => clr_rd_req: ReadOnly<u32,IC_CLR_RD_REQ::Register>),
        (0x54 => clr_tx_abrt: ReadOnly<u32,IC_CLR_TX_ABRT::Register>),
        (0x58 => clr_rx_done: ReadOnly<u32,IC_CLR_RX_DONE::Register>),
        (0x5C => clr_activity: ReadOnly<u32,IC_CLR_ACTIVITY::Register>),
        (0x60 => clr_stop_del: ReadOnly<u32,IC_CLR_STOP_DEL::Register>),
        (0x64 => clr_start_del: ReadOnly<u32,IC_CLR_START_DEL::Register>),
        (0x68 => clr_gen_call: ReadOnly<u32,IC_CLR_GEN_CALL::Register>),
        (0x6C => enable: ReadWrite<u32,IC_ENABLE::Register>),
        (0x70 => status: ReadOnly<u32,IC_STATUS::Register>),
        (0x74 => txflr: ReadOnly<u32,IC_TXFLR::Register>),
        (0x78 => rxflr: ReadOnly<u32,IC_RXFLR::Register>),
        (0x7C => sda_hold: ReadWrite<u32,IC_SDA_HOLD::Register>),
        (0x80 => tx_abrt_source: ReadOnly<u32,IC_TX_ABRT_SOURCE::Register>),
        (0x84 => slv_data_nack_only: ReadWrite<u32,IC_SLV_DATA_NACK_ONLY::Register>),
        (0x88 => dma_cr: ReadWrite<u32,IC_DMA_CR::Register>),
        (0x8C => dma_tdlr: ReadWrite<u32,IC_DMA_TDLR::Register>),
        (0x90 => dma_rdlr: ReadWrite<u32,IC_DMA_RDLR::Register>),
        (0x94 => sda_setup: ReadWrite<u32,IC_SDA_SETUP::Register>),
        (0x98 => ack_general_call: ReadWrite<u32,IC_ACK_GENERAL_CALL::Register>),
        (0x9C => enable_status: ReadOnly<u32,IC_ENABLE_STATUS::Register>),
        (0xA0 => fs_spklen: ReadWrite<u32,IC_FS_SPKLEN::Register>),
        (0xA4 => hs_spklen: ReadWrite<u32,IC_HS_SPKLEN::Register>),
        (0xA8 => @END),
    }
}
