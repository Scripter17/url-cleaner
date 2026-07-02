//! Components.

use crate::prelude::*;

/** [`Condition::Extern`].          **/ pub type ConditionExtern          =             fn(&    TaskState   , Option<&   FunctionArgs>                           ) -> Result<bool                , ConditionError         >;
/** [`Action::Extern`].             **/ pub type ActionExtern             =             fn(&mut TaskState   , Option<&   FunctionArgs>                           ) -> Result<bool                , ActionError            >;
/** [`StringMatcher::Extern`].      **/ pub type StringMatcherExtern      =             fn(&   TaskState    , Option<&   FunctionArgs>,      Option<&str>        ) -> Result<bool                , StringMatcherError     >;
/** [`StringModification::Extern`]. **/ pub type StringModificationExtern = for<'j, 't> fn(&'t TaskState<'j>, Option<&'j FunctionArgs>, &mut Option<Cow<'t, str>>) -> Result<bool                , StringModificationError>;
/** [`StringSource::Extern`].       **/ pub type StringSourceExtern       = for<'j, 't> fn(&'t TaskState<'j>, Option<&'j FunctionArgs>                           ) -> Result<Option<Cow<'t, str>>, StringSourceError      >;

mod string_source       ; pub use string_source       ::*;
mod string_modification ; pub use string_modification ::*;
mod string_location     ; pub use string_location     ::*;
mod string_matcher      ; pub use string_matcher      ::*;
mod char_matcher        ; pub use char_matcher        ::*;
mod action              ; pub use action              ::*;
mod condition           ; pub use condition           ::*;
mod url_part            ; pub use url_part            ::*;
mod host_part           ; pub use host_part           ::*;
mod query_param_selector; pub use query_param_selector::*;
