//! Implementing path stuff for [`BetterUrl`].

#[expect(unused_imports, reason = "Used in doc comments.")]
use url::Url;

use crate::prelude::*;

impl BetterUrl {
    /// [`Url::set_path`].
    pub fn set_path(&mut self, path: &str) {
        if self.path_str() != path {
            self.url.set_path(path)
        }
    }

    /// [`Url::path`].
    pub fn path_str(&self) -> &str {
        self.url.path()
    }

    /// A [`BetterPath`].
    pub fn path(&self) -> BetterPath<'_> {
        self.path_str().into()
    }

    /// Returns [`true`] if the path has segments.
    pub fn has_path_segments(&self) -> bool {
        self.path_str().starts_with("/")
    }

    /// Set the path segments.
    /// # Errors
    /// If the call to [`Self::has_path_segments`] returns [`false`], returns the error [`OpaquePath`].
    pub fn set_path_segments(&mut self, to: &str) -> Result<(), OpaquePath> {
        if !self.has_path_segments() {
            return Err(OpaquePath);
        }

        if to.starts_with("/") {
            self.set_path(&format!("/{to}"));
        } else {
            self.set_path(to);
        }

        Ok(())
    }

    /// A [`BetterRefPath`].
    pub fn ref_path(&self) -> BetterRefPath<'_> {
        self.path_str().into()
    }

    /// [`Self::path`]'s segments.
    pub fn path_segments_str(&self) -> Option<&str> {
        self.url.path().strip_prefix("/")
    }

    /// A [`BetterPathSegments`].
    pub fn path_segments(&self) -> Option<BetterPathSegments<'_>> {
        self.path_segments_str().map(Into::into)
    }

    /// A [`BetterRefPathSegments`].
    pub fn ref_path_segments(&self) -> Option<BetterRefPathSegments<'_>> {
        self.path_segments_str().map(Into::into)
    }

    /// Modify the path.
    pub fn modify_path<F: FnOnce(&mut BetterPath<'_>)>(&mut self, f: F) {
        let mut path = self.path();

        f(&mut path);

        if self.path_str() != path.as_str() {
            self.set_path(&path.into_string());
        }
    }

    /// Modify the path.
    /// # Errors
    /// If the call to `f` returns an error, that error is returned.
    pub fn try_modify_path<F: FnOnce(&mut BetterPath<'_>) -> Result<(), E>, E>(&mut self, f: F) -> Result<(), E> {
        let mut path = self.path();

        f(&mut path)?;

        if self.path_str() != path.as_str() {
            self.set_path(&path.into_string());
        }

        Ok(())
    }

    /// Modify the path segments.
    /// # Errors
    /// If the call to [`Self::path_segments`] returns [`None`], returns the error [`OpaquePath`].
    pub fn modify_path_segments<F: FnOnce(&mut BetterPathSegments<'_>)>(&mut self, f: F) -> Result<(), OpaquePath> {
        let mut path_segments = self.path_segments().ok_or(OpaquePath)?;

        f(&mut path_segments);

        if self.path_segments_str() != Some(path_segments.as_str()) {
            self.set_path(&path_segments.into_path_string());
        }

        Ok(())
    }

    /// Modify the path segments.
    /// # Errors
    /// If the call to [`Self::path_segments`] returns [`None`], returns the error [`OpaquePath`].
    ///
    /// If the call to `f` returns an error, that error is returned.
    pub fn try_modify_path_segments<F: FnOnce(&mut BetterPathSegments<'_>) -> Result<(), E>, E>(&mut self, f: F) -> Result<Result<(), E>, OpaquePath> {
        let mut path_segments = self.path_segments().ok_or(OpaquePath)?;

        if let Err(e) = f(&mut path_segments) {
            return Ok(Err(e));
        }

        if self.path_segments_str() != Some(path_segments.as_str()) {
            self.set_path(&path_segments.into_path_string());
        }

        Ok(Ok(()))
    }
}
