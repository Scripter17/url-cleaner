use crate::prelude::*;

mod signed;
mod unsigned;

impl BidiDetails {    
    /// Attempt to append a [`BidiDetail`].
    /// # Errors
    /// See [#validity] for details.
    pub fn try_push(&mut self, detail: BidiDetail) -> Result<(), InvalidDomainSegments> {
        match (self.is_bidi(), detail) {
            (false, BidiDetail::Ltr) => self.0.push(false),
            (false, BidiDetail::Rtl) => {
                if !self.0.all_false() {
                    Err(InvalidDomainSegments)?;
                }
                unsafe {self.0.set_unchecked(0, true);}
                self.0.push(true);
            },
            (false, BidiDetail::Inv) => self.0.push(true),

            (true, BidiDetail::Ltr) => self.0.push(false),
            (true, BidiDetail::Rtl) => self.0.push(true ),
            (true, BidiDetail::Inv) => Err(InvalidDomainSegments)?,
        }

        Ok(())
    }
    /// Remove the last [`BidiDetail`].
    pub fn pop(&mut self) -> Option<BidiDetail> {
        if self.is_empty() {
            None?;
        }

        Some(match (self.is_bidi(), self.0.pop()?) {
            (_    , false) => BidiDetail::Ltr,
            (false, true ) => BidiDetail::Inv,
            (true , true ) => {
                if self.count_ones() == 0 {
                    unsafe {
                        self.0.set_unchecked(0, false);
                    }
                }
                BidiDetail::Rtl
            },
        })
    }
}
