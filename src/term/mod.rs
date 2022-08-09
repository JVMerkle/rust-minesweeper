#[cfg(target_os = "linux")]
extern "C" {
    fn termios_icanon_echo() -> i32;
    fn termios_revert() -> i32;
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
            Self::parse_terminal_result(unsafe { termios_icanon_echo() })
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err(UnbufferError::PlatformNotSupported)
        }
    }

    fn parse_terminal_result(result: i32) -> TerminalResult {
        match result {
            0 => Ok(()),
            -1 => Err(UnbufferError::TcGetAttrFailed),
            -2 => Err(UnbufferError::TcSetAttrFailed),
            _ => Err(UnbufferError::Unknown),
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

impl Drop for FastTerm {
    fn drop(&mut self) {
        if let Ok(_) = self.self_test_result {
            if let Err(e) = Self::parse_terminal_result(unsafe { termios_revert() }) {
                eprintln!("Could not revert terminal settings: {:?}", e);
            }
        }
    }
}
