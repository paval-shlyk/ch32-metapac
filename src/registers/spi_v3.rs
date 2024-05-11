
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Spi",
            extends: None,
            description: Some(
                "Serial peripheral interface.",
            ),
            items: &[
                BlockItem {
                    name: "ctlr1",
                    description: Some(
                        "control register 1.",
                    ),
                    array: None,
                    byte_offset: 0x0,
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
                    byte_offset: 0x4,
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
                    name: "statr",
                    description: Some(
                        "status register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
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
                    name: "datar",
                    description: Some(
                        "data register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Datar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "crcr",
                    description: Some(
                        "CRCR polynomial register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Crcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rcrcr",
                    description: Some(
                        "RX CRC register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Rcrcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tcrcr",
                    description: Some(
                        "TX CRC register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Tcrcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "i2s_cfgr",
                    description: Some(
                        "SPI_I2S configure register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "I2sCfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "i2spr",
                    description: Some(
                        "SPI_I2S prescaler register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "I2spr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hscr",
                    description: Some(
                        "high speed control register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hscr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Crcr",
            extends: None,
            description: Some(
                "CRCR polynomial register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "crcpoly",
                    description: Some(
                        "CRC polynomial register.",
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
            name: "Ctlr1",
            extends: None,
            description: Some(
                "control register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cpha",
                    description: Some(
                        "Clock phase.",
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
                    name: "cpol",
                    description: Some(
                        "Clock polarity.",
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
                    name: "mstr",
                    description: Some(
                        "Master selection.",
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
                    name: "br",
                    description: Some(
                        "Baud rate control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "BaudRate",
                    ),
                },
                Field {
                    name: "spe",
                    description: Some(
                        "SPI enable.",
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
                    name: "lsbfirst",
                    description: Some(
                        "Frame format.",
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
                    name: "ssi",
                    description: Some(
                        "Internal slave select.",
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
                    name: "ssm",
                    description: Some(
                        "Software slave management.",
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
                    name: "rxonly",
                    description: Some(
                        "Receive only.",
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
                    name: "dff",
                    description: Some(
                        "Data frame format.",
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
                    name: "crcnext",
                    description: Some(
                        "CRC transfer next.",
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
                    name: "crcen",
                    description: Some(
                        "Hardware CRC calculation enable.",
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
                    name: "bidioe",
                    description: Some(
                        "Output enable in bidirectional mode.",
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
                    name: "bidimode",
                    description: Some(
                        "Bidirectional data mode enable.",
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
            name: "Ctlr2",
            extends: None,
            description: Some(
                "control register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxdmaen",
                    description: Some(
                        "Rx buffer DMA enable.",
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
                    name: "txdmaen",
                    description: Some(
                        "Tx buffer DMA enable.",
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
                    name: "ssoe",
                    description: Some(
                        "SS output enable.",
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
                    name: "errie",
                    description: Some(
                        "Error interrupt enable.",
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
                    name: "rxneie",
                    description: Some(
                        "RX buffer not empty interrupt enable.",
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
                    name: "txeie",
                    description: Some(
                        "Tx buffer empty interrupt enable.",
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
            name: "Datar",
            extends: None,
            description: Some(
                "data register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "datar",
                    description: Some(
                        "Data register.",
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
            name: "Hscr",
            extends: None,
            description: Some(
                "high speed control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsrxen",
                    description: Some(
                        "High speed mode read enable.",
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
            ],
        },
        FieldSet {
            name: "I2sCfgr",
            extends: None,
            description: Some(
                "SPI_I2S configure register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chlen",
                    description: Some(
                        "Channel length (number of bits per audio channel).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Chlen",
                    ),
                },
                Field {
                    name: "datlen",
                    description: Some(
                        "DATLEN[1:0] bits (Data length to be transferred).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2sdatlen",
                    ),
                },
                Field {
                    name: "ckpol",
                    description: Some(
                        "steady state clock polarity.",
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
                    name: "i2sstd",
                    description: Some(
                        "I2SSTD[1:0] bits (I2S standard selection).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2sstd",
                    ),
                },
                Field {
                    name: "pcmsync",
                    description: Some(
                        "PCM frame synchronization.",
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
                    name: "i2scfg",
                    description: Some(
                        "I2SCFG[1:0] bits (I2S configuration mode).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "I2scfg",
                    ),
                },
                Field {
                    name: "i2se",
                    description: Some(
                        "I2S Enable.",
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
                    name: "i2smod",
                    description: Some(
                        "I2S mode selection.",
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
            ],
        },
        FieldSet {
            name: "I2spr",
            extends: None,
            description: Some(
                "SPI_I2S prescaler register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "i2sdiv",
                    description: Some(
                        "I2SDIV[7:0] bits (I2S Linear prescaler).",
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
                Field {
                    name: "odd",
                    description: Some(
                        "Odd factor for the prescaler.",
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
                    name: "mckoe",
                    description: Some(
                        "Master clock output enable.",
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
            ],
        },
        FieldSet {
            name: "Rcrcr",
            extends: None,
            description: Some(
                "RX CRC register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxcrc",
                    description: Some(
                        "Rx CRC register.",
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
            name: "Statr",
            extends: None,
            description: Some(
                "status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxne",
                    description: Some(
                        "Receive buffer not empty.",
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
                    name: "txe",
                    description: Some(
                        "Transmit buffer empty.",
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
                    name: "chsid",
                    description: Some(
                        "Channel side.",
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
                    name: "udr",
                    description: Some(
                        "Underrun flag.",
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
                    name: "crcerr",
                    description: Some(
                        "CRC error flag.",
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
                    name: "modf",
                    description: Some(
                        "Mode fault.",
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
                    name: "ovr",
                    description: Some(
                        "Overrun flag.",
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
                    name: "bsy",
                    description: Some(
                        "Busy flag.",
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
            name: "Tcrcr",
            extends: None,
            description: Some(
                "TX CRC register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txcrc",
                    description: Some(
                        "Tx CRC register.",
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
    ],
    enums: &[
        Enum {
            name: "BaudRate",
            description: Some(
                "Baud rate control.",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIV_2",
                    description: Some(
                        "fPCLK/2",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV_4",
                    description: Some(
                        "fPCLK/4",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV_8",
                    description: Some(
                        "fPCLK/8",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV_16",
                    description: Some(
                        "fPCLK/16",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DIV_32",
                    description: Some(
                        "fPCLK/32",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV_64",
                    description: Some(
                        "fPCLK/64",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV_128",
                    description: Some(
                        "fPCLK/128",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV_256",
                    description: Some(
                        "fPCLK/256",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Chlen",
            description: Some(
                "Channel length (number of bits per audio channel).",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "BIT16",
                    description: Some(
                        "16-bit data length.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BIT32",
                    description: Some(
                        "32-bit data length.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "I2scfg",
            description: Some(
                "I2S mode selection",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "SLAVETX",
                    description: Some(
                        "Slave transmit.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SLAVERX",
                    description: Some(
                        "Slave receive.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MASTERTX",
                    description: Some(
                        "Master transmit.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "MASTERRX",
                    description: Some(
                        "Master receive.",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "I2sdatlen",
            description: Some(
                "Data length to be transferred.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BIT16",
                    description: Some(
                        "16-bit data length.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BIT24",
                    description: Some(
                        "24-bit data length.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BIT32",
                    description: Some(
                        "32-bit data length.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "I2sstd",
            description: Some(
                "I2S standard selection.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PHILIPS",
                    description: Some(
                        "I2S Philips standard.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MSB",
                    description: Some(
                        "MSB justified standard, left justified.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LSB",
                    description: Some(
                        "LSB justified standard, right justified.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PCM",
                    description: Some(
                        "PCM standard.",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
                