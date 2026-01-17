//! Command stuff.

/// Sends a SIGTERM to the child when dropped.
#[derive(Debug)]
pub struct TerminateOnDrop(pub std::process::Child);

impl std::ops::Drop for TerminateOnDrop {
    fn drop(&mut self) {
        unsafe {
            libc::kill(self.0.id() as _, libc::SIGTERM);
        }
        self.0.wait().unwrap();
    }
}
