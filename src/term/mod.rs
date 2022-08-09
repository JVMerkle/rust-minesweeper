#[cfg(target_os = "linux")]
extern "C" {
    fn tcsetattr_icanon_echo() -> i32;
}

#[derive(Debug)]
#[allow(unused)]
pub enum UnbufferError {
    TcGetAttrFailed,
    TcSetAttrFailed,
    PlatformNotSupported,
}

pub fn unbuffer_stdin() -> Result<(), UnbufferError> {
    #[cfg(target_os = "linux")]
    {
        return match unsafe { tcsetattr_icanon_echo() } {
            -1 => Err(UnbufferError::TcGetAttrFailed),
            -2 => Err(UnbufferError::TcSetAttrFailed),
            _ => Ok(())
        };
    }
    #[cfg(not(target_os = "linux"))]
    {
        Err(UnbufferError::PlatformNotSupported)
    }
}

pub fn clear() {
    #[cfg(target_os = "linux")]
    {
        print!("\n\u{001b}c");
    }
    #[cfg(not(target_os = "linux"))]
    {
        println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    }
}
