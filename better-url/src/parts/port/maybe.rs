//! [`MaybePort`].

use crate::prelude::*;

/// A maybe port.
///
/// When possible, impls like [`PartialEq`] and [`Hash`] work on [`Self::as_num`] instead of [`Self::as_str`].
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct MaybePort<'a>(pub Option<Port<'a>>);

impl<'a> MaybePort<'a> {
    /// Make a new [`Self`] without doing any validity checks.
    /// # Safety
    /// `value` must be either [`None`] or a valid pair of a [`Self`] literal its number.
    pub unsafe fn new_unchecked<T: Into<Cow<'a, str>>>(value: Option<(T, u16)>) -> Self {
        Self(value.map(|(s, n)| unsafe {Port::new_unchecked(s, n)}))
    }

    /// Make a new [`Self`].
    /// # Errors
    /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new<T: TryInto<Self>>(value: T) -> Result<Self, T::Error> {
        value.try_into()
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> Option<&str> {
        self.0.as_ref().map(Port::as_str)
    }

    /// Get it as a [`u16`].
    pub fn as_num(&self) -> Option<u16> {
        self.0.as_ref().map(Port::as_num)
    }

    /// If it's [`Some`].
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    /// If it's [`None`].
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    /// The length
    #[expect(clippy::len_without_is_empty, reason = "Can't be empty.")]
    pub fn len(&self) -> Option<usize> {
        self.0.as_ref().map(Port::len)
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> MaybePort<'_> {
        MaybePort(self.0.as_ref().map(Port::borrowed))
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> MaybePort<'static> {
        MaybePort(self.0.map(Port::into_owned))
    }

    /// Turn into the inner [`Cow`] and [`u16`].
    pub fn into_parts(self) -> Option<(Cow<'a, str>, u16)> {
        self.0.map(Port::into_parts)
    }
}



impl<'a> TryFrom<Option<Cow<'a, str>>> for MaybePort<'a> {
    type Error = InvalidPort;

    fn try_from(value: Option<Cow<'a, str>>) -> Result<Self, Self::Error> {
        Ok(Self(value.map(TryInto::try_into).transpose()?))
    }
}

impl<'a> TryFrom<Cow<'a, str>> for MaybePort<'a> {
    type Error = InvalidPort;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Some(value).try_into()
    }
}

impl<'a> TryFrom<Option<&'a str>> for MaybePort<'a> {
    type Error = InvalidPort;

    fn try_from(value: Option<&'a str>) -> Result<Self, Self::Error> {
        value.map(Cow::from).try_into()
    }
}

impl<'a> TryFrom<&'a str> for MaybePort<'a> {
    type Error = InvalidPort;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Some(value).try_into()
    }
}

impl TryFrom<Option<String>> for MaybePort<'static> {
    type Error = InvalidPort;

    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        value.map(Cow::from).try_into()
    }
}

impl TryFrom<String> for MaybePort<'static> {
    type Error = InvalidPort;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Some(value).try_into()
    }
}

impl FromStr for MaybePort<'static> {
    type Err = InvalidPort;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MaybePort::try_from(s)?.into_owned())
    }
}



impl From<u16> for MaybePort<'static> {
    fn from(value: u16) -> Self {
        Self(Some(value.into()))
    }
}

impl From<Option<u16>> for MaybePort<'static> {
    fn from(value: Option<u16>) -> Self {
        Self(value.map(Into::into))
    }
}



impl PartialEq for MaybePort<'_> {fn eq(&self, other: &Self) -> bool {self.as_num() == other.as_num() }}
impl Eq for MaybePort<'_> {}

impl PartialEq<       u16 > for MaybePort<'_> {fn eq(&self, other: &u16        ) -> bool {self.as_num() == Some(*other)}}
impl PartialEq<Option<u16>> for MaybePort<'_> {fn eq(&self, other: &Option<u16>) -> bool {self.as_num() ==      *other }}

impl PartialEq<       Port<'_> > for MaybePort<'_> {fn eq(&self, other: &       Port<'_> ) -> bool {self.as_num() == Some(other.as_num())}}
impl PartialEq<Option<Port<'_>>> for MaybePort<'_> {fn eq(&self, other: &Option<Port<'_>>) -> bool {self.as_num() == other.as_ref().map(Port::as_num)}}

impl PartialOrd for MaybePort<'_> {fn partial_cmp(&self, other: &Self) -> Option<Ordering> {Some(self.cmp(other))}}
impl Ord        for MaybePort<'_> {fn cmp        (&self, other: &Self) ->        Ordering  {self.as_num().cmp(&other.as_num())}}

impl PartialOrd<       u16 > for MaybePort<'_> {fn partial_cmp(&self, other: &       u16 ) -> Option<Ordering> {self.as_num().partial_cmp(&Some(*other))}}
impl PartialOrd<Option<u16>> for MaybePort<'_> {fn partial_cmp(&self, other: &Option<u16>) -> Option<Ordering> {self.as_num().partial_cmp(       other) }}

impl PartialOrd<       Port<'_> > for MaybePort<'_> {fn partial_cmp(&self, other: &       Port<'_> ) -> Option<Ordering> {self.as_num().partial_cmp(&Some(other.as_num()))}}
impl PartialOrd<Option<Port<'_>>> for MaybePort<'_> {fn partial_cmp(&self, other: &Option<Port<'_>>) -> Option<Ordering> {self.as_num().partial_cmp(&other.as_ref().map(Port::as_num))}}

impl Hash for MaybePort<'_> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.as_num().hash(hasher)
    }
}

#[cfg(feature = "serde")]
impl Serialize for MaybePort<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for MaybePort<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self(<Option<Port<'de>>>::deserialize(deserializer)?))
    }
}
