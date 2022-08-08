extern "C" {
    fn tcsetattr_icanon_echo() -> i32;
}

#[derive(Debug)]
pub enum UnbufferError {
    TcGetAttrFailed,
    TcSetAttrFailed,
}

pub fn unbuffer_stdin() -> Result<(), UnbufferError> {
    let retval = unsafe {
        tcsetattr_icanon_echo()
    };

    match retval {
        -1 => Err(UnbufferError::TcGetAttrFailed),
        -2 => Err(UnbufferError::TcSetAttrFailed),
        _ => Ok(())
    }
}

pub fn clear() {
    print!("\n\u{001b}c");
}
