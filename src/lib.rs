#![cfg_attr(feature="lint", feature(plugin))]
#![cfg_attr(feature="lint", plugin(clippy))]

extern crate phf;

pub mod currencies;

include!(concat!(env!("OUT_DIR"), "/phf_gen.rs"));

#[derive(Debug)]
pub struct Currency<'a> {
    code: &'a str,
    name: &'a str,
    countries: &'a [&'a str],
    fund: bool,
    number: u16,
    minor_units: Option<u8>,
}

impl<'a> Currency<'a> {
    pub fn code(&self) -> &'a str {
        self.code
    }
    pub fn name(&self) -> &'a str {
        self.name
    }
    pub fn countries(&self) -> &'a [&'a str] {
        self.countries
    }
    pub fn fund(&self) -> bool {
        self.fund
    }
    pub fn number(&self) -> u16 {
        self.number
    }
    pub fn minor_units(&self) -> Option<u8> {
        self.minor_units
    }
}

pub struct Money<'a> {
    amount: i64,
    currency: &'a Currency<'a>,
}

impl<'a> Money<'a> {
    pub fn new(amount: i64, currency: &'a Currency<'a>) -> Money<'a> {
        Money {
            amount: amount,
            currency: currency,
        }
    }

    pub fn amount(&self) -> i64 {
        self.amount
    }
    pub fn currency(&self) -> &Currency {
        self.currency
    }
}
