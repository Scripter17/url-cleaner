//! Process stuff.

/// SIGTERMs the [`std::process::Child`] on [`Drop`].
#[derive(Debug)]
pub struct TerminateOnDrop(pub std::process::Child);

impl Drop for TerminateOnDrop {
    fn drop(&mut self) {
        if self.0.try_wait().unwrap().is_none() {
            cfg_select! {
                target_family = "windows" => assert_eq!(std::process::Command::new("taskkill").arg("/pid").arg(self.0.id().to_string()).spawn().unwrap().wait().unwrap().code(), Some(0)),
                _ => unsafe {libc::kill(self.0.id() as _, libc::SIGTERM);}
            }
            assert_eq!(self.0.wait().unwrap().code(), None);
        }
    }
}
