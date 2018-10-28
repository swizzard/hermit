use std::fs;

extern crate hermit;
use hermit::parse::parse_s;


fn main() {
    let raw = fs::read_to_string("test.toml").expect("can't read file");
    let kvs = parse_s(raw);
    println!("{:?}", kvs);
}


// extern crate pest;
// #[macro_use]
// extern crate pest_derive;

// use pest::Parser;

// #[derive(Parser)]
// #[grammar = "toml.pest"]
// pub struct TomlParser;

// fn main() {
//     let raw = fs::read_to_string("test.toml").expect("can't read file");
//     let parsed = TomlParser::parse(Rule::file, &raw).expect("parsing error")
//         .next().unwrap();
//     for line in parsed.into_inner() {
//         match line.as_rule() {
//             Rule::kv => {
//                 let mut inner_rules = line.into_inner();
//                 let key: &str = inner_rules.next().unwrap().as_str();
//                 let mut val: &str = "";
//                 for x in inner_rules {
//                     let r = x.as_rule();
//                     println!("{:?}", r);
//                     match r {
//                         Rule::simple_string_val => {
//                             for iv in x.into_inner() {
//                                 let ir = iv.as_rule();
//                                 println!("inner {:?}", ir);
//                                 match ir {
//                                     Rule::ssv => val = iv.as_str(),
//                                     _ => (),
//                                 }
//                             }
//                         },
//                         _ => (),
//                     }
//                 }
//                 println!("key: {:?}, value: {:?}", key, val);
//             },
//             _ => println!("something happened"),
//         }
//     }
// }
