extern crate phf_codegen;
extern crate quick_xml;

use std::collections::btree_map::{BTreeMap, Entry};
use std::env;
use std::io::{BufRead, Write};
use std::str;

use quick_xml::events::Event;
use quick_xml::reader::Reader;

#[derive(Debug)]
struct Currency {
    code: String,
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
    let mut reader = Reader::from_file("src/list_one.xml").expect("list_one.xml missing");
    println!("cargo:rerun-if-changed=src/list_one.xml");
    reader.trim_text(true);

    let mut xml_currencies: BTreeMap<String, Currency> = BTreeMap::new();

    for mut currency in Currencies::new(&mut reader) {
        let entry = xml_currencies.entry(currency.code.clone());
        match entry {
            Entry::Occupied(mut e) => {
                e.get_mut().countries.push(currency.countries.remove(0));
            }
            Entry::Vacant(e) => {
                e.insert(currency);
            }
        };
    }

    let out_dir = env::var("OUT_DIR").unwrap();

    let currencies_path = std::path::Path::new(&out_dir).join("currencies.rs");
    let mut currencies =
        std::fs::File::create(&currencies_path).expect("Can't create currencies.rs");

    let phf_cur_path = std::path::Path::new(&out_dir).join("phf_cur.rs");
    let mut phf_cur = std::fs::File::create(&phf_cur_path).expect("Can't create phf_cur.rs");

    writeln!(&mut currencies, "currency! {{ Currency;\n").unwrap();

    write!(
        &mut phf_cur,
        "pub static CURRENCY: phf::Map<&'static str, Currency> = "
    ).unwrap();
    let mut cur_map = phf_codegen::Map::new();
    cur_map.phf_path("phf");

    for (code, currency) in xml_currencies {
        let cur_str = String::from("Currency::") + &code;

        cur_map.entry(code.clone(), &cur_str);
        writeln!(
            &mut currencies,
            concat!(
                "{code} {{\n",
                "    name: {name:?},\n",
                "    countries: &[{countries}],\n",
                "    _countries_str: {countries_str:?},\n",
                "    fund: {fund},\n",
                "    number: {number},\n",
                "    minor_units: {minor_units:?},\n",
                "}},\n"
            ),
            code = code,
            name = currency.name,
            countries = &(currency.countries)
                .iter()
                .map(|s| format!("{:?}", s))
                .collect::<Vec<_>>()
                .join(", "),
            countries_str = countries_str(&*currency.countries) + ".",
            fund = currency.fund,
            number = currency.number,
            minor_units = currency.minor_units
        ).unwrap();
    }

    writeln!(&mut currencies, "}}").unwrap();
    cur_map.build(&mut phf_cur).unwrap();
    writeln!(&mut phf_cur, ";").unwrap();
}

struct Currencies<'a, B>
where
    B: BufRead,
    B: 'a,
{
    reader: &'a mut Reader<B>,
    buf: Vec<u8>,
}

impl<'a, B: BufRead> Currencies<'a, B> {
    fn new(reader: &'a mut Reader<B>) -> Currencies<'a, B> {
        Currencies {
            reader: reader,
            buf: Vec::new(),
        }
    }
}

impl<'a, B: BufRead> Iterator for Currencies<'a, B> {
    type Item = Currency;

    fn next(&mut self) -> Option<Currency> {
        let mut tag: Tag = Tag::CountryName;
        let mut country = String::new();
        let mut fund = false;
        let mut name = String::new();
        let mut code = String::new();
        let mut num: u16 = 0;
        let mut units: Option<u8> = None;

        loop {
            match self.reader.read_event(&mut self.buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"CtryNm" => tag = Tag::CountryName,
                        b"CcyNm" => {
                            tag = Tag::CurrencyName;
                            fund = e.attributes()
                                .find(|r| {
                                    r.as_ref()
                                        .map(|a| a.key == b"IsFund" && a.value == b"true")
                                        .unwrap_or(false)
                                })
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
                        Tag::CountryName => country = String::from_utf8(e.to_vec()).unwrap(),
                        Tag::CurrencyName => name = String::from_utf8(e.to_vec()).unwrap(),
                        Tag::Code => code = String::from_utf8(e.to_vec()).unwrap(),
                        Tag::Number => num = str::from_utf8(&e).unwrap().parse().unwrap(),
                        Tag::Units => units = str::from_utf8(&e).unwrap().parse().ok(),
                    }
                }
                Ok(Event::End(e)) => {
                    match e.name() {
                        b"CcyNtry" => {
                            if code != "" {
                                return Some(Currency {
                                    code: code.clone(),
                                    name: name.clone(),
                                    countries: vec![country.to_owned()],
                                    fund: fund,
                                    number: num,
                                    minor_units: units,
                                });
                            }
                        }
                        _ => (),
                    }
                }
                Ok(Event::Eof) => return None,
                Err(e) => {
                    panic!(
                        "Error at position {}: {:?}",
                        self.reader.buffer_position(),
                        e
                    )
                }
                _ => (),
            }
            self.buf.clear();
        }
    }
}

fn countries_str(countries: &[String]) -> String {
    let len = countries.len();
    if len == 1 {
        countries[0].clone()
    } else if len == 2 {
        let mut out = String::new();
        out.push_str(&countries[0]);
        out.push_str(" and ");
        out.push_str(&countries[1]);
        out
    } else if let Some((last, init)) = countries.split_last() {
        let mut out = init.join(", ");
        out.push_str(", and ");
        out.push_str(last);
        out
    } else {
        String::from("nowhere")
    }
}
