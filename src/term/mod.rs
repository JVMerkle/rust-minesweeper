#[cfg(target_os = "linux")]
extern "C" {
    fn tcsetattr_icanon_echo() -> i32;
}

#[derive(Debug)]
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
        }
    }
    // else
    Err(UnbufferError::PlatformNotSupported)
}

pub fn clear() {
    #[cfg(target_os = "linux")]
    {
        print!("\n\u{001b}c");
    }
    // else
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
}
