extern crate phf;

pub mod currencies;
mod phf_gen;
pub use phf_gen::CURRENCIES;

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
    pub amount: u64,
    pub currency: Currency<'a>,
}
