use crate::error_container::ErrorContainer;
use crate::handle::Handle;

pub struct ParseResult {
    pub doc: Handle,
    pub error_container: ErrorContainer,
}
