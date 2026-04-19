//! Parts.

use crate::prelude::*;

mod scheme;
mod userinfo;
mod username;
mod password;
mod host;
mod path;
mod query;
mod fragment;

pub use scheme::*;
pub use userinfo::*;
pub use username::*;
pub use password::*;
pub use host::*;
pub use path::*;
pub use query::*;
pub use fragment::*;

as_str_impls!(Userinfo);
as_str_impls!(Username, Password);
as_str_impls!(Password, Username);

from_cow_impls!(Userinfo, Username, Password);



as_str_impls!(Path                       ,       OpaquePath, SegmentedPath, SpecialNotFileSegmentedPath, FileSegmentedPath, NonSpecialSegmentedPath, NonSpecialPath);
as_str_impls!(OpaquePath                 , Path,             SegmentedPath, SpecialNotFileSegmentedPath, FileSegmentedPath, NonSpecialSegmentedPath, NonSpecialPath);
as_str_impls!(SegmentedPath              , Path, OpaquePath,                SpecialNotFileSegmentedPath, FileSegmentedPath, NonSpecialSegmentedPath, NonSpecialPath);
as_str_impls!(SpecialNotFileSegmentedPath, Path, OpaquePath, SegmentedPath,                              FileSegmentedPath, NonSpecialSegmentedPath, NonSpecialPath);
as_str_impls!(FileSegmentedPath          , Path, OpaquePath, SegmentedPath, SpecialNotFileSegmentedPath,                    NonSpecialSegmentedPath, NonSpecialPath);
as_str_impls!(NonSpecialSegmentedPath    , Path, OpaquePath, SegmentedPath, SpecialNotFileSegmentedPath, FileSegmentedPath,                          NonSpecialPath);
as_str_impls!(NonSpecialPath             , Path, OpaquePath, SegmentedPath, SpecialNotFileSegmentedPath, FileSegmentedPath, NonSpecialSegmentedPath                );

from_cow_impls!(Path, OpaquePath, SegmentedPath, SpecialNotFileSegmentedPath, FileSegmentedPath, NonSpecialSegmentedPath, NonSpecialPath);



as_str_impls!(PathSegments              ,               SpecialNotFilePathSegments, FilePathSegments, NonSpecialPathSegments);
as_str_impls!(SpecialNotFilePathSegments, PathSegments,                             FilePathSegments, NonSpecialPathSegments);
as_str_impls!(FilePathSegments          , PathSegments, SpecialNotFilePathSegments,                   NonSpecialPathSegments);
as_str_impls!(NonSpecialPathSegments    , PathSegments, SpecialNotFilePathSegments, FilePathSegments                        );



as_str_impls!(PathSegment               ,              SpecialNotFilePathSegment, FilePathSegment, NonSpecialPathSegment);
as_str_impls!(SpecialNotFilePathSegment , PathSegment,                            FilePathSegment, NonSpecialPathSegment);
as_str_impls!(FilePathSegment           , PathSegment, SpecialNotFilePathSegment,                  NonSpecialPathSegment);
as_str_impls!(NonSpecialPathSegment     , PathSegment, SpecialNotFilePathSegment, FilePathSegment                       );

from_cow_impls!(PathSegment, SpecialNotFilePathSegment, FilePathSegment, NonSpecialPathSegment);



as_str_impls!(Host      ,       DomainHost, IpHost, Ipv4Host, Ipv6Host, OpaqueHost, EmptyHost);
as_str_impls!(DomainHost, Host,             IpHost, Ipv4Host, Ipv6Host, OpaqueHost, EmptyHost);
as_str_impls!(IpHost    , Host, DomainHost,         Ipv4Host, Ipv6Host, OpaqueHost, EmptyHost);
as_str_impls!(Ipv4Host  , Host, DomainHost, IpHost,           Ipv6Host, OpaqueHost, EmptyHost);
as_str_impls!(Ipv6Host  , Host, DomainHost, IpHost, Ipv4Host,           OpaqueHost, EmptyHost);
as_str_impls!(OpaqueHost, Host, DomainHost, IpHost, Ipv4Host, Ipv6Host,             EmptyHost);
as_str_impls!(EmptyHost , Host, DomainHost, IpHost, Ipv4Host, Ipv6Host, OpaqueHost           );

try_from_cow_impls!(Host, DomainHost, IpHost, Ipv4Host, Ipv6Host, OpaqueHost, EmptyHost);



as_str_impls!( Query                 ,        SpecialQuery, NonSpecialQuery, ?MaybeQuery, ?MaybeSpecialQuery, ?MaybeNonSpecialQuery, QuerySegment, SpecialQuerySegment, NonSpecialQuerySegment, Fragment, ?MaybeFragment);
as_str_impls!( SpecialQuery          , Query,               NonSpecialQuery, ?MaybeQuery, ?MaybeSpecialQuery, ?MaybeNonSpecialQuery, QuerySegment, SpecialQuerySegment, NonSpecialQuerySegment, Fragment, ?MaybeFragment);
as_str_impls!( NonSpecialQuery       , Query, SpecialQuery,                  ?MaybeQuery, ?MaybeSpecialQuery, ?MaybeNonSpecialQuery, QuerySegment, SpecialQuerySegment, NonSpecialQuerySegment, Fragment, ?MaybeFragment);
as_str_impls!(?MaybeQuery            , Query, SpecialQuery, NonSpecialQuery,              ?MaybeSpecialQuery, ?MaybeNonSpecialQuery, QuerySegment, SpecialQuerySegment, NonSpecialQuerySegment, Fragment, ?MaybeFragment);
as_str_impls!(?MaybeSpecialQuery     , Query, SpecialQuery, NonSpecialQuery, ?MaybeQuery,                     ?MaybeNonSpecialQuery, QuerySegment, SpecialQuerySegment, NonSpecialQuerySegment, Fragment, ?MaybeFragment);
as_str_impls!(?MaybeNonSpecialQuery  , Query, SpecialQuery, NonSpecialQuery, ?MaybeQuery, ?MaybeSpecialQuery,                        QuerySegment, SpecialQuerySegment, NonSpecialQuerySegment, Fragment, ?MaybeFragment);
as_str_impls!( QuerySegment          , Query, SpecialQuery, NonSpecialQuery, ?MaybeQuery, ?MaybeSpecialQuery, ?MaybeNonSpecialQuery,               SpecialQuerySegment, NonSpecialQuerySegment, Fragment, ?MaybeFragment);
as_str_impls!( SpecialQuerySegment   , Query, SpecialQuery, NonSpecialQuery, ?MaybeQuery, ?MaybeSpecialQuery, ?MaybeNonSpecialQuery, QuerySegment,                      NonSpecialQuerySegment, Fragment, ?MaybeFragment);
as_str_impls!( NonSpecialQuerySegment, Query, SpecialQuery, NonSpecialQuery, ?MaybeQuery, ?MaybeSpecialQuery, ?MaybeNonSpecialQuery, QuerySegment, SpecialQuerySegment,                         Fragment, ?MaybeFragment);
as_str_impls!( Fragment              , Query, SpecialQuery, NonSpecialQuery, ?MaybeQuery, ?MaybeSpecialQuery, ?MaybeNonSpecialQuery, QuerySegment, SpecialQuerySegment, NonSpecialQuerySegment,           ?MaybeFragment);
as_str_impls!(?MaybeFragment         , Query, SpecialQuery, NonSpecialQuery, ?MaybeQuery, ?MaybeSpecialQuery, ?MaybeNonSpecialQuery, QuerySegment, SpecialQuerySegment, NonSpecialQuerySegment, Fragment                );

from_cow_impls!(Query, SpecialQuery, NonSpecialQuery, QuerySegment, SpecialQuerySegment, NonSpecialQuerySegment);
from_option_cow_impls!(MaybeQuery, MaybeSpecialQuery, MaybeNonSpecialQuery);

from_cow_impls!(Fragment);
from_option_cow_impls!(MaybeFragment);
