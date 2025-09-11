use core::{ffi::c_void, fmt::Write};

use legacy::SEGGER_RTT_Write;

pub struct RTT;

impl Write for RTT {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let len = s.len();
        let ptr = s.as_ptr() as *const c_void;

        unsafe {
            SEGGER_RTT_Write(0, ptr, len as u32);
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! println {
    ($($args:expr),*) => {
        {
            use core::fmt::Write as _;
            writeln!(&mut $crate::rtt::RTT, $($args),*).unwrap();
        }
    };
}
