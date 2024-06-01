use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Usbpd",
            extends: None,
            description: Some(
                "USBPD configuration.",
            ),
            items: &[
                BlockItem {
                    name: "config",
                    description: Some(
                        "PD interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Config",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bmc_clk_cnt",
                    description: Some(
                        "BMC sampling clock counter.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "BmcClkCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "control",
                    description: Some(
                        "PD Send and receive enable register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Control",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tx_sel",
                    description: Some(
                        "SOP port selection register.",
                    ),
                    array: None,
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "TxSel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bmc_tx_sz",
                    description: Some(
                        "PD send length register.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "BmcTxSz",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "data_buf",
                    description: Some(
                        "DMA cache data register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "status",
                    description: Some(
                        "PD interrupt flag register.",
                    ),
                    array: None,
                    byte_offset: 0x9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Status",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bmc_byte_cnt",
                    description: Some(
                        "Byte counter.",
                    ),
                    array: None,
                    byte_offset: 0xa,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "BmcByteCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "port_cc",
                    description: Some(
                        "CC port control register.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "PortCc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma",
                    description: Some(
                        "PD buffer start address register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "BmcByteCnt",
            extends: None,
            description: Some(
                "Byte counter.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "bmc_byte_cnt",
                    description: Some(
                        "BMC_BYTE_CNT value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "BmcClkCnt",
            extends: None,
            description: Some(
                "BMC sampling clock counter.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "bmc_clk_cnt",
                    description: Some(
                        "R/T counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "BmcTxSz",
            extends: None,
            description: Some(
                "PD send length register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "bmc_tx_sz",
                    description: Some(
                        "BMC_TX_SZ value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Config",
            extends: None,
            description: Some(
                "PD interrupt enable register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "cc_filter",
                    description: Some(
                        "PIN filtering enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pd_all_clr",
                    description: Some(
                        "PD ITClear.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cc_sel",
                    description: Some(
                        "PD Commutation port.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "CcSel",
                    ),
                },
                Field {
                    name: "pd_dma_en",
                    description: Some(
                        "PD DMA Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pd_rst_en",
                    description: Some(
                        "PD RST Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wake_polar",
                    description: Some(
                        "wakeup polarity.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ie_pd_io",
                    description: Some(
                        "IO Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ie_rx_bit",
                    description: Some(
                        "bit interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ie_rx_byte",
                    description: Some(
                        "Receive byte register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ie_rx_act",
                    description: Some(
                        "Receive complete register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ie_rx_reset",
                    description: Some(
                        "Receive complete rst register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ie_tx_end",
                    description: Some(
                        "transfer complete register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Control",
            extends: None,
            description: Some(
                "PD Send and receive enable register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "pd_tx_en",
                    description: Some(
                        "PD_TX_EN value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bmc_start",
                    description: Some(
                        "BMC_START value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rx_start",
                    description: Some(
                        "PD receive status identification.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "data_flag",
                    description: Some(
                        "DATA_FLAG value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tx_bit_back",
                    description: Some(
                        "TX_BIT_BACK value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bmc_byte_hi",
                    description: Some(
                        "BMC_BYTE_HI value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PortCc",
            extends: None,
            description: Some(
                "CC port control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "pa_cc_ai",
                    description: Some(
                        "CC1 output of the voltage comparator.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cc_pd",
                    description: Some(
                        "CC1 port pull-down resistor enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cc_pu",
                    description: Some(
                        "CC1 port pull-up current selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "PortCcPu",
                    ),
                },
                Field {
                    name: "cc_lve",
                    description: Some(
                        "CC1 port output of the low voltage.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cc_ce",
                    description: Some(
                        "CC1 voltage comparator enable and voltage comparator reference voltage. (CC_CE and CC_CVS)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "PortCcCe",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Status",
            extends: None,
            description: Some(
                "PD interrupt flag register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "bmc_aux",
                    description: Some(
                        "BMC_AUX value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "BmcAux",
                    ),
                },
                Field {
                    name: "buf_err",
                    description: Some(
                        "BUF_ERR value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "if_rx_bit",
                    description: Some(
                        "IF_RX_BIT value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "if_rx_byte",
                    description: Some(
                        "IF_RX_BYTE value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "if_rx_act",
                    description: Some(
                        "IF_RX_ACT value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "if_rx_reset",
                    description: Some(
                        "IF_RX_RESET value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "if_tx_end",
                    description: Some(
                        "IF_TX_END value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "TxSel",
            extends: None,
            description: Some(
                "SOP port selection register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tx_sel1",
                    description: Some(
                        "K-CODE1 type selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tx_sel2",
                    description: Some(
                        "K-CODE2 type selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tx_sel3",
                    description: Some(
                        "K-CODE3 type selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tx_sel4",
                    description: Some(
                        "K-CODE4 type selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "BmcAux",
            description: Some(
                "PD status after receive.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NONE",
                    description: Some(
                        "BMC_AUX0 value.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SOP0",
                    description: Some(
                        "SOP, aka SOP0",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "SOP1",
                    description: Some(
                        "SOP', aka SOP1 or Hard Reset",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "SOP2",
                    description: Some(
                        "SOP'', aka SOP2 or Cable Resed",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "CcSel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "CC1",
                    description: Some(
                        "Select CC1.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CC2",
                    description: Some(
                        "Select CC2.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CC3",
                    description: Some(
                        "Select CC3.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "PortCcCe",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "CLOSED",
                    description: Some(
                        "No pull up current.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "V0_55",
                    description: Some(
                        "0.55V",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "V0_22",
                    description: Some(
                        "0.22V",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "V0_66",
                    description: Some(
                        "0.66V",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "V1_23",
                    description: Some(
                        "1.23V",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "PortCcPu",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "CLOSED",
                    description: Some(
                        "No pull up current.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "UA330",
                    description: Some(
                        "330uA.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "UA180",
                    description: Some(
                        "180uA.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "UA80",
                    description: Some(
                        "80uA.",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
