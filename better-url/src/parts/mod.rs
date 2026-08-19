//! Parts.

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

from_cow_impls!(
    Userinfo, Username, Password,

    OpaquePath,
    FilePath          , FilePathSegment          , FilePathSegments          ,
    SpecialNotFilePath, SpecialNotFilePathSegment, SpecialNotFilePathSegments,
    NonSpecialPath    , NonSpecialPathSegment    , NonSpecialPathSegments    ,

    SpecialQuery   , SpecialQuerySegment   , SpecialQueryName   , SpecialQueryValue   ,
    NonSpecialQuery, NonSpecialQuerySegment, NonSpecialQueryName, NonSpecialQueryValue,
    FragmentQuery  , FragmentQuerySegment  , FragmentQueryName  , FragmentQueryValue  ,

    Fragment
);

try_from_cow_impls!(
    Scheme,
    FileHost, SpecialNotFileHost, NonSpecialHost,
    DomainHost, DomainSegment, DomainSegments,
    Ipv4Host, Ipv6Host, OpaqueHost, EmptyHost
);

from_option_cow_impls!(
    MaybeSpecialQuery, MaybeNonSpecialQuery, MaybeFragmentQuery, MaybeFragment,
    MaybeSpecialQueryValue, MaybeNonSpecialQueryValue, MaybeFragmentQueryValue
);


as_str_impls!(Scheme);

as_str_impls!(Userinfo);
as_str_impls!(Username, Password);

as_str_impls!(
    Host, FileHost, SpecialNotFileHost, NonSpecialHost,
    DomainHost, DomainSegment, DomainSegments,
    Ipv4Host, Ipv6Host, OpaqueHost, EmptyHost
);

as_str_impls!(
    Path,
    OpaquePath, SegmentedPath,
    SpecialNotFilePath, FilePath, NonSpecialPath
);

as_str_impls!(
    PathSegment              , PathSegments              ,
    SpecialNotFilePathSegment, SpecialNotFilePathSegments,
    FilePathSegment          , FilePathSegments          ,
    NonSpecialPathSegment    , NonSpecialPathSegments
);

as_str_impls!(
    Query    , ?MaybeQuery    , QuerySegment    , QueryName    , QueryValue    , ?MaybeQueryValue    ,
    QueryLike, ?MaybeQueryLike, QueryLikeSegment, QueryLikeName, QueryLikeValue, ?MaybeQueryLikeValue,

    SpecialQuery   , ?MaybeSpecialQuery   , SpecialQuerySegment   , SpecialQueryName   , SpecialQueryValue   , ?MaybeSpecialQueryValue   ,
    NonSpecialQuery, ?MaybeNonSpecialQuery, NonSpecialQuerySegment, NonSpecialQueryName, NonSpecialQueryValue, ?MaybeNonSpecialQueryValue,
    FragmentQuery  , ?MaybeFragmentQuery  , FragmentQuerySegment  , FragmentQueryName  , FragmentQueryValue  , ?MaybeFragmentQueryValue  ,

    Fragment, ?MaybeFragment
);

borrowed_impls!(
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
