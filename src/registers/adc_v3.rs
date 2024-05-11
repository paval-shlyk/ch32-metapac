
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Adc",
            extends: None,
            description: Some(
                "Analog to digital converter.",
            ),
            items: &[
                BlockItem {
                    name: "statr",
                    description: Some(
                        "status register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Statr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctlr1",
                    description: Some(
                        "control register 1.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctlr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctlr2",
                    description: Some(
                        "control register 2.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctlr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "samptr1",
                    description: Some(
                        "sample time register 1.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Samptr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "samptr2",
                    description: Some(
                        "sample time register 2.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Samptr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iofr",
                    description: Some(
                        "injected channel data offset register x.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iofr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdhtr",
                    description: Some(
                        "watchdog higher threshold register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wdhtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdltr",
                    description: Some(
                        "watchdog lower threshold register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wdltr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rsqr1",
                    description: Some(
                        "regular sequence register 1.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rsqr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rsqr2",
                    description: Some(
                        "regular sequence register 2.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rsqr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rsqr3",
                    description: Some(
                        "regular sequence register 3.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rsqr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isqr",
                    description: Some(
                        "injected sequence register.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Isqr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "idatar",
                    description: Some(
                        "injected data register x.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Idatar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rdatar",
                    description: Some(
                        "regular data register.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rdatar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctlr3",
                    description: Some(
                        "ADC control register 3.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctlr3",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Tkey",
            extends: Some(
                "ADC",
            ),
            description: Some(
                "Touch key",
            ),
            items: &[
                BlockItem {
                    name: "statr",
                    description: Some(
                        "status register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Statr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctlr1",
                    description: Some(
                        "control register 1.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctlr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctlr2",
                    description: Some(
                        "control register 2.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctlr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "samptr1",
                    description: Some(
                        "sample time register 1.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Samptr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "samptr2",
                    description: Some(
                        "sample time register 2.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Samptr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iofr",
                    description: Some(
                        "injected channel data offset register x.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iofr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdhtr",
                    description: Some(
                        "watchdog higher threshold register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wdhtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdltr",
                    description: Some(
                        "watchdog lower threshold register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wdltr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rsqr1",
                    description: Some(
                        "regular sequence register 1.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rsqr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rsqr2",
                    description: Some(
                        "regular sequence register 2.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rsqr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rsqr3",
                    description: Some(
                        "regular sequence register 3.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rsqr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isqr",
                    description: Some(
                        "injected sequence register.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Isqr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chgoffset",
                    description: Some(
                        "Charge time offset register (map of ADC_IDATAR1)",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "idatar",
                    description: Some(
                        "injected data register x.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Idatar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "act_dcg",
                    description: Some(
                        "Activate and discharge time register (map of ADC_RDATAR)",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "dr",
                    description: Some(
                        "Data register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "rdatar",
                    description: Some(
                        "regular data register.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rdatar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctlr3",
                    description: Some(
                        "ADC control register 3.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctlr3",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ctlr1",
            extends: None,
            description: Some(
                "control register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awdch",
                    description: Some(
                        "Analog watchdog channel select bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eocie",
                    description: Some(
                        "Interrupt enable for EOC.",
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
                    name: "awdie",
                    description: Some(
                        "Analog watchdog interrupt enable.",
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
                    name: "jeocie",
                    description: Some(
                        "Interrupt enable for injected channels.",
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
                Field {
                    name: "scan",
                    description: Some(
                        "Scan mode enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awdsgl",
                    description: Some(
                        "Enable the watchdog on a single channel in scan mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jauto",
                    description: Some(
                        "Automatic injected group conversion.",
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
                    name: "discen",
                    description: Some(
                        "Discontinuous mode on regular channels.",
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
                    name: "jdiscen",
                    description: Some(
                        "Discontinuous mode on injected channels.",
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
                    name: "discnum",
                    description: Some(
                        "Discontinuous mode channel count.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dualmod",
                    description: Some(
                        "Dual mode selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jawden",
                    description: Some(
                        "Analog watchdog enable on injected channels.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awden",
                    description: Some(
                        "Analog watchdog enable on regular channels.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tkenable",
                    description: Some(
                        "Touch key module enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tkitune",
                    description: Some(
                        "Touch key current tune",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Tkitune",
                    ),
                },
                Field {
                    name: "bufen",
                    description: Some(
                        "ADC BUFFER enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pga",
                    description: Some(
                        "ADC channel gain configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pga",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ctlr2",
            extends: None,
            description: Some(
                "control register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adon",
                    description: Some(
                        "A/D converter ON / OFF.",
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
                    name: "cont",
                    description: Some(
                        "Continuous conversion.",
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
                    name: "cal",
                    description: Some(
                        "A/D calibration.",
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
                    name: "rstcal",
                    description: Some(
                        "Reset calibration.",
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
                    name: "dma",
                    description: Some(
                        "Direct memory access mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "align",
                    description: Some(
                        "Data alignment.",
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
                    name: "jextsel",
                    description: Some(
                        "External event select for injected group.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Jextsel",
                    ),
                },
                Field {
                    name: "jexttrig",
                    description: Some(
                        "External trigger conversion mode for injected channels.",
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
                Field {
                    name: "extsel",
                    description: Some(
                        "External event select for regular group.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Extsel",
                    ),
                },
                Field {
                    name: "exttrig",
                    description: Some(
                        "External trigger conversion mode for regular channels.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jswstart",
                    description: Some(
                        "Start conversion of injected channels.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "swstart",
                    description: Some(
                        "Start conversion of regular channels.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsvrefe",
                    description: Some(
                        "Temperature sensor and VREFINT enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ctlr3",
            extends: None,
            description: Some(
                "ADC control register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "clk_div",
                    description: Some(
                        "Clock divider.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Idatar",
            extends: None,
            description: Some(
                "injected data register x.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jdata",
                    description: Some(
                        "Injected data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iofr",
            extends: None,
            description: Some(
                "injected channel data offset register x.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "joffset",
                    description: Some(
                        "Data offset for injected channel x.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 0,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Isqr",
            extends: None,
            description: Some(
                "injected sequence register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jsq",
                    description: Some(
                        "1st conversion in injected sequence.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 5,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "jl",
                    description: Some(
                        "Injected sequence length.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rdatar",
            extends: None,
            description: Some(
                "regular data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "Regular data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc2data",
                    description: Some(
                        "Regular data of the second ADC, in dual mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rsqr1",
            extends: None,
            description: Some(
                "regular sequence register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sq",
                    description: Some(
                        "13th conversion in regular sequence.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 5,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "l",
                    description: Some(
                        "Regular channel sequence length.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rsqr2",
            extends: None,
            description: Some(
                "regular sequence register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sq",
                    description: Some(
                        "7th conversion in regular sequence.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 6,
                                stride: 5,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rsqr3",
            extends: None,
            description: Some(
                "regular sequence register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sq",
                    description: Some(
                        "1st conversion in regular sequence.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 6,
                                stride: 5,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Samptr1",
            extends: None,
            description: Some(
                "sample time register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smp",
                    description: Some(
                        "Channel 10 sample time selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 3,
                            },
                        ),
                    ),
                    enumm: Some(
                        "SampleTime",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Samptr2",
            extends: None,
            description: Some(
                "sample time register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smp",
                    description: Some(
                        "Channel 0 sample time selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 10,
                                stride: 3,
                            },
                        ),
                    ),
                    enumm: Some(
                        "SampleTime",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Statr",
            extends: None,
            description: Some(
                "status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awd",
                    description: Some(
                        "Analog watchdog flag.",
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
                    name: "eoc",
                    description: Some(
                        "Regular channel end of conversion.",
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
                    name: "jeoc",
                    description: Some(
                        "Injected channel end of conversion.",
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
                    name: "jstrt",
                    description: Some(
                        "Injected channel start flag.",
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
                    name: "strt",
                    description: Some(
                        "Regular channel start flag.",
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
            ],
        },
        FieldSet {
            name: "Wdhtr",
            extends: None,
            description: Some(
                "watchdog higher threshold register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ht",
                    description: Some(
                        "Analog watchdog higher threshold.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wdltr",
            extends: None,
            description: Some(
                "watchdog lower threshold register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lt",
                    description: Some(
                        "Analog watchdog lower threshold.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Extsel",
            description: Some(
                "External trigger event select for regular channels",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "TIM1_CC1",
                    description: Some(
                        "Timer 1 capture compare 1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TIM1_CC2",
                    description: Some(
                        "Timer 1 capture compare 2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "TIM1_CC3",
                    description: Some(
                        "Timer 1 capture compare 3",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "TIM2_CC2",
                    description: Some(
                        "Timer 2 capture compare 2",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "TIM3_TRGO",
                    description: Some(
                        "Timer 3 TRGO event",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "TIM4_CC4",
                    description: Some(
                        "Timer 4 capture compare 4",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "TIM8_CC1_EXTI11",
                    description: Some(
                        "Timer 8 capture compare 1 and external interrupt line 11",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "SWSTART",
                    description: Some(
                        "Software start",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Jextsel",
            description: Some(
                "External trigger event select for injected channels",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "TIM1_TRGO",
                    description: Some(
                        "Timer 1 TRGO event",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TIM1_CC4",
                    description: Some(
                        "Timer 1 capture compare 4",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "TIM2_TRGO",
                    description: Some(
                        "Timer 2 TRGO event",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "TIM2_CC1",
                    description: Some(
                        "Timer 2 capture compare 1",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "TIM3_CC4",
                    description: Some(
                        "Timer 3 capture compare 4",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "TIM4_TRGO",
                    description: Some(
                        "Timer 4 TRGO event",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "TIM8_CC4_EXTI15",
                    description: Some(
                        "Timer 8 capture compare 4 and external interrupt line 15",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "JSWSTART",
                    description: Some(
                        "Software start",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Pga",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "X1",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "X4",
                    description: Some(
                        "x4",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "X16",
                    description: Some(
                        "x16",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "X64",
                    description: Some(
                        "x64",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "SampleTime",
            description: Some(
                "Sample time selection",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "CYCLES1_5",
                    description: Some(
                        "1.5 cycles",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CYCLES7_5",
                    description: Some(
                        "7.5 cycles",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CYCLES13_5",
                    description: Some(
                        "13.5 cycles",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CYCLES28_5",
                    description: Some(
                        "28.5 cycles",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CYCLES41_5",
                    description: Some(
                        "41.5 cycles",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "CYCLES55_5",
                    description: Some(
                        "55.5 cycles",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "CYCLES71_5",
                    description: Some(
                        "71.5 cycles",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "CYCLES239_5",
                    description: Some(
                        "239.5 cycles",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Tkitune",
            description: Some(
                "Touch key current tune",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "FULL",
                    description: Some(
                        "35uA",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HALF",
                    description: Some(
                        "Decrease to half",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
                