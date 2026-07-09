//! Parts.

use crate::prelude::*;

mod scheme;
mod userinfo;
mod host;
mod port;
mod path;
mod query;
mod fragment;
mod query_like;

pub use scheme::*;
pub use userinfo::*;
pub use host::*;
pub use port::*;
pub use path::*;
pub use query::*;
pub use fragment::*;
pub use query_like::*;

from_cow_impls!(
    Userinfo, Username, Password,

    OpaquePath,
    FileSegmentedPath          , FilePathSegment,
    SpecialNotFileSegmentedPath, SpecialNotFilePathSegment,
    NonSpecialSegmentedPath    , NonSpecialPathSegment,
    NonSpecialPath,

    SpecialQuery   , SpecialQuerySegment,
    NonSpecialQuery, NonSpecialQuerySegment,
    FragmentQuery  , FragmentQuerySegment,

    Fragment
);

try_from_cow_impls!(
    Scheme,
    FileHost, SpecialNotFileHost, NonSpecialHost,
    DomainHost, DomainSegment, DomainSegments,
    IpHost, Ipv4Host, Ipv6Host, OpaqueHost, EmptyHost,
    NonSpecialEmptyPath
);

from_option_cow_impls!(MaybeSpecialQuery, MaybeNonSpecialQuery, MaybeFragmentQuery, MaybeFragment);


as_str_impls!(Scheme);

as_str_impls!(Userinfo);
as_str_impls!(Username, Password);

as_str_impls!(
    Host, FileHost, SpecialNotFileHost, NonSpecialHost,
    DomainHost, DomainSegment, DomainSegments,
    IpHost, Ipv4Host, Ipv6Host, OpaqueHost, EmptyHost
);

as_str_impls!(
    Path,
    OpaquePath, SegmentedPath,
    SpecialNotFileSegmentedPath, FileSegmentedPath, NonSpecialPath, NonSpecialSegmentedPath, NonSpecialEmptyPath
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

    FragmentQuery, ?MaybeFragmentQuery, FragmentQuerySegment
);

borrowed_impls!(
    Scheme,

    Userinfo,
    Username, Password,

    Host, FileHost, SpecialNotFileHost, NonSpecialHost,
    DomainHost, DomainSegment, DomainSegments,
    IpHost, Ipv4Host, Ipv6Host, OpaqueHost, EmptyHost,

    Port, MaybePort,

    Path, SegmentedPath, OpaquePath,
    FileSegmentedPath, SpecialNotFileSegmentedPath,
    NonSpecialPath, NonSpecialSegmentedPath, NonSpecialEmptyPath,

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
