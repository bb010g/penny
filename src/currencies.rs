use phf;
use std::fmt;

use Currency;

macro_rules! currency {
    ($currency_code:ident;
     $($code:ident($code_ref:ident) {
         code: $code_str:expr,
         name: $name:expr,
         countries: $countries:expr,
         fund: $fund:expr,
         number: $number:expr,
         minor_units: $minor_units:expr,
     },)*) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        #[cfg_attr(feature="serde-serialize", derive(Serialize, Deserialize))]
        pub enum $currency_code {
            $($code,)*
        }
        impl fmt::Display for $currency_code {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    $(CurrencyCode::$code => write!(f, "{}", $code_str),)*
                }
            }
        }
        $(pub const $code: Currency<'static> = Currency {
            code: $code_str,
            name: $name,
            countries: $countries,
            fund: $fund,
            number: $number,
            minor_units: $minor_units,
        };
        pub static $code_ref: &'static Currency<'static> = &$code;)*
    }
}

include!(concat!(env!("OUT_DIR"), "/currencies.rs"));

include!(concat!(env!("OUT_DIR"), "/phf_cur_code.rs"));
include!(concat!(env!("OUT_DIR"), "/phf_currencies.rs"));
