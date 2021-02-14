use bindings::windows::Guid;

pub static WPD_CONTENT_TYPE_FUNCTIONAL_OBJECT: Guid = Guid::from_values(
    0x99ED0160,
    0x17FF,
    0x4C44,
    [0x9D, 0x98, 0x1D, 0x7A, 0x6F, 0x94, 0x19, 0x21],
);

pub static WPD_CONTENT_TYPE_FOLDER: Guid = Guid::from_values(
    0x27E2E392,
    0xA111,
    0x48E0,
    [0xAB, 0x0C, 0xE1, 0x77, 0x05, 0xA0, 0x5F, 0x85],
);

pub static WPD_FUNCTIONAL_CATEGORY_DEVICE: Guid = Guid::from_values(
    0x08EA466B,
    0xE3A4,
    0x4336,
    [0xA1, 0xF3, 0xA4, 0x4D, 0x2B, 0x5C, 0x43, 0x8C],
);

pub static WPD_FUNCTIONAL_CATEGORY_STORAGE: Guid = Guid::from_values(
    0x23F05BBC,
    0x15DE,
    0x4C2A,
    [0xA5, 0x5B, 0xA9, 0xAF, 0x5C, 0xE4, 0x12, 0xEF],
);
