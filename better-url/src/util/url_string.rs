//! [`UrlString`].

use std::ptr::NonNull;
use std::mem::ManuallyDrop;

use crate::prelude::*;

/// A [`String`] with a length and capacity defined as [`u32`]s, saving a total 8 bytes on 64 bit systems.
///
/// Used in [`BetterUrl`] since URLs can never be longer than 4GiB.
/// # Examples
/// ```
/// use better_url::util::*;
///
/// assert!(std::mem::size_of::<UrlString>() <= 16);
/// ```
pub struct UrlString {
    /// The data.
    ptr: NonNull<u8>,
    /// The length.
    len: u32,
    /// The capacity.
    cap: u32,
}

impl UrlString {
    /// Make a new [`UrlString`].
    /// # Safety
    /// The [`String`] must have a capacity below [`u32::MAX`].
    pub unsafe fn new_unchecked(value: String) -> Self {
        let (ptr, len, cap) = value.into_raw_parts();

        unsafe {
            Self {
                ptr: NonNull::new_unchecked(ptr),
                len: len.try_into().unwrap_unchecked(),
                cap: cap.try_into().unwrap_unchecked(),
            }
        }
    }

    /// Make a temporary [`String`] to modify.
    ///
    /// `f` is allowed to reallocate and even grow beyond [`u32::MAX`], as long at the end the capacity isn't above [`u32::MAX`].
    ///
    /// Additionally, `f` is, I think, allowed to panic.
    /// # Safety
    /// The [`String`] must not end up with a capacity above [`u32::MAX`].
    pub unsafe fn modify<F: FnOnce(&mut String)>(&mut self, f: F) {
        let mut temp = ManuallyDrop::new(unsafe {String::from_raw_parts(self.ptr.as_ptr(), self.len as _, self.cap as _)});

        f(&mut temp);

        let (ptr, len, cap) = ManuallyDrop::into_inner(temp).into_raw_parts();

        unsafe {
            self.ptr = NonNull::new_unchecked(ptr);
            self.len = len.try_into().unwrap_unchecked();
            self.cap = cap.try_into().unwrap_unchecked();
        }
    }

    /// Borrow as a [`str`].
    pub fn as_str(&self) -> &str {
        unsafe {
            str::from_utf8_unchecked(self.as_bytes())
        }
    }

    /// Borrow as a mutable [`str`].
    pub fn as_mut_str(&mut self) -> &mut str {
        unsafe {
            str::from_utf8_unchecked_mut(self.as_bytes_mut())
        }
    }

    /// Borrow as a [`slice`] of [`u8`].
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.ptr.as_ptr(), self.len as _)
        }
    }

    /// Borrow as a mutable [`slice`] of [`u8`].
    /// # Safety
    /// The [`slice`] must be valid UTF-8 at the end.
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len as _)
        }
    }

    /// The length.
    pub fn len(&self) -> u32 {
        self.len
    }

    /// If it's empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// The capacity.
    pub fn capacity(&self) -> u32 {
        self.cap
    }
}



impl From<UrlString> for String {
    fn from(value: UrlString) -> Self {
        let ret = unsafe {String::from_raw_parts(value.ptr.as_ptr(), value.len as _, value.cap as _)};
        std::mem::forget(value);
        ret
    }
}

impl TryFrom<String> for UrlString {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.capacity() > u32::MAX as usize {
            Err(value)
        } else {
            Ok(unsafe {
                Self::new_unchecked(value)
            })
        }
    }
}

impl std::ops::Drop for UrlString {
    fn drop(&mut self) {
        unsafe {String::from_raw_parts(self.ptr.as_ptr(), self.len as _, self.cap as _)};
    }
}



unsafe impl Send for UrlString {}
unsafe impl Sync for UrlString {}



impl std::fmt::Debug for UrlString {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{:?}", self.as_str())
    }
}

impl std::fmt::Display for UrlString {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl Clone for UrlString {
    fn clone(&self) -> Self {
        unsafe {
            let temp = ManuallyDrop::new(String::from_raw_parts(self.ptr.as_ptr(), self.len as _, self.cap as _));
            let ret = temp.clone();
            Self::new_unchecked(ManuallyDrop::into_inner(ret))
        }
    }
}



impl std::ops::Deref for UrlString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl std::ops::DerefMut for UrlString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_str()
    }
}

impl AsRef    <str> for UrlString {fn as_ref    (&    self) -> &    str {self}}
impl Borrow   <str> for UrlString {fn borrow    (&    self) -> &    str {self}}
impl AsMut    <str> for UrlString {fn as_mut    (&mut self) -> &mut str {self}}
impl BorrowMut<str> for UrlString {fn borrow_mut(&mut self) -> &mut str {self}}



impl PartialEq for UrlString {fn eq(&self, other: &Self) -> bool {self.as_str() == other.as_str()}}
impl Eq for UrlString {}

impl PartialEq< str         > for UrlString {fn eq(&self, other: & str         ) -> bool {self.as_str() ==     other}}
impl PartialEq<&str         > for UrlString {fn eq(&self, other: &&str         ) -> bool {self.as_str() ==    *other}}
impl PartialEq< String      > for UrlString {fn eq(&self, other: & String      ) -> bool {self.as_str() == & **other}}
impl PartialEq<&String      > for UrlString {fn eq(&self, other: &&String      ) -> bool {self.as_str() == &***other}}
impl PartialEq< Cow<'_, str>> for UrlString {fn eq(&self, other: & Cow<'_, str>) -> bool {self.as_str() == & **other}}
impl PartialEq<&Cow<'_, str>> for UrlString {fn eq(&self, other: &&Cow<'_, str>) -> bool {self.as_str() == &***other}}

impl PartialEq<UrlString> for  str          {fn eq(&self, other: &UrlString) -> bool {other == self}}
impl PartialEq<UrlString> for &str          {fn eq(&self, other: &UrlString) -> bool {other == self}}
impl PartialEq<UrlString> for  String       {fn eq(&self, other: &UrlString) -> bool {other == self}}
impl PartialEq<UrlString> for &String       {fn eq(&self, other: &UrlString) -> bool {other == self}}
impl PartialEq<UrlString> for  Cow<'_, str> {fn eq(&self, other: &UrlString) -> bool {other == self}}
impl PartialEq<UrlString> for &Cow<'_, str> {fn eq(&self, other: &UrlString) -> bool {other == self}}



impl PartialOrd for UrlString {fn partial_cmp(&self, other: &Self) -> Option<Ordering> {Some(self.cmp(other))}}
impl Ord for UrlString {fn cmp(&self, other: &Self) -> Ordering {self.as_str().cmp(other.as_str())}}

impl PartialOrd< str         > for UrlString {fn partial_cmp(&self, other: & str         ) -> Option<Ordering> {self.as_str().partial_cmp(    other)}}
impl PartialOrd<&str         > for UrlString {fn partial_cmp(&self, other: &&str         ) -> Option<Ordering> {self.as_str().partial_cmp(   *other)}}
impl PartialOrd< String      > for UrlString {fn partial_cmp(&self, other: & String      ) -> Option<Ordering> {self.as_str().partial_cmp(& **other)}}
impl PartialOrd<&String      > for UrlString {fn partial_cmp(&self, other: &&String      ) -> Option<Ordering> {self.as_str().partial_cmp(&***other)}}
impl PartialOrd< Cow<'_, str>> for UrlString {fn partial_cmp(&self, other: & Cow<'_, str>) -> Option<Ordering> {self.as_str().partial_cmp(& **other)}}
impl PartialOrd<&Cow<'_, str>> for UrlString {fn partial_cmp(&self, other: &&Cow<'_, str>) -> Option<Ordering> {self.as_str().partial_cmp(&***other)}}

impl PartialOrd<UrlString> for  str          {fn partial_cmp(&self, other: &UrlString) -> Option<Ordering> {other.partial_cmp(self)}}
impl PartialOrd<UrlString> for &str          {fn partial_cmp(&self, other: &UrlString) -> Option<Ordering> {other.partial_cmp(self)}}
impl PartialOrd<UrlString> for  String       {fn partial_cmp(&self, other: &UrlString) -> Option<Ordering> {other.partial_cmp(self)}}
impl PartialOrd<UrlString> for &String       {fn partial_cmp(&self, other: &UrlString) -> Option<Ordering> {other.partial_cmp(self)}}
impl PartialOrd<UrlString> for  Cow<'_, str> {fn partial_cmp(&self, other: &UrlString) -> Option<Ordering> {other.partial_cmp(self)}}
impl PartialOrd<UrlString> for &Cow<'_, str> {fn partial_cmp(&self, other: &UrlString) -> Option<Ordering> {other.partial_cmp(self)}}



impl Hash for UrlString {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.as_str().hash(hasher)
    }
}
