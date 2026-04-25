// Util functions and macros

use std::time::SystemTime;

// Enum macro that declares an enum as well as an array
// with all its variants e
// #[macro_export]
// macro_rules! make_enum {
//     (
//         $name:ident $array:ident {
//             $( $variant:ident, )*
//         }
//     ) => {
//         pub enum $name {
//             $( $variant, )*
//         }
//         static $array: &[$name] = &[
//             $( $name::$variant, )*
//         ];
//     }
// }

/// Shuffle an array
///
/// # Panics
///
/// Panics if the system clock is set before the UNIX epoch.
pub fn shuffle_array<T>(array: &mut [T]) {
    for _ in 0..20 {
        let seed = u64::try_from(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("system time is before UNIX epoch")
                .as_nanos(),
        )
        .unwrap_or(u64::MAX);

        let rng = std::num::Wrapping(seed);

        for i in (1..array.len()).rev() {
            #[allow(clippy::cast_possible_truncation)]
            let j = (rng.0 % (i as u64 + 1)) as usize;
            array.swap(i, j);
        }
    }
}
