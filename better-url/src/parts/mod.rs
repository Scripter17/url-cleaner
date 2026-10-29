//! Individual parts of URLs.
//!
//! These allow for very fast and ergonomic manipulation of URLs, only checking invariants as needed.
//!
//! ```
//! use better_url::prelude::*;
//!
//! let mut url = BetterUrl::new("https://example.com?a=2&utm_b=3&c=4").unwrap();
//!
//! // A `MaybeQuery`, which easily handles both special and non-special queries as well as their absence.
//! let mut query = url.query();
//!
//! // Remove any segment that doesn't satisfy the predicate.
//! query.filter(|segment| !segment.name().decode().starts_with(b"utm_"));
//!
//! // Basically just a `String::replace_range`. No need to check encoding that's known to be valid at the type level.
//! url.set_query(query.into_owned());
//!
//! assert_eq!(url, "https://example.com/?a=2&c=4");
//! ```
//!
//! Of course, the above example can be written as just
//!
//! ```
//! use better_url::prelude::*;
//!
//! let mut url = BetterUrl::new("https://example.com?a=2&utm_b=3&c=4").unwrap();
//!
//! url.filter_query(|segment| !segment.name().decode().starts_with(b"utm_"));
//!
//! assert_eq!(url, "https://example.com/?a=2&c=4");
//! ```
//!
//! but the non-shorthand version better demonstrates how type level invariants are used.

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
    Userinfo, Username, Password,

    FilePath          , FilePathSegments          , FilePathSegment          ,
    SpecialNotFilePath, SpecialNotFilePathSegments, SpecialNotFilePathSegment,
    NonSpecialPath    , NonSpecialPathSegments    , NonSpecialPathSegment    ,
    OpaquePath,

    SpecialQuery   , SpecialQuerySegment   , SpecialQueryName   , SpecialQueryValue   ,
    NonSpecialQuery, NonSpecialQuerySegment, NonSpecialQueryName, NonSpecialQueryValue,
    FragmentQuery  , FragmentQuerySegment  , FragmentQueryName  , FragmentQueryValue  ,

    Fragment
);

from_option_cow_bytes!(
    MaybeFragment,
    MaybeFragmentQuery     , MaybeSpecialQuery     , MaybeNonSpecialQuery     ,
    MaybeFragmentQueryValue, MaybeSpecialQueryValue, MaybeNonSpecialQueryValue
);

try_from_cow_bytes!(
    Scheme,

    NonSpecialHost,

    OpaqueHost, EmptyHost
);

try_from_cow_bytes_or_str!(
    FileHost, SpecialNotFileHost,

    DomainSegments, DomainHost, DomainSegment,

    Ipv4Host, Ipv6Host
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
    SegmentedPath, OpaquePath,
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
