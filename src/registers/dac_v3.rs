
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dac",
            extends: None,
            description: Some(
                "Digital-to-analog converter",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "control register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "swtrigr",
                    description: Some(
                        "software trigger register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Swtrigr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhr12r",
                    description: Some(
                        "channel 12-bit right-aligned data holding register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 12,
                            },
                        ),
                    ),
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhr12r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhr12l",
                    description: Some(
                        "channel 12-bit left-aligned data holding register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 12,
                            },
                        ),
                    ),
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhr12l",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhr8r",
                    description: Some(
                        "channel 8-bit right-aligned data holding register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 12,
                            },
                        ),
                    ),
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhr8r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhr12rd",
                    description: Some(
                        "dual 12-bit right-aligned data holding register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhr12rd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhr12ld",
                    description: Some(
                        "dual 12-bit left aligned data holding register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhr12ld",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhr8rd",
                    description: Some(
                        "dual 8-bit right aligned data holding register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhr8rd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dor",
                    description: Some(
                        "channel data output register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dor",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "channel enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "boff",
                    description: Some(
                        "channel output buffer disable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ten",
                    description: Some(
                        "channel trigger enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tsel",
                    description: Some(
                        "channel trigger selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: Some(
                        "TrigSel",
                    ),
                },
                Field {
                    name: "wave",
                    description: Some(
                        "channel noise/triangle wave generation enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Wave",
                    ),
                },
                Field {
                    name: "mamp",
                    description: Some(
                        "channel mask/amplitude selector",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "dmaen",
                    description: Some(
                        "channel DMA enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhr12l",
            extends: None,
            description: Some(
                "channel 12-bit left-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhr",
                    description: Some(
                        "channel 12-bit left-aligned data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhr12ld",
            extends: None,
            description: Some(
                "dual 12-bit left aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhr",
                    description: Some(
                        "channel 12-bit left-aligned data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 12,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhr12r",
            extends: None,
            description: Some(
                "channel 12-bit right-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhr",
                    description: Some(
                        "channel 12-bit right-aligned data",
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
            name: "Dhr12rd",
            extends: None,
            description: Some(
                "dual 12-bit right-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhr",
                    description: Some(
                        "channel 12-bit right-aligned data",
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
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhr8r",
            extends: None,
            description: Some(
                "channel 8-bit right-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhr",
                    description: Some(
                        "channel 8-bit right-aligned data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhr8rd",
            extends: None,
            description: Some(
                "dual 8-bit right aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhr",
                    description: Some(
                        "channel 8-bit right-aligned data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dor",
            extends: None,
            description: Some(
                "channel data output register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dor",
                    description: Some(
                        "channel data output",
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
            name: "Swtrigr",
            extends: None,
            description: Some(
                "software trigger register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "swtrig",
                    description: Some(
                        "channel software trigger",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "TrigSel",
            description: Some(
                "DAC channel1/2 trigger selection.",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "TIM6_TRGO",
                    description: Some(
                        "TIM6 TRGO event.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TIM8_TRGO",
                    description: Some(
                        "TIM8 TRGO event.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "TIM7_TRGO",
                    description: Some(
                        "TIM7 TRGO event.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "TIM5_TRGO",
                    description: Some(
                        "TIM5 TRGO event.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "TIM2_TRGO",
                    description: Some(
                        "TIM2 TRGO event.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "TIM4_TRGO",
                    description: Some(
                        "TIM4 TRGO event.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "EXTI9",
                    description: Some(
                        "External line 9.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "SOFTWARE",
                    description: Some(
                        "Software trigger.",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Wave",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Wave generation disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NOISE",
                    description: Some(
                        "Noise wave generation enabled",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "TRIANGLE",
                    description: Some(
                        "Triangle wave generation enabled",
                    ),
                    value: 2,
                },
            ],
        },
    ],
};
                