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
    Unknown,
}

type TerminalResult = Result<(), UnbufferError>;

pub struct FastTerm {
    self_test_result: TerminalResult,
}

impl FastTerm {
    pub fn new() -> Self {
        Self {
            self_test_result: Self::unbuffer_stdin(),
        }
    }

    fn unbuffer_stdin() -> TerminalResult {
        #[cfg(target_os = "linux")]
        {
            match unsafe { tcsetattr_icanon_echo() } {
                0 => Ok(()),
                -1 => Err(UnbufferError::TcGetAttrFailed),
                -2 => Err(UnbufferError::TcSetAttrFailed),
                _ => Err(UnbufferError::Unknown),
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err(UnbufferError::PlatformNotSupported)
        }
    }

    pub fn clear(&self) {
        #[cfg(target_os = "linux")]
        if let Ok(_) = self.self_test_result {
            Self::terminal_ansi_clear();
        } else {
            Self::terminal_agnostic_clear();
        }
        #[cfg(not(target_os = "linux"))]
        {
            Self::terminal_agnostic_clear();
        }
    }

    fn terminal_ansi_clear() {
        print!("\u{001b}c");
    }

    fn terminal_agnostic_clear() {
        println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    }
}
