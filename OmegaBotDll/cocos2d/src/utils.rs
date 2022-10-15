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

pub fn read_value<T>(address: usize) -> *mut T {
    address as _
}

pub unsafe fn read_ptr(address: usize) -> usize {
    *read_value(address)
}

#[macro_export]
macro_rules! lpcstr(
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as _
    }
);
