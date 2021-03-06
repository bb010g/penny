use std::fmt;
use std::str::FromStr;

use CurrencyInfo;

macro_rules! currency {
    ($currency:ident;
     $($code:ident {
         name: $name:expr,
         countries: $countries:expr,
         _countries_str: $countries_str:expr,
         fund: $fund:expr,
         number: $number:expr,
         minor_units: $minor_units:expr,
     },)*) => {
        /// The set of active currencies and funds codes.
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature="serde-serialize", derive(Serialize, Deserialize))]
        pub enum $currency {
            $(
                /// The
                #[doc=$name]
                /// of
                #[doc=$countries_str]
                $code,
            )*
        }
        impl fmt::Display for $currency {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    $(Currency::$code => write!(f, "{}", stringify!($code)),)*
                }
            }
        }
        impl FromStr for $currency {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                CURRENCY.get(s).cloned().ok_or(())
            }
        }
        impl $currency {
            /// Returns information about the currency.
            pub fn info(&self) -> &'static CurrencyInfo {
                match *self {
                    $($currency::$code => $code,)*
                }
            }
        }

        $(pub static $code: &CurrencyInfo = &CurrencyInfo {
            code: stringify!($code),
            name: $name,
            countries: $countries,
            fund: $fund,
            number: $number,
            minor_units: $minor_units,
        };)*
    };
}

include!(concat!(env!("OUT_DIR"), "/currencies.rs"));

#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
mod phf_cur {
    use super::Currency;
    use phf;
    include!(concat!(env!("OUT_DIR"), "/phf_cur.rs"));
}
pub use self::phf_cur::CURRENCY;
