use bindings::windows::win32::windows_properties_system::PROPERTYKEY;
use bindings::windows::Guid;

const fn define_propertykey(
    l: u32,
    w1: u16,
    w2: u16,
    b1: u8,
    b2: u8,
    b3: u8,
    b4: u8,
    b5: u8,
    b6: u8,
    b7: u8,
    b8: u8,
    pid: u32,
) -> PROPERTYKEY {
    PROPERTYKEY {
        fmtid: Guid::from_values(l, w1, w2, [b1, b2, b3, b4, b5, b6, b7, b8]),
        pid,
    }
}

pub static WPD_OBJECT_NAME: PROPERTYKEY = define_propertykey(
    0xEF6B490D, 0x5CD8, 0x437A, 0xAF, 0xFC, 0xDA, 0x8B, 0x60, 0xEE, 0x4A, 0x3C, 4,
);

pub static WPD_OBJECT_CONTENT_TYPE: PROPERTYKEY = define_propertykey(
    0xEF6B490D, 0x5CD8, 0x437A, 0xAF, 0xFC, 0xDA, 0x8B, 0x60, 0xEE, 0x4A, 0x3C, 7,
);

pub static WPD_FUNCTIONAL_OBJECT_CATEGORY: PROPERTYKEY = define_propertykey(
    0x8F052D93, 0xABCA, 0x4FC5, 0xA5, 0xAC, 0xB0, 0x1D, 0xF4, 0xDB, 0xE5, 0x98, 2,
);
