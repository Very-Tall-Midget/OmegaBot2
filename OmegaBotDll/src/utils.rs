pub fn read_value<T>(address: usize) -> *mut T {
    address as _
}

pub unsafe fn read_ptr(address: usize) -> usize {
    *read_value(address)
}

// pub fn get_offset_address(base: usize, offsets: Vec<usize>) -> usize {
//     if offsets.len() > 1 {
//         let mut buf = unsafe { read_ptr(base + offsets[0]) };

//         // filter so that first and last elements are not included
//         for offset in offsets.iter().enumerate().filter_map(|(i, val)| {
//             if i > 0 && i != offsets.len() - 1 {
//                 Some(*val)
//             } else {
//                 None
//             }
//         }) {
//             buf = unsafe { read_ptr(buf + offset) };
//         }
//         buf + offsets.last().unwrap()
//     } else {
//         if offsets.len() != 0 {
//             base + offsets[0]
//         } else {
//             base
//         }
//     }
// }

pub fn patch(address: usize, bytes: Vec<u8>) {
    unsafe {
        let mut old: u32 = 0;
        winapi::um::memoryapi::VirtualProtect(
            address as _,
            bytes.len(),
            winapi::um::winnt::PAGE_EXECUTE_READWRITE,
            &mut old as _,
        );
        winapi::um::winnt::RtlCopyMemory(address as _, bytes.as_ptr() as _, bytes.len());
        winapi::um::memoryapi::VirtualProtect(address as _, bytes.len(), old, 0 as _);
    }
}

macro_rules! lpcstr(
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as _
    }
);

pub trait IsNull {
    fn is_null(&self) -> bool;

    fn expect(&self, error: &str) -> &Self {
        if self.is_null() {
            panic!("{}", error);
        }
        self
    }

    fn if_not_null<T>(&self, f: impl FnOnce(&Self) -> T) -> Option<T> {
        if !self.is_null() {
            Some(f(self))
        } else {
            None
        }
    }
}

#[macro_export]
macro_rules! impl_is_null(
    ($t:ty) => {
        impl $crate::utils::IsNull for $t {
            fn is_null(&self) -> bool {
                self.address == 0
            }
        }
    }
);

#[macro_export]
macro_rules! get_orig (
    ($orig_name:ident $callconv:literal($($t:ty),+) $(-> $ret:ty)?) => {
        std::mem::transmute::<*mut std::ffi::c_void, unsafe extern $callconv fn($($t),+) $(-> $ret)?>($orig_name)
    }
);

#[macro_export]
macro_rules! string_from_utf16 {
    ($s:expr) => {
        String::from_utf16(&$s)
            .unwrap()
            .trim_matches(char::from(0))
            .to_string()
    };
}
