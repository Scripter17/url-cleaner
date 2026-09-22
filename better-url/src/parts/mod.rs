//! Individual parts of URLs.
//!
//! These APIs are the core advantage of Better URL over most other URL implementations.
//! They allow the various setters on [`BetterUrl`] to often just be a few very cheap checks and a [`String::replace_range`].
//!
//! A nice ergonimic feature of part types is the ability to construct a `Part<'a>` from a `&'a Part<'_>`, as well as a `MaybePart<'a>` from a `Part<'a>`, `Option<Part<'a>>`, `&'a Part<'_>`, `&'a Option<Part<'_>>`, or `Option<&'a Part<'_>>`.
//!
//! ```
//! use better_url::prelude::*;
//!
//! let mut url = BetterUrl::new("https://example.com").unwrap();
//!
//! let mut query = SpecialQuery::new("all the expensive processing happens here");
//!
//! assert_eq!(query, "all%20the%20expensive%20processing%20happens%20here");
//!
//! // Dirt cheap.
//! url.set_query(&query).unwrap();
//!
//! // Because `MaybeQuery<'a>` impls `From<&'a SpecialQuery<'_>>`, this can be done as many times as you want.
//! url.set_query(&query).unwrap();
//!
//! // Though setting the same URL's query to the same value 3 times is a bit redundant.
//! url.set_query(&query).unwrap();
//!
//! // And you can just pass ownership.
//! url.set_query(query).unwrap();
//! ```
//!
//! However please note that the constructors for these types aren't just "what would this input result in when parsing an appropriate URL?" or "what would this input result in in an appropriate URL's setter?".
//!
//! Specifically, tabs, newlines, and carrage returns are not detected or removed, [`Fragment`] and co. don't remove trailing C0-or-space codepoints, and [`OpaquePath`] always percent encodes a trailing space.
//!
//! ```
//! use better_url::prelude::*;
//!
//! let mut url = BetterUrl::new("https://example.com").unwrap();
//!
//! // Implicit conversion into a `SpecialQuery`.
//! url.set_query("abc\tdef").unwrap();
//!
//! assert_eq!(url.query_str(), Some("abc%09def"));
//!
//! // The "canon" APIs do emulate this behavior.
//! url.canon_set_search("abc\tdef").unwrap();
//!
//! assert_eq!(url.query_str(), Some("abcdef"));
//! ```
//!
//! This is because those are stupid. For example it'd require that [`Ipv6Host`] have separate constructors/types for special and non-special URLs.
//! 
//! ```
//! use better_url::prelude::*;
//!
//! let mut url = BetterUrl::new("https://example.com").unwrap();
//!
//! url.canon_set_hostname("[::1]\\removed").unwrap();
//!
//! assert_eq!(url, "https://[::1]/");
//!
//!
//!
//! let mut url = BetterUrl::new("non-special://example.com").unwrap();
//!
//! url.canon_set_hostname("[::1]\\errored").unwrap_err();
//!
//! assert_eq!(url, "non-special://example.com");
//! ```

use crate::prelude::*;

mod scheme;
mod userinfo;
mod host;
mod port;
mod path;
mod query;
mod fragment;

pub use scheme::*;
pub use userinfo::*;
pub use host::*;
pub use port::*;
pub use path::*;
pub use query::*;
pub use fragment::*;



from_cow_bytes!(
    root;

    Userinfo, Username, Password,

    OpaquePath,
    FilePath, FilePathSegments, FilePathSegment,
    SpecialNotFilePath, SpecialNotFilePathSegments, SpecialNotFilePathSegment,
    NonSpecialPath, NonSpecialPathSegments, NonSpecialPathSegment,

    SpecialQuery   , SpecialQuerySegment   , SpecialQueryName   , SpecialQueryValue   ,
    NonSpecialQuery, NonSpecialQuerySegment, NonSpecialQueryName, NonSpecialQueryValue,
    FragmentQuery  , FragmentQuerySegment  , FragmentQueryName  , FragmentQueryValue  ,

    Fragment
);

from_option_cow_bytes!(
    root;

    MaybeFragment,
    MaybeFragmentQuery     , MaybeSpecialQuery     , MaybeNonSpecialQuery     ,
    MaybeFragmentQueryValue, MaybeSpecialQueryValue, MaybeNonSpecialQueryValue
);

try_from_cow_bytes!(
    root;

    Scheme,

    FileHost, SpecialNotFileHost, NonSpecialHost,

    DomainHost, DomainSegments, DomainSegment,
    Ipv4Host, Ipv6Host, OpaqueHost, EmptyHost
);



as_str!(Scheme);

as_str!(Userinfo);
as_str!(Username, Password);

as_str!(
    Host, FileHost, SpecialNotFileHost, NonSpecialHost,
    DomainHost, DomainSegment, DomainSegments,
    Ipv4Host, Ipv6Host, OpaqueHost, EmptyHost
);

as_str!(
    Path,
    OpaquePath, SegmentedPath,
    SpecialNotFilePath, FilePath, NonSpecialPath
);

as_str!(
    PathSegment              , PathSegments              ,
    SpecialNotFilePathSegment, SpecialNotFilePathSegments,
    FilePathSegment          , FilePathSegments          ,
    NonSpecialPathSegment    , NonSpecialPathSegments
);

as_str!(
    Query    , ?MaybeQuery    , QuerySegment    , QueryName    , QueryValue    , ?MaybeQueryValue    ,
    QueryLike, ?MaybeQueryLike, QueryLikeSegment, QueryLikeName, QueryLikeValue, ?MaybeQueryLikeValue,

    SpecialQuery   , ?MaybeSpecialQuery   , SpecialQuerySegment   , SpecialQueryName   , SpecialQueryValue   , ?MaybeSpecialQueryValue   ,
    NonSpecialQuery, ?MaybeNonSpecialQuery, NonSpecialQuerySegment, NonSpecialQueryName, NonSpecialQueryValue, ?MaybeNonSpecialQueryValue,
    FragmentQuery  , ?MaybeFragmentQuery  , FragmentQuerySegment  , FragmentQueryName  , FragmentQueryValue  , ?MaybeFragmentQueryValue  ,

    Fragment, ?MaybeFragment
);

borrowed!(
    Scheme,

    Userinfo,
    Username, Password,

    Host, FileHost, SpecialNotFileHost, NonSpecialHost,
    DomainHost, DomainSegment, DomainSegments,
    Ipv4Host, Ipv6Host, OpaqueHost, EmptyHost,

    Port, MaybePort,

    Fragment, MaybeFragment
);



from_borrowed!(Path              , Path, SegmentedPath, FilePath, SpecialNotFilePath, NonSpecialPath, OpaquePath);
from_borrowed!(OpaquePath        , Path, SegmentedPath, FilePath, SpecialNotFilePath, NonSpecialPath, OpaquePath);
from_borrowed!(SegmentedPath     ,       SegmentedPath, FilePath, SpecialNotFilePath, NonSpecialPath            );
from_borrowed!(FilePath          , Path, SegmentedPath, FilePath, SpecialNotFilePath, NonSpecialPath, OpaquePath);
from_borrowed!(SpecialNotFilePath, Path, SegmentedPath, FilePath, SpecialNotFilePath, NonSpecialPath, OpaquePath);
from_borrowed!(NonSpecialPath    , Path, SegmentedPath, FilePath, SpecialNotFilePath, NonSpecialPath, OpaquePath);


from_borrowed!(
    PathSegments,

    PathSegments, FilePathSegments, SpecialNotFilePathSegments, NonSpecialPathSegments,
    PathSegment , FilePathSegment , SpecialNotFilePathSegment , NonSpecialPathSegment ,
);


from_borrowed!(
    FilePathSegments,

    PathSegments, FilePathSegments, SpecialNotFilePathSegments, NonSpecialPathSegments,
    PathSegment , FilePathSegment , SpecialNotFilePathSegment , NonSpecialPathSegment ,
);


from_borrowed!(
    SpecialNotFilePathSegments,

    PathSegments, FilePathSegments, SpecialNotFilePathSegments, NonSpecialPathSegments,
    PathSegment , FilePathSegment , SpecialNotFilePathSegment , NonSpecialPathSegment ,
);


from_borrowed!(
    NonSpecialPathSegments,

    PathSegments, FilePathSegments, SpecialNotFilePathSegments, NonSpecialPathSegments,
    PathSegment , FilePathSegment , SpecialNotFilePathSegment , NonSpecialPathSegment ,
);

from_borrowed!(PathSegment              , PathSegment, FilePathSegment, SpecialNotFilePathSegment, NonSpecialPathSegment);
from_borrowed!(FilePathSegment          , PathSegment, FilePathSegment, SpecialNotFilePathSegment, NonSpecialPathSegment);
from_borrowed!(SpecialNotFilePathSegment, PathSegment, FilePathSegment, SpecialNotFilePathSegment, NonSpecialPathSegment);
from_borrowed!(NonSpecialPathSegment    , PathSegment, FilePathSegment, SpecialNotFilePathSegment, NonSpecialPathSegment);



from_borrowed!(
    QueryLike,

    QueryLike       , FragmentQuery       , Fragment, Query       , SpecialQuery       , NonSpecialQuery       ,
    QueryLikeSegment, FragmentQuerySegment,           QuerySegment, SpecialQuerySegment, NonSpecialQuerySegment,
);
from_borrowed!(
    FragmentQuery,

    QueryLike       , FragmentQuery       , Fragment, Query       , SpecialQuery,        NonSpecialQuery,
    QueryLikeSegment, FragmentQuerySegment,           QuerySegment, SpecialQuerySegment, NonSpecialQuerySegment,
);
from_borrowed!(
    Query,

    Query       , SpecialQuery       , NonSpecialQuery       ,
    QuerySegment, SpecialQuerySegment, NonSpecialQuerySegment,
);
from_borrowed!(
    NonSpecialQuery,

    QueryLike       , FragmentQuery       , Fragment, Query       , SpecialQuery,        NonSpecialQuery,
    QueryLikeSegment, FragmentQuerySegment,           QuerySegment, SpecialQuerySegment, NonSpecialQuerySegment,
);
from_borrowed!(
    SpecialQuery,

    QueryLike       , FragmentQuery       , Fragment, Query       , SpecialQuery,        NonSpecialQuery,
    QueryLikeSegment, FragmentQuerySegment,           QuerySegment, SpecialQuerySegment, NonSpecialQuerySegment,
);



from_borrowed!(QueryLikeSegment      , QueryLikeSegment, FragmentQuerySegment, QuerySegment, SpecialQuerySegment, NonSpecialQuerySegment);
from_borrowed!(FragmentQuerySegment  , QueryLikeSegment, FragmentQuerySegment, QuerySegment, SpecialQuerySegment, NonSpecialQuerySegment);
from_borrowed!(QuerySegment          ,                                         QuerySegment, SpecialQuerySegment, NonSpecialQuerySegment);
from_borrowed!(SpecialQuerySegment   , QueryLikeSegment, FragmentQuerySegment, QuerySegment, SpecialQuerySegment, NonSpecialQuerySegment);
from_borrowed!(NonSpecialQuerySegment, QueryLikeSegment, FragmentQuerySegment, QuerySegment, SpecialQuerySegment, NonSpecialQuerySegment);


from_borrowed!(
    MaybeQueryLike,

    MaybeQueryLike  , MaybeFragmentQuery  , MaybeFragment, MaybeQuery  , MaybeSpecialQuery  , MaybeNonSpecialQuery  ,
    QueryLike       , FragmentQuery       , Fragment     , Query       , SpecialQuery       , NonSpecialQuery       ,
    QueryLikeSegment, FragmentQuerySegment,                QuerySegment, SpecialQuerySegment, NonSpecialQuerySegment,
);
from_options!(
    MaybeQueryLike,

    FragmentQuery, Fragment, SpecialQuery, NonSpecialQuery, SpecialQuerySegment, NonSpecialQuerySegment, FragmentQuerySegment
);



from_borrowed!(
    MaybeFragmentQuery,

    QueryLike       , FragmentQuery       , Fragment     , Query       , SpecialQuery          , NonSpecialQuery     ,
    MaybeQueryLike  , MaybeFragmentQuery  , MaybeFragment, MaybeQuery  , MaybeSpecialQuery     , MaybeNonSpecialQuery,
    QueryLikeSegment, FragmentQuerySegment,                QuerySegment, NonSpecialQuerySegment, SpecialQuerySegment ,
);
from_options!(
    MaybeFragmentQuery,

    QueryLike       , FragmentQuery       , Fragment, Query       , SpecialQuery       , NonSpecialQuery       ,
    QueryLikeSegment, FragmentQuerySegment,           QuerySegment, SpecialQuerySegment, NonSpecialQuerySegment,
);



from_borrowed!(
    MaybeQuery,

    Query       , SpecialQuery          , NonSpecialQuery     ,
    MaybeQuery  , MaybeSpecialQuery     , MaybeNonSpecialQuery,
    QuerySegment, NonSpecialQuerySegment, SpecialQuerySegment ,
);

from_options!(
    MaybeQuery,

    SpecialQuery          , NonSpecialQuery    ,
    NonSpecialQuerySegment, SpecialQuerySegment,
);



from_borrowed!(
    MaybeSpecialQuery,

    QueryLike       , FragmentQuery       , Fragment     , Query       , SpecialQuery          , NonSpecialQuery     ,
    MaybeQueryLike  , MaybeFragmentQuery  , MaybeFragment, MaybeQuery  , MaybeSpecialQuery     , MaybeNonSpecialQuery,
    QueryLikeSegment, FragmentQuerySegment,                QuerySegment, NonSpecialQuerySegment, SpecialQuerySegment ,
);
from_options!(
    MaybeSpecialQuery,

    QueryLike       , FragmentQuery       , Fragment, Query       , SpecialQuery       , NonSpecialQuery       ,
    QueryLikeSegment, FragmentQuerySegment,           QuerySegment, SpecialQuerySegment, NonSpecialQuerySegment,
);



from_borrowed!(
    MaybeNonSpecialQuery,

    QueryLike       , FragmentQuery       , Fragment     , Query       , SpecialQuery          , NonSpecialQuery     ,
    MaybeQueryLike  , MaybeFragmentQuery  , MaybeFragment, MaybeQuery  , MaybeSpecialQuery     , MaybeNonSpecialQuery,
    QueryLikeSegment, FragmentQuerySegment,                QuerySegment, NonSpecialQuerySegment, SpecialQuerySegment ,
);
from_options!(
    MaybeNonSpecialQuery,


    QueryLike       , FragmentQuery       , Fragment, Query       , SpecialQuery       , NonSpecialQuery       ,
    QueryLikeSegment, FragmentQuerySegment,           QuerySegment, SpecialQuerySegment, NonSpecialQuerySegment,
);

from_borrowed!(QueryLikeName      , QueryLikeName, QueryName, SpecialQueryName, NonSpecialQueryName, FragmentQueryName);
from_borrowed!(FragmentQueryName  , QueryLikeName, QueryName, SpecialQueryName, NonSpecialQueryName, FragmentQueryName);
from_borrowed!(QueryName          ,                QueryName, SpecialQueryName, NonSpecialQueryName                   );
from_borrowed!(SpecialQueryName   , QueryLikeName, QueryName, SpecialQueryName, NonSpecialQueryName, FragmentQueryName);
from_borrowed!(NonSpecialQueryName, QueryLikeName, QueryName, SpecialQueryName, NonSpecialQueryName, FragmentQueryName);

from_borrowed!(QueryLikeValue      , QueryLikeValue, QueryValue, SpecialQueryValue, NonSpecialQueryValue, FragmentQueryValue);
from_borrowed!(FragmentQueryValue  , QueryLikeValue, QueryValue, SpecialQueryValue, NonSpecialQueryValue, FragmentQueryValue);
from_borrowed!(QueryValue          ,                 QueryValue, SpecialQueryValue, NonSpecialQueryValue                    );
from_borrowed!(SpecialQueryValue   , QueryLikeValue, QueryValue, SpecialQueryValue, NonSpecialQueryValue, FragmentQueryValue);
from_borrowed!(NonSpecialQueryValue, QueryLikeValue, QueryValue, SpecialQueryValue, NonSpecialQueryValue, FragmentQueryValue);

from_borrowed!(MaybeQueryLikeValue      , MaybeQueryLikeValue, MaybeQueryValue, MaybeSpecialQueryValue, MaybeNonSpecialQueryValue, MaybeFragmentQueryValue);
from_borrowed!(MaybeFragmentQueryValue  , MaybeQueryLikeValue, MaybeQueryValue, MaybeSpecialQueryValue, MaybeNonSpecialQueryValue, MaybeFragmentQueryValue);
from_borrowed!(MaybeQueryValue          ,                      MaybeQueryValue, MaybeSpecialQueryValue, MaybeNonSpecialQueryValue                         );
from_borrowed!(MaybeSpecialQueryValue   , MaybeQueryLikeValue, MaybeQueryValue, MaybeSpecialQueryValue, MaybeNonSpecialQueryValue, MaybeFragmentQueryValue);
from_borrowed!(MaybeNonSpecialQueryValue, MaybeQueryLikeValue, MaybeQueryValue, MaybeSpecialQueryValue, MaybeNonSpecialQueryValue, MaybeFragmentQueryValue);

from_options!(MaybeQueryLikeValue      ,                             SpecialQueryValue, NonSpecialQueryValue, FragmentQueryValue);
from_options!(MaybeFragmentQueryValue  , QueryLikeValue, QueryValue, SpecialQueryValue, NonSpecialQueryValue, FragmentQueryValue);
from_options!(MaybeQueryValue          ,                             SpecialQueryValue, NonSpecialQueryValue                    );
from_options!(MaybeSpecialQueryValue   , QueryLikeValue, QueryValue, SpecialQueryValue, NonSpecialQueryValue, FragmentQueryValue);
from_options!(MaybeNonSpecialQueryValue, QueryLikeValue, QueryValue, SpecialQueryValue, NonSpecialQueryValue, FragmentQueryValue);
