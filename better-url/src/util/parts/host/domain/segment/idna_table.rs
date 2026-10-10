//! [`idna_valid`] and [`idna_invalid`].

/// The list of [`u32`]s at which subsequent [`char`]s have the opposite IDNA validity, starting with 0 marking subsequent [`char`]s as valid.
const IDNA_SWAPS: &[u32] = include_data::include_u32s!(concat!(env!("OUT_DIR"), "/idna-data.bin"));


// const P00: &[u16] = include_data::include_u16s!(concat!(env!("OUT_DIR"), "/idna-data-00.bin"));
// const P01: &[u16] = include_data::include_u16s!(concat!(env!("OUT_DIR"), "/idna-data-01.bin"));
// const P02: &[u16] = include_data::include_u16s!(concat!(env!("OUT_DIR"), "/idna-data-02.bin"));
// const P03: &[u16] = include_data::include_u16s!(concat!(env!("OUT_DIR"), "/idna-data-03.bin"));
// const P04: &[u16] = include_data::include_u16s!(concat!(env!("OUT_DIR"), "/idna-data-04.bin"));
// const P05: &[u16] = include_data::include_u16s!(concat!(env!("OUT_DIR"), "/idna-data-05.bin"));
// const P06: &[u16] = include_data::include_u16s!(concat!(env!("OUT_DIR"), "/idna-data-06.bin"));
// const P07: &[u16] = include_data::include_u16s!(concat!(env!("OUT_DIR"), "/idna-data-07.bin"));
// const P08: &[u16] = include_data::include_u16s!(concat!(env!("OUT_DIR"), "/idna-data-08.bin"));
// const P09: &[u16] = include_data::include_u16s!(concat!(env!("OUT_DIR"), "/idna-data-09.bin"));
// const P0A: &[u16] = include_data::include_u16s!(concat!(env!("OUT_DIR"), "/idna-data-0A.bin"));
// const P0B: &[u16] = include_data::include_u16s!(concat!(env!("OUT_DIR"), "/idna-data-0B.bin"));
// const P0C: &[u16] = include_data::include_u16s!(concat!(env!("OUT_DIR"), "/idna-data-0C.bin"));
// const P0D: &[u16] = include_data::include_u16s!(concat!(env!("OUT_DIR"), "/idna-data-0D.bin"));
// const P0E: &[u16] = include_data::include_u16s!(concat!(env!("OUT_DIR"), "/idna-data-0E.bin"));
// const P0F: &[u16] = include_data::include_u16s!(concat!(env!("OUT_DIR"), "/idna-data-0F.bin"));
// const P10: &[u16] = include_data::include_u16s!(concat!(env!("OUT_DIR"), "/idna-data-10.bin"));

// const PXX: [&[u16]; 17] = [P00, P01, P02, P03, P04, P05, P06, P07, P08, P09, P0A, P0B, P0C, P0D, P0E, P0F, P10];

// const fn thing() -> [bool; 17] {
//     let mut ret = [false; 17];

//     let mut i = 0;
//     while i < 16 {
//         ret[i as usize + 1] = PXX[i as usize].len() % 2 != (ret[i as usize] as usize);
//         i += 1;
//     }

//     ret
// }

// const THING: [bool; 17] = thing();

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

    // let plane = c as usize >> 16;
    // let index = c as u16;

    // unsafe {
    //     match PXX.get_unchecked(plane).binary_search(&index) {
    //         Ok (i) => i % 2 == *THING.get_unchecked(plane) as usize,
    //         Err(i) => i % 2 != *THING.get_unchecked(plane) as usize,
    //     }
    // }
}
