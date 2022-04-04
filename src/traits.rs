pub mod codon;
pub mod nucleobase;

pub mod from_letter;
pub mod from_str;
pub mod to_letter;

pub use self::codon::Codon;
pub use self::nucleobase::NucleoBase;

pub use self::from_letter::*;
pub use self::from_str::*;
pub use self::to_letter::*;
