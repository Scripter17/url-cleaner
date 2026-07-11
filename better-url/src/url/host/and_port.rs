//! HostPort.

use crate::prelude::*;

impl BetterUrl {
    /// The host and port.
    ///
    /// Basically only used by [`Self::canon_get_host`].
    pub fn host_port_str(&self) -> Option<&str> {
        Some(&self.serialization[self.host_start?.get() as usize .. self.path_start as usize])
    }

    /// Set the host and, if `port` is [`Some`], the port.
    ///
    /// Basically only used by [`Self::canon_set_host`].
    /// # Errors
    /// If the call to [`Self::set_host`] returns an error, that error is returned.
    ///
    /// If the call to [`Self::set_port`] returns an error, that error is returned.
    pub fn set_host_port<'h, 'p, H: TryInto<FileHost<'h>> + TryInto<SpecialNotFileHost<'h>> + TryInto<NonSpecialHost<'h>>, P: TryInto<MaybePort<'p>>>(&mut self, host: H, port: Option<P>) -> Result<(), SetHostPortError>
        where InvalidHost: From<<H as TryInto<FileHost<'h>>>::Error> + From<<H as TryInto<SpecialNotFileHost<'h>>>::Error> + From<<H as TryInto<NonSpecialHost<'h>>>::Error>, InvalidPort: From<P::Error>
    {
        self.set_host(host)?;

        if let Some(port) = port {
            self.set_port(port)?;
        }

        Ok(())
    }
}
