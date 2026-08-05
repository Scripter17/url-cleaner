//! [`idna_valid`].

/// The list of [`u32`]s at which subsequent [`char`]s have the opposite IDNA validity, starting with 0 marking subsequent [`char`]s as valid.
#[cfg(target_endian = "big"   )] const IDNA_SWAPS: &[u32] = include_data::include_u32s!(concat!(env!("OUT_DIR"), "/idna-data-be.bin"));
/// The list of [`u32`]s at which subsequent [`char`]s have the opposite IDNA validity, starting with 0 marking subsequent [`char`]s as valid.
#[cfg(target_endian = "little")] const IDNA_SWAPS: &[u32] = include_data::include_u32s!(concat!(env!("OUT_DIR"), "/idna-data-le.bin"));

/// Not [`idna_valid`].
pub fn idna_invalid(c: char) -> bool {
    !idna_valid(c)
}

/// If `c`'s value in the [IDNA mapping table](https://www.unicode.org/reports/tr46/#IDNA_Mapping_Table) is either valid or deviation.
///
/// Used for [validity criteria 7.2](https://www.unicode.org/reports/tr46/#Validity_Criteria).
pub fn idna_valid(c: char) -> bool {
    match IDNA_SWAPS.binary_search(&(c as u32)) {
        Ok (i) => i % 2 == 0,
        Err(i) => i % 2 == 1,
    }
}
