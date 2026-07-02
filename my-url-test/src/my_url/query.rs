use crate::prelude::*;

impl MyUrl {
    fn query_start(&self) -> Option<usize> {
        Some(self.query_start?.get() as usize + 1)
    }

    fn query_after(&self) -> Option<usize> {
        Some(self.fragment_start.map_or(self.len(), |x| x.get() as usize))
    }

    fn query_range(&self) -> Option<Range<usize>> {
        Some(self.query_start()? .. self.query_after()?)
    }

    /// The query as a [`str`].
    pub fn query(&self) -> Option<&str> {
        Some(&self.serialization[self.query_range()?])
    }

    pub fn set_query<'a, T: Into<MaybeSpecialQuery<'a>> + Into<MaybeNonSpecialQuery<'a>>>(&mut self, value: T) -> Result<bool, SetQueryError> {
        let old = self.query().zip(self.query_range());
        let new = match self.details.scheme.is_special() {
            true  => MaybeQuery::new_special    (value),
            false => MaybeQuery::new_non_special(value),
        };

        Ok(match (old, new.as_str()) {
            (None               , None     )               => false,
            (Some((old, _     )), Some(new)) if old == new => false,

            (None               , Some(new)) if self.len() + new.len() + 1         > u32::MAX as usize => Err(TooLong)?,
            (Some((old, _     )), Some(new)) if self.len() - old.len() + new.len() > u32::MAX as usize => Err(TooLong)?,

            (None               , Some(new)) => {
                match self.fragment_start() {
                    Some(x) => todo!(),
                    None => {
                        self.query_start = NonZero::new(self.len() as u32);
                        self.serialization.extend(["?", new]);
                        true
                    }
                }
            },
            (Some((old, orange)), None     ) => todo!(),
            (Some((old, orange)), Some(new)) => todo!(),
        })
    }
}
