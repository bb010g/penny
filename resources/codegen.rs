#!/usr/bin/env run-cargo-script
//! ```cargo
//! [dependencies]
//! quick-xml = "0.4.2"
//! phf_codegen = "0.7.20"
//! ```
extern crate quick_xml;
extern crate phf_codegen;

use quick_xml::{XmlReader, Event};
use std::collections::BTreeMap;
use std::str;
use std::io::Write;

#[derive(Debug)]
struct Currency {
    name: String,
    countries: Vec<String>,
    fund: bool,
    number: u16,
    minor_units: Option<u8>,
}
enum Tag {
    CountryName,
    CurrencyName,
    Code,
    Number,
    Units,
}
fn main() {
    let reader = XmlReader::from_file("list_one.xml").unwrap().trim_text(true);
    let mut tag: Tag = Tag::CountryName;

    let mut currencies: BTreeMap<String, Currency> = BTreeMap::new();

    let mut country = String::from("");
    let mut fund = false;
    let mut name = String::from("");
    let mut code = String::from("");
    let mut num: u16 = 0;
    let mut units: Option<u8> = None;
    for r in reader {
        match r {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"CtryNm" => tag = Tag::CountryName,
                    b"CcyNm" => {
                        tag = Tag::CurrencyName;
                        fund = e.attributes()
                            .find(|r| r.as_ref().map(|&(a, _)| a == b"IsFund").unwrap_or(false))
                            .is_some()
                    }
                    b"Ccy" => tag = Tag::Code,
                    b"CcyNbr" => tag = Tag::Number,
                    b"CcyMnrUnts" => tag = Tag::Units,
                    _ => (),
                }
            }
            Ok(Event::Text(e)) => {
                match tag {
                    Tag::CountryName => country = String::from_utf8(e.content().to_vec()).unwrap(),
                    Tag::CurrencyName => name = String::from_utf8(e.content().to_vec()).unwrap(),
                    Tag::Code => code = String::from_utf8(e.content().to_vec()).unwrap(),
                    Tag::Number => num = str::from_utf8(e.content()).unwrap().parse().unwrap(),
                    Tag::Units => units = str::from_utf8(e.content()).unwrap().parse().ok(),
                }
            }
            Ok(Event::End(e)) => {
                match e.name() {
                    b"CcyNtry" => {
                        currencies.entry(code.clone())
                            .or_insert(Currency {
                                name: name.clone(),
                                countries: Vec::new(),
                                fund: fund,
                                number: num,
                                minor_units: units,
                            })
                            .countries
                            .push(country.clone());
                    }
                    _ => (),
                }
            }
            Err((e, pos)) => panic!("{:?} at position {}", e, pos),
            _ => (),
        }
    }

    let mut src = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("../src/currencies.rs")
        .unwrap();
    writeln!(&mut src,
             "// THIS CODE IS GENERATED, DO NOT MANUALLY MODIFY")
        .unwrap();
    writeln!(&mut src, "use super::Currency;").unwrap();
    writeln!(&mut src, "").unwrap();

    let mut phf_src = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("../src/phf_gen.rs")
        .unwrap();
    writeln!(&mut phf_src,
             "// THIS CODE IS GENERATED, DO NOT MANUALLY MODIFY")
        .unwrap();
    writeln!(&mut phf_src, "use super::currencies::*;").unwrap();
    writeln!(&mut phf_src, "use phf;").unwrap();
    writeln!(&mut phf_src, "").unwrap();
    write!(&mut phf_src,
           "pub static CURRENCIES: phf::Map<&'static str, super::Currency<'static>> = ")
        .unwrap();
    let mut map = phf_codegen::Map::new();
    map.phf_path("phf");

    for (code, currency) in currencies {
        writeln!(&mut src,
                 concat!("pub const {code}: Currency<'static> = Currency {{\n",
                         "    code: {code:?},\n",
                         "    name: {name:?},\n",
                         "    countries: &[{countries}],\n",
                         "    fund: {fund},\n",
                         "    number: {number},\n",
                         "    minor_units: {minor_units:?},\n",
                         "}};\n"),
                 code = code,
                 name = currency.name,
                 countries = &(currency.countries)
                     .iter()
                     .map(|s| format!("{:?}", s))
                     .collect::<Vec<_>>()
                     .join(", "),
                 fund = currency.fund,
                 number = currency.number,
                 minor_units = currency.minor_units)
            .unwrap();
        map.entry(code.clone(), &code);
    }

    map.build(&mut phf_src).unwrap();
    writeln!(&mut phf_src, ";").unwrap();
}
