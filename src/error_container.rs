use std::borrow::Cow;

#[repr(C)]
pub struct CError {
    line: libc::size_t,
    msg_len: libc::size_t,
    msg: *const libc::__u8,
}

pub struct Error {
    line: u64,
    msg: Cow<'static, str>,
}

pub struct ErrorContainer {
    errors: Vec<Error>,
}

impl ErrorContainer {
    pub fn new() -> Self {
        Self {
            errors: Default::default(),
        }
    }

    pub fn add(&mut self, line: u64, msg: Cow<'static, str>) {
        self.errors.push(Error { line, msg });
    }

    pub fn count(&self) -> usize {
        self.errors.len()
    }

    pub fn get(&self, index: usize) -> Option<&Error> {
        self.errors.get(index)
    }
}

impl Error {
    pub fn as_c_error(&self) -> CError {
        CError {
            line: self.line as _,
            msg_len: self.msg.len() as _,
            msg: self.msg.as_ptr(),
        }
    }
}
