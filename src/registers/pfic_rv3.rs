
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Pfic",
            extends: None,
            description: Some(
                "Programmable Fast Interrupt Controller.",
            ),
            items: &[
                BlockItem {
                    name: "isr1",
                    description: Some(
                        "Interrupt Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Isr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isr2",
                    description: Some(
                        "Interrupt Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Isr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipr1",
                    description: Some(
                        "Interrupt Pending Register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipr2",
                    description: Some(
                        "Interrupt Pending Register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ithresdr",
                    description: Some(
                        "Interrupt Priority Register.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ithresdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fibaddrr",
                    description: Some(
                        "Interrupt Fast Address Register.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fibaddrr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr",
                    description: Some(
                        "Interrupt Config Register.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gisr",
                    description: Some(
                        "Interrupt Global Register.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Gisr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fifoaddrr0",
                    description: Some(
                        "Interrupt 0 address Register.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fifoaddrr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fifoaddrr1",
                    description: Some(
                        "Interrupt 1 address Register.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fifoaddrr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fifoaddrr2",
                    description: Some(
                        "Interrupt 2 address Register.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fifoaddrr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fifoaddrr3",
                    description: Some(
                        "Interrupt 3 address Register.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fifoaddrr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ienr1",
                    description: Some(
                        "Interrupt Setting Register.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ienr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ienr2",
                    description: Some(
                        "Interrupt Setting Register.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ienr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irer1",
                    description: Some(
                        "Interrupt Clear Register.",
                    ),
                    array: None,
                    byte_offset: 0x180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Irer1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irer2",
                    description: Some(
                        "Interrupt Clear Register.",
                    ),
                    array: None,
                    byte_offset: 0x184,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Irer2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipsr1",
                    description: Some(
                        "Interrupt Pending Register.",
                    ),
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipsr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipsr2",
                    description: Some(
                        "Interrupt Pending Register.",
                    ),
                    array: None,
                    byte_offset: 0x204,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipsr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprr1",
                    description: Some(
                        "Interrupt Pending Clear Register.",
                    ),
                    array: None,
                    byte_offset: 0x280,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprr2",
                    description: Some(
                        "Interrupt Pending Clear Register.",
                    ),
                    array: None,
                    byte_offset: 0x284,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iactr1",
                    description: Some(
                        "Interrupt ACTIVE Register.",
                    ),
                    array: None,
                    byte_offset: 0x300,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iactr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iactr2",
                    description: Some(
                        "Interrupt ACTIVE Register.",
                    ),
                    array: None,
                    byte_offset: 0x304,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iactr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sctlr",
                    description: Some(
                        "System Control Register.",
                    ),
                    array: None,
                    byte_offset: 0xd10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sctlr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cfgr",
            extends: None,
            description: Some(
                "Interrupt Config Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hwstkctrl",
                    description: Some(
                        "HWSTKCTRL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hwstkctrl",
                    ),
                },
                Field {
                    name: "nestctrl",
                    description: Some(
                        "NESTCTRL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Nestctrl",
                    ),
                },
                Field {
                    name: "nmiset",
                    description: Some(
                        "NMISET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Nmiset",
                    ),
                },
                Field {
                    name: "nmireset",
                    description: Some(
                        "NMIRESET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Nmireset",
                    ),
                },
                Field {
                    name: "excset",
                    description: Some(
                        "EXCSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Excset",
                    ),
                },
                Field {
                    name: "excreset",
                    description: Some(
                        "EXCRESET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Excreset",
                    ),
                },
                Field {
                    name: "pficrset",
                    description: Some(
                        "PFICRSET.",
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
                    name: "sysreset",
                    description: Some(
                        "SYSRESET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sysreset",
                    ),
                },
                Field {
                    name: "keycode",
                    description: Some(
                        "KEYCODE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: Some(
                        "Keycode",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Fibaddrr",
            extends: None,
            description: Some(
                "Interrupt Fast Address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "baseaddr",
                    description: Some(
                        "BASEADDR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fifoaddrr0",
            extends: None,
            description: Some(
                "Interrupt 0 address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "offaddr0",
                    description: Some(
                        "OFFADDR0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "irqid0",
                    description: Some(
                        "IRQID0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fifoaddrr1",
            extends: None,
            description: Some(
                "Interrupt 1 address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "offaddr1",
                    description: Some(
                        "OFFADDR1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "irqid1",
                    description: Some(
                        "IRQID1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fifoaddrr2",
            extends: None,
            description: Some(
                "Interrupt 2 address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "offaddr2",
                    description: Some(
                        "OFFADDR2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "irqid2",
                    description: Some(
                        "IRQID2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fifoaddrr3",
            extends: None,
            description: Some(
                "Interrupt 3 address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "offaddr3",
                    description: Some(
                        "OFFADDR3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "irqid3",
                    description: Some(
                        "IRQID3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gisr",
            extends: None,
            description: Some(
                "Interrupt Global Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "neststa",
                    description: Some(
                        "NESTSTA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: Some(
                        "Neststa",
                    ),
                },
                Field {
                    name: "gactsta",
                    description: Some(
                        "GACTSTA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Gactsta",
                    ),
                },
                Field {
                    name: "gpendsta",
                    description: Some(
                        "GPENDSTA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Gpendsta",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Iactr1",
            extends: None,
            description: Some(
                "Interrupt ACTIVE Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iacts",
                    description: Some(
                        "IACTS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iactr2",
            extends: None,
            description: Some(
                "Interrupt ACTIVE Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iacts",
                    description: Some(
                        "IACTS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 28,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ienr1",
            extends: None,
            description: Some(
                "Interrupt Setting Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inten",
                    description: Some(
                        "INTEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ienr2",
            extends: None,
            description: Some(
                "Interrupt Setting Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inten",
                    description: Some(
                        "INTEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 28,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ipr1",
            extends: None,
            description: Some(
                "Interrupt Pending Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendsta2_3",
                    description: Some(
                        "PENDSTA.",
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
                    name: "pendsta12_31",
                    description: Some(
                        "PENDSTA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ipr2",
            extends: None,
            description: Some(
                "Interrupt Pending Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendsta",
                    description: Some(
                        "PENDSTA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 28,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprr1",
            extends: None,
            description: Some(
                "Interrupt Pending Clear Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendreset2_3",
                    description: Some(
                        "PENDRESET.",
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
                    name: "pendreset12_31",
                    description: Some(
                        "PENDRESET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprr2",
            extends: None,
            description: Some(
                "Interrupt Pending Clear Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendreset",
                    description: Some(
                        "PENDRESET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 28,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ipsr1",
            extends: None,
            description: Some(
                "Interrupt Pending Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendset2_3",
                    description: Some(
                        "PENDSET.",
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
                    name: "pendset12_31",
                    description: Some(
                        "PENDSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ipsr2",
            extends: None,
            description: Some(
                "Interrupt Pending Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendset",
                    description: Some(
                        "PENDSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 28,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Irer1",
            extends: None,
            description: Some(
                "Interrupt Clear Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "intrset",
                    description: Some(
                        "INTRSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Irer2",
            extends: None,
            description: Some(
                "Interrupt Clear Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "intrset",
                    description: Some(
                        "INTRSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 28,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Isr1",
            extends: None,
            description: Some(
                "Interrupt Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "intensta2_3",
                    description: Some(
                        "Interrupt ID Status.",
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
                    name: "intensta12_31",
                    description: Some(
                        "Interrupt ID Status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Isr2",
            extends: None,
            description: Some(
                "Interrupt Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "intensta",
                    description: Some(
                        "Interrupt ID Status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 28,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ithresdr",
            extends: None,
            description: Some(
                "Interrupt Priority Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "threshold",
                    description: Some(
                        "THRESHOLD.",
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
            name: "Sctlr",
            extends: None,
            description: Some(
                "System Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sleeponexit",
                    description: Some(
                        "SLEEPONEXIT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sleeponexit",
                    ),
                },
                Field {
                    name: "sleepdeep",
                    description: Some(
                        "SLEEPDEEP.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sleepdeep",
                    ),
                },
                Field {
                    name: "wfitowfe",
                    description: Some(
                        "WFITOWFE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wfitowfe",
                    ),
                },
                Field {
                    name: "sevonpend",
                    description: Some(
                        "SEVONPEND.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sevonpend",
                    ),
                },
                Field {
                    name: "setevent",
                    description: Some(
                        "SETEVENT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Setevent",
                    ),
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Excreset",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RESET",
                    description: Some(
                        "Reset the module.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Excset",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SET",
                    description: Some(
                        "Set interrupt.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Gactsta",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOINTERRUPT",
                    description: Some(
                        "No interrupt ongoing.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HASINTERRUPT",
                    description: Some(
                        "Interrupt ongoing.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Gpendsta",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOPENDINGINTERRUPT",
                    description: Some(
                        "No interrupt pending.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HASPENDINGINTERRUPT",
                    description: Some(
                        "Has interrupt pending.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Hwstkctrl",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Hardware stack enabled.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Hardware stack disabled.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Keycode",
            description: None,
            bit_size: 16,
            variants: &[
                EnumVariant {
                    name: "KEY2",
                    description: Some(
                        "NMI and EXC key.",
                    ),
                    value: 48303,
                },
                EnumVariant {
                    name: "KEY3",
                    description: Some(
                        "System Reset key.",
                    ),
                    value: 48879,
                },
                EnumVariant {
                    name: "KEY1",
                    description: Some(
                        "HWSTK and NEST key.",
                    ),
                    value: 64005,
                },
            ],
        },
        Enum {
            name: "Nestctrl",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Interrupt nesting enabled.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Interrupt nesting disabled.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Neststa",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "NOINTERRUPT",
                    description: Some(
                        "No interrupt ongoing.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PRIMARY",
                    description: Some(
                        "Primary interrupt ongoing.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "SECONDARY",
                    description: Some(
                        "Secondary interrupt ongoing.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Nmireset",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RESET",
                    description: Some(
                        "Reset the module.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Nmiset",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SET",
                    description: Some(
                        "Set interrupt.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Setevent",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SET",
                    description: Some(
                        "Set WFE event.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sevonpend",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ONLYENABLED",
                    description: Some(
                        "Only enabled events and interrupts can wake up the system.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ALLINTERRUPTS",
                    description: Some(
                        "Enabled events and all interrupts can wake up the system.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sleepdeep",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SLEEP",
                    description: Some(
                        "Sleep mode.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DEEPSLEEP",
                    description: Some(
                        "Deep Sleep mode.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sleeponexit",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CONTINUE",
                    description: Some(
                        "Don't sleep after exiting interrupt service.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SLEEP",
                    description: Some(
                        "Enter sleep mode after exiting interrupt service.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sysreset",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RESET",
                    description: Some(
                        "Reset the module.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wfitowfe",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "Nothing.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "WFI is treated as WFE.",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
                