//! Converters from the inputs to the canonical setters to their corresponding part types.

mod common   ; pub use common   ::*;
mod whole    ; pub use whole    ::*;
mod scheme   ; pub use scheme   ::*;
mod username ; pub use username ::*;
mod password ; pub use password ::*;
mod host     ; pub use host     ::*;
mod host_port; pub use host_port::*;
mod path     ; pub use path     ::*;
mod port     ; pub use port     ::*;
mod query    ; pub use query    ::*;
mod fragment ; pub use fragment ::*;
