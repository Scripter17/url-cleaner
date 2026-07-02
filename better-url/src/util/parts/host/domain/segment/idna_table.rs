//! [`idna_valid`].

/// `idna-data.bin`.
const IDNA_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/idna-data.bin"));
/// [`u32::to_be_bytes`] where validity swaps.
const IDNA_SWAP_POINTS: &[[u8; 4]] = IDNA_DATA.as_chunks::<4>().0;

/// Not [`idna_valid`].
pub fn idna_invalid(c: char) -> bool {
    !idna_valid(c)
}

/// If `c`'s value in the [IDNA mapping table](https://www.unicode.org/reports/tr46/#IDNA_Mapping_Table) is either valid or deviation.
///
/// Used for [validity criteria 7.2](https://www.unicode.org/reports/tr46/#Validity_Criteria).
pub fn idna_valid(c: char) -> bool {
    match IDNA_SWAP_POINTS.binary_search(&(c as u32).to_be_bytes()) {
        Ok (i) => i % 2 == 0,
        Err(i) => i % 2 == 1,
    }
}
