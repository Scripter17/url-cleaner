//! [`Port`].

use crate::prelude::*;

/// A port.
///
/// When possible, impls like [`PartialEq`] and [`Hash`] work on [`Self::as_num`] instead of [`Self::as_str`].
#[derive(Debug, Clone)]
pub struct Port<'a> {
    /// The port as a string.
    pub(crate) port: Cow<'a, str>,
    /// The port as a [`u16`].
    pub(crate) port_num: u16
}

impl<'a> Port<'a> {
    /// Make a new [`Self`].
    /// # Errors
    /// If the call to [`TryInto::try_into`] returns an error, that error is returned.
    pub fn new<T: TryInto<Self>>(value: T) -> Result<Self, T::Error> {
        value.try_into()
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        &self.port
    }

    /// Get it as a [`u16`].
    pub fn as_num(&self) -> u16 {
        self.port_num
    }

    /// The length
    #[expect(clippy::len_without_is_empty, reason = "Can't be empty.")]
    pub fn len(&self) -> usize {
        self.port.len()
    }



    /// Make a borrowing [`Self`].
    pub fn borrowed(&self) -> Port<'_> {
        Port {
            port    : Cow::Borrowed(&self.port),
            port_num: self.port_num
        }
    }

    /// Turn into an owned [`Self`].
    pub fn into_owned(self) -> Port<'static> {
        Port {
            port    : self.port.into_owned().into(),
            port_num: self.port_num
        }
    }

    /// Turn into the inner [`Cow`] and [`u16`].
    pub fn into_parts(self) -> (Cow<'a, str>, u16) {
        (self.port, self.port_num)
    }
}



impl<'a> TryFrom<Cow<'a, str>> for Port<'a> {
    type Error = InvalidPort;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        let (_, num, value) = make_port(value)?;

        Ok(Self {
            port: value,
            port_num: num,
        })
    }
}

impl<'a> TryFrom<&'a str> for Port<'a> {
    type Error = InvalidPort;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Cow::from(value).try_into()
    }
}

impl TryFrom<String> for Port<'static> {
    type Error = InvalidPort;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Cow::from(value).try_into()
    }
}

impl FromStr for Port<'static> {
    type Err = InvalidPort;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Port::try_from(value)?.into_owned())
    }
}



impl From<u16> for Port<'static> {
    fn from(value: u16) -> Self {
        Self {
            port: value.to_string().into(),
            port_num: value
        }
    }
}



impl PartialEq for Port<'_> {fn eq(&self, other: &Self) -> bool {self.as_num() == other.as_num()}}
impl Eq for Port<'_> {}

impl PartialEq<u16     > for Port<'_> {fn eq(&self, other: &u16     ) -> bool {self.as_num() == *other}}
impl PartialEq<Port<'_>> for u16      {fn eq(&self, other: &Port<'_>) -> bool {*self == other.as_num()}}

impl PartialEq<MaybePort<'_>> for Port<'_> {fn eq(&self, other: &MaybePort<'_>) -> bool {Some(self.as_num()) ==  other.as_num()}}


impl PartialOrd for Port<'_> {fn partial_cmp(&self, other: &Self) -> Option<Ordering> {Some(self.cmp(other))             }}
impl Ord        for Port<'_> {fn cmp        (&self, other: &Self) ->        Ordering  {self.as_num().cmp(&other.as_num())}}

impl PartialOrd<u16> for Port<'_> {fn partial_cmp(&self, other: &u16     ) -> Option<Ordering> {self.as_num().partial_cmp(other)}}
impl PartialOrd<Port<'_>> for u16 {fn partial_cmp(&self, other: &Port<'_>) -> Option<Ordering> {other.as_num().partial_cmp(self)}}


impl AsRef <str> for Port<'_> {fn as_ref(&self) -> &str {self.as_str()}}
impl Borrow<str> for Port<'_> {fn borrow(&self) -> &str {self.as_str()}}

impl AsRef <u16> for Port<'_> {fn as_ref(&self) -> &u16 {&self.port_num}}
impl Borrow<u16> for Port<'_> {fn borrow(&self) -> &u16 {&self.port_num}}

impl Hash for Port<'_> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.as_num().hash(hasher)
    }
}

impl Display for Port<'_> {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}



#[cfg(feature = "serde")]
impl Serialize for Port<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_num().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Port<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(PortVisitor)
    }
}

/// A [`Visitor`] for [`Port`].
#[cfg(feature = "serde")]
#[derive(Debug)]
struct PortVisitor;

#[cfg(feature = "serde")]
impl<'de> Visitor<'de> for PortVisitor {
    type Value = Port<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "A u16 or string.")
    }

    fn visit_u16<E: serde::de::Error>(self, v: u16) -> Result<Self::Value, E> {
        Ok(v.into())
    }

    fn visit_borrowed_str<E: serde::de::Error>(self, v: &'de str) -> Result<Self::Value, E> {
        v.try_into().map_err(E::custom)
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        v.to_string().try_into().map_err(E::custom)
    }

    fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
        v.try_into().map_err(E::custom)
    }
}
