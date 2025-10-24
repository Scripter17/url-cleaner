//! [`TaskCow`].

use std::ops::Deref;
use std::borrow::{Borrow, Cow};
use std::fmt::Debug;

use serde::{Serialize, Serializer, Deserialize, Deserializer};

use crate::prelude::*;

/// Allows components to only allocate when they need to.
pub enum TaskCow<'j: 't, 't: 'c, 'c, T: ToOwned + ?Sized> {
    /// Owned.
    Owned(<T as ToOwned>::Owned),
    Job  (&'j T),
    Task (&'t T),
    Call (&'c T)
}

impl<'j: 't, 't: 'c, 'c, T: ToOwned + ?Sized + Serialize> Serialize for TaskCow<'j, 't, 'c, T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        (**self).serialize(serializer)
    }
}

impl<'de, 'j: 't, 't: 'c, 'c, T: ToOwned + ?Sized> Deserialize<'de> for TaskCow<'j, 't, 'c, T> where T::Owned: Deserialize<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        T::Owned::deserialize(deserializer).map(TaskCow::Owned)
    }
}

impl<'j: 't, 't: 'c, 'c, T: ToOwned + ?Sized> Clone for TaskCow<'j, 't, 'c, T> where <T as ToOwned>::Owned: Clone {
    fn clone(&self) -> Self {
        match self {
            Self::Owned(x) => Self::Owned(x.clone()),
            Self::Job  (x) => Self::Job  (x),
            Self::Task (x) => Self::Task (x),
            Self::Call (x) => Self::Call (x)
        }
    }
}

impl<'j> Default for TaskCow<'j, '_, '_, str> {
    fn default() -> Self {
        Self::Job("")
    }
}

impl<'j: 't, 't: 'c, 'c> FromIterator<TaskCow<'j, 't, 'c, str>> for String {
    fn from_iter<I: IntoIterator<Item = TaskCow<'j, 't, 'c, str>>>(iter: I) -> String {
        let mut iterator = iter.into_iter();

        match iterator.next() {
            None => String::new(),
            Some(cow) => {
                let mut buf = cow.into_owned();
                buf.extend(iterator);
                buf
            }
        }
    }
}

impl<'j: 't, 't: 'c, 'c> Extend<TaskCow<'j, 't, 'c, str>> for String {
    fn extend<T: IntoIterator<Item = TaskCow<'j, 't, 'c, str>>>(&mut self, iter: T) {
        iter.into_iter().for_each(move |s| self.push_str(&s))
    }
}

impl<T: ToOwned + ?Sized> std::fmt::Debug for TaskCow<'_, '_, '_, T> where T: Debug, T::Owned: Debug {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Owned(x) => formatter.debug_tuple("Owned").field(&x).finish(),
            Self::Job  (x) => formatter.debug_tuple("Job"  ).field(&x).finish(),
            Self::Task (x) => formatter.debug_tuple("Task" ).field(&x).finish(),
            Self::Call (x) => formatter.debug_tuple("Call" ).field(&x).finish()
        }
    }
}

impl<T: ToOwned + ?Sized> Deref for TaskCow<'_, '_, '_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Owned(x) => (*x).borrow(),
            Self::Job  (x) => x,
            Self::Task (x) => x,
            Self::Call (x) => x
        }
    }
}

impl<T: ToOwned + ?Sized> AsRef<T> for TaskCow<'_, '_, '_, T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T: ToOwned + ?Sized, U: ToOwned + ?Sized> PartialEq<TaskCow<'_, '_, '_, U>> for TaskCow<'_, '_, '_, T> where T: PartialEq<U> {
    fn eq(&self, other: &TaskCow<'_, '_, '_, U>) -> bool {
        **self == **other
    }
}
impl<T: ToOwned + ?Sized + Eq> Eq for TaskCow<'_, '_, '_, T> {}

impl<T: ToOwned + ?Sized, U: ToOwned + ?Sized> PartialEq<Cow<'_, U>> for TaskCow<'_, '_, '_, T> where T: PartialEq<U> {
    fn eq(&self, other: &Cow<'_, U>) -> bool {
        **self == **other
    }
}

impl<T: ToOwned + ?Sized, U: ToOwned + ?Sized> PartialEq<TaskCow<'_, '_, '_, U>> for Cow<'_, T> where T: PartialEq<U> {
    fn eq(&self, other: &TaskCow<'_, '_, '_, U>) -> bool {
        **self == **other
    }
}

impl<'j: 't, 't: 'c, 'c, T: ToOwned + ?Sized + Debug> TaskCow<'j, 't, 'c, T> where T::Owned: Debug {
    pub fn into_owned(self) -> <T as ToOwned>::Owned {
        debug!(TaskCow::into_owned, &self);

        match self {
            Self::Owned(x) => x,
            Self::Job  (x) => x.to_owned(),
            Self::Task (x) => x.to_owned(),
            Self::Call (x) => x.to_owned()
        }
    }

    pub fn as_mut(&mut self) -> &mut T::Owned {
        debug!(TaskCow::as_mut, &self);

        match self {
            Self::Owned(x) => x,
            Self::Job  (x) => {*self = Self::Owned(x.to_owned()); self.as_mut()},
            Self::Task (x) => {*self = Self::Owned(x.to_owned()); self.as_mut()},
            Self::Call (x) => {*self = Self::Owned(x.to_owned()); self.as_mut()}
        }
    }

    pub fn as_job(&'j mut self) -> &'j T {
        debug!(TaskCow::as_job, &self);

        match self {
            Self::Owned(x) => (*x).borrow(),
            Self::Job  (x) => x,
            Self::Task (x) => {*self = Self::Owned(x.to_owned()); self.as_job()},
            Self::Call (x) => {*self = Self::Owned(x.to_owned()); self.as_job()}
        }
    }

    pub fn as_task(&'t mut self) -> &'t T {
        debug!(TaskCow::as_task, &self);

        match self {
            Self::Owned(x) => (*x).borrow(),
            Self::Job  (x) => x,
            Self::Task (x) => x,
            Self::Call (x) => {*self = Self::Owned(x.to_owned()); self.as_task()}
        }
    }

    pub fn as_call(&'c mut self) -> &'c T {
        match self {
            Self::Owned(x) => (*x).borrow(),
            Self::Job  (x) => x,
            Self::Task (x) => x,
            Self::Call (x) => x
        }
    }

    pub fn into_job_cow(self) -> Cow<'j, T> {
        debug!(TaskCow::into_job_cow, &self);

        match self {
            Self::Owned(x) => Cow::Owned   (x),
            Self::Job  (x) => Cow::Borrowed(x),
            Self::Task (x) => Cow::Owned   (x.to_owned()),
            Self::Call (x) => Cow::Owned   (x.to_owned())
        }
    }

    pub fn into_task_cow(self) -> Cow<'t, T> {
        debug!(TaskCow::into_task_cow, &self);

        match self {
            Self::Owned(x) => Cow::Owned   (x),
            Self::Job  (x) => Cow::Borrowed(x),
            Self::Task (x) => Cow::Borrowed(x),
            Self::Call (x) => Cow::Owned   (x.to_owned())
        }
    }

    pub fn into_call_cow(self) -> Cow<'c, T> {
        match self {
            Self::Owned(x) => Cow::Owned(x),
            Self::Job  (x) => Cow::Borrowed(x),
            Self::Task (x) => Cow::Borrowed(x),
            Self::Call (x) => Cow::Borrowed(x)
        }
    }

    pub fn into_job_task_cow<'a>(self) -> TaskCow<'j, 'a, 'a, T> {
        debug!(TaskCow::into_job_task_cow, &self);

        match self {
            Self::Owned(x) => TaskCow::Owned(x),
            Self::Job  (x) => TaskCow::Job  (x),
            Self::Task (x) => TaskCow::Owned(x.to_owned()),
            Self::Call (x) => TaskCow::Owned(x.to_owned())
        }
    }

    pub fn into_task_task_cow<'a>(self) -> TaskCow<'j, 't, 'a, T> {
        debug!(TaskCow::into_task_task_cow, &self);

        match self {
            Self::Owned(x) => TaskCow::Owned(x),
            Self::Job  (x) => TaskCow::Job  (x),
            Self::Task (x) => TaskCow::Task (x),
            Self::Call (x) => TaskCow::Owned(x.to_owned())
        }
    }

    pub fn from_job_cow(value: Cow<'j, T>) -> Self {
        match value {
            Cow::Owned   (x) => Self::Owned(x),
            Cow::Borrowed(x) => Self::Job  (x)
        }
    }

    pub fn from_task_cow(value: Cow<'t, T>) -> Self {
        match value {
            Cow::Owned   (x) => Self::Owned(x),
            Cow::Borrowed(x) => Self::Task (x)
        }
    }

    pub fn from_call_cow(value: Cow<'c, T>) -> Self {
        match value {
            Cow::Owned   (x) => Self::Owned(x),
            Cow::Borrowed(x) => Self::Call (x)
        }
    }
}
