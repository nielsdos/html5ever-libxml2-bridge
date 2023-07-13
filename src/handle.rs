#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct Handle(pub *mut libc::c_void);

impl Handle {
    pub fn as_raw(self) -> *mut libc::c_void {
        self.0
    }
}
