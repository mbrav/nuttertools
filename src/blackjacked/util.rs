// Util functions and macros

use std::time::SystemTime;

/// Enum macro that declares an enum as well as an array
/// with all its variants e
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
pub fn shuffle_array<T>(array: &mut Vec<T>) {
    for _ in 0..20 {
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        let rng = std::num::Wrapping(seed);

        for i in (1..array.len()).rev() {
            let j = (rng.0 % (i as u64 + 1)) as usize;
            array.swap(i, j);
        }
    }
}
