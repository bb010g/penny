/*!
This crate provides a library for dealing with currency as specified by ISO-4217. It currently only
provides a native representation of the data from [SNV's][snv] [current currency & funds code
list][list-one].

[snv]: https://www.currency-iso.org/en/home.html
[list-one]: https://www.currency-iso.org/en/home/tables/table-a1.html
*/

#![cfg_attr(feature="lint", feature(plugin))]
#![cfg_attr(feature="lint", plugin(clippy))]
#![warn(missing_docs)]

extern crate mitochondria;
extern crate phf;
#[cfg(featuer="serde")]
extern crate serde;
#[cfg(feature="serde")]
#[macro_use]
extern crate serde_derive;

use mitochondria::OnceCell;

mod currencies;

pub use currencies::Currency;

/// A set of information returned from the [`Currency::info`][info] method.
///
/// [info]: enum.Currency.html#method.info
#[derive(Debug, Clone)]
#[cfg_attr(feature="serde-serialize", derive(Serialize))]
pub struct CurrencyInfo {
    code: &'static str,
    name: &'static str,
    countries: &'static [&'static str],
    fund: bool,
    number: u16,
    minor_units: Option<u8>,
}

impl CurrencyInfo {
    /// Returns the 3 letter code of the currency.
    pub fn code(&self) -> &'static str {
        self.code
    }
    /// Returns the name of the currency.
    pub fn name(&self) -> &'static str {
        self.name
    }
    /// Returns the countries that use the currency.
    pub fn countries(&self) -> &'static [&'static str] {
        self.countries
    }
    /// Returns `true` if the currency is a fund code.
    pub fn is_fund(&self) -> bool {
        self.fund
    }
    /// Returns the 3 digit code of the currency.
    pub fn number(&self) -> u16 {
        self.number
    }
    /// Returns the decimal places of the currency.
    pub fn minor_units(&self) -> Option<u8> {
        self.minor_units
    }
}

/// A bundle of an amount of money and the currency it's in.
#[derive(Debug, Clone)]
#[cfg_attr(feature="serde-serialize", derive(Serialize, Deserialize))]
pub struct Money {
    amount: i64,
    currency: Currency,
    #[serde(skip)]
    currency_info: OnceCell<&'static CurrencyInfo>,
}

impl Money {
    /// Creates a new `Money` of the given amount and currency.
    pub fn new(amount: i64, currency: Currency) -> Money {
        Money {
            amount,
            currency,
            currency_info: OnceCell::default(),
        }
    }

    /// Returns the contained amount of money.
    pub fn amount(&self) -> i64 {
        self.amount
    }
    /// Returns the type of currency.
    pub fn currency(&self) -> Currency {
        self.currency
    }
    /// Returns information about this money's currency.
    ///
    /// Caches the reference to make future accesses quicker.
    pub fn currency_info(&self) -> &'static CurrencyInfo {
        self.currency_info.init_once(|| self.currency.info())
    }
}
