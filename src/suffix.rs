use publicsuffix::List;
use std::include_str;
use std::sync::OnceLock;

const TLDS_STR: &str=include_str!("tlds.dat");
pub static TLDS: OnceLock<List>=OnceLock::new();

pub fn init_tlds() {
    TLDS.get_or_init(|| TLDS_STR.parse().unwrap());
}
