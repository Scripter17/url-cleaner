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
    FilePath          , FilePathSegment          , FilePathSegments,
    SpecialNotFilePath, SpecialNotFilePathSegment, SpecialNotFilePathSegments,
    NonSpecialPath    , NonSpecialPathSegment    , NonSpecialPathSegments,

    SpecialQuery   , SpecialQuerySegment,
    NonSpecialQuery, NonSpecialQuerySegment,
    FragmentQuery  , FragmentQuerySegment,

    Fragment
);

try_from_cow_impls!(
    Scheme,
    FileHost, SpecialNotFileHost, NonSpecialHost,
    DomainHost, DomainSegment, DomainSegments,
    Ipv4Host, Ipv6Host, OpaqueHost, EmptyHost
);

from_option_cow_impls!(MaybeSpecialQuery, MaybeNonSpecialQuery, MaybeFragmentQuery, MaybeFragment);


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
    Query          , ?MaybeQuery          , QuerySegment          ,
    SpecialQuery   , ?MaybeSpecialQuery   , SpecialQuerySegment   ,
    NonSpecialQuery, ?MaybeNonSpecialQuery, NonSpecialQuerySegment,

    Fragment, ?MaybeFragment,

    FragmentQuery, ?MaybeFragmentQuery, FragmentQuerySegment,

    QueryLike, ?MaybeQueryLike
);

borrowed_impls!(
    Scheme,

    Userinfo,
    Username, Password,

    Host, FileHost, SpecialNotFileHost, NonSpecialHost,
    DomainHost, DomainSegment, DomainSegments,
    Ipv4Host, Ipv6Host, OpaqueHost, EmptyHost,

    Port, MaybePort,

    Path, SegmentedPath, OpaquePath,
    FilePath, SpecialNotFilePath, NonSpecialPath,

    PathSegment              , PathSegments              ,
    SpecialNotFilePathSegment, SpecialNotFilePathSegments,
    FilePathSegment          , FilePathSegments          ,
    NonSpecialPathSegment    , NonSpecialPathSegments    ,

    Query          , MaybeQuery          , QuerySegment          ,
    SpecialQuery   , MaybeSpecialQuery   , SpecialQuerySegment   ,
    NonSpecialQuery, MaybeNonSpecialQuery, NonSpecialQuerySegment,

    Fragment, MaybeFragment,

    FragmentQuery, MaybeFragmentQuery, FragmentQuerySegment,

    QueryLike, MaybeQueryLike, QueryLikeSegment
);
