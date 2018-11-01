extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod parse {

    const DEBUG: bool = true;

    use pest::Parser;
    use pest::iterators::*;


    #[derive(Parser)]
    #[grammar = "toml.pest"]
    struct TomlParser;

    pub trait TomlValue {}

    impl TomlValue for bool {}
    impl TomlValue for String {}
    impl TomlValue for str {}
    impl TomlValue for i8 {}
    impl TomlValue for i16 {}
    impl TomlValue for i32 {}
    impl TomlValue for i64 {}
    impl TomlValue for isize {}
    impl TomlValue for u8 {}
    impl TomlValue for u16 {}
    impl TomlValue for u32 {}
    impl TomlValue for u64 {}
    impl TomlValue for usize {}
    impl TomlValue for f32 {}
    impl TomlValue for f64 {}

    #[derive(Debug)]
    pub struct RawKV {
        namespace: Vec<String>,
        key: String,
        value: String,
    }

    fn parse_simple_key(p: Pair<Rule>) -> String {
        let mut k = String::from("");
        for inner in p.into_inner() {
            match inner.as_rule() {
                Rule::quoted_key => {
                    for inn in inner.into_inner() {
                        match inn.as_rule() {
                            Rule::char => k.push_str(inn.as_str()),
                            Rule::dot => k.push_str(inn.as_str()),
                            Rule::quote => (),
                            _ => {
                                if DEBUG {
                                    println!("{:?}", inn)
                                }
                            },
                        }
                    }
                },
                Rule::bare_key => k = inner.as_str().to_owned(),
                _ => {
                    if DEBUG {
                        println!("{:?}", inner)
                    }
                }
            }
        }
        k
    }

    fn parse_dotted_key(p: Pair<Rule>) -> (Vec<String>, String) {
        let mut ns: Vec<String> = Vec::new();
        let mut curr = String::new();
        for inner in p.into_inner() {
            match inner.as_rule() {
                Rule::simple_key => {
                    curr = parse_simple_key(inner);
                },
                Rule::dot => {
                    ns.push(curr);
                    curr = String::new();
                },
                _ => {
                    if DEBUG {
                        println!("{:?}", inner)
                    }
                }
            }
        }
        (ns, curr)
    }

    fn parse_uni_escape_4(p: Pair<Rule>) -> String {
        p.as_str().to_owned()
    }

    fn parse_uni_escape_8(p: Pair<Rule>) -> String {
        p.as_str().to_owned()
    }

    fn parse_escape(p: Pair<Rule>) -> Option<String> {
        match p.into_inner().next().unwrap().as_rule() {
            Rule::escape_backspace => Some(r"\b".to_owned()),
            Rule::escape_tab => Some("\t".to_owned()),
            Rule::escape_linefeed => Some("\n".to_owned()),
            Rule::escape_formfeed => Some(r"\f".to_owned()),
            Rule::escape_cr => Some("\r".to_owned()),
            Rule::escape_quote => Some("\"".to_owned()),
            Rule::escape_backslash => Some("\\".to_owned()),
            _ => None,
        }
    }

    fn parse_string_val(p: Pair<Rule>) -> String {
        let mut s = String::new();
        for inner in p.into_inner() {
            match inner.as_rule() {
                Rule::str_val_chr => {
                    for iv in inner.into_inner() {
                        match iv.as_rule() {
                            Rule::char => s.push_str(iv.as_str()),
                            Rule::uni_escape_4 => {
                                s += &parse_uni_escape_4(iv);
                            },
                            Rule::uni_escape_8 => {
                                s += &parse_uni_escape_8(iv);
                            },
                            Rule::escape => {
                                if let Some(v) = parse_escape(iv) {
                                    s.push_str(&v);
                                }
                            },
                            _ => (),
                        }
                    }
                },
                Rule::quote => (),
                Rule::char => s.push_str(inner.as_str()),
                _ => {
                    if DEBUG {
                        println!("parse_string_val other: {:?}", inner)
                    }
                },
            }
        }
        s
    }

    fn parse_triple_quote(p: Pair<Rule>) -> String {
        let mut s = String::new();
        for inner in p.into_inner() {
            match inner.as_rule() {
                Rule::escaped_whitespace => s.push_str(" "),
                Rule::newline => s.push_str("\n"),
                Rule::str_val_chr => s += &parse_string_val(inner),
                Rule::triple_quote => (),
                _ => {
                    if DEBUG {
                        println!("other: {:?}", inner);
                    }
                },
            }
        }
        s
    }

    fn parse_datetime(p: Pair<Rule>) -> String {
        p.as_str().to_owned()
    }

    fn parse_simple_val(p: Pair<Rule>) -> String {
        let v = p.into_inner().next().unwrap();
        match v.as_rule() {
            Rule::int | Rule::float => v.as_str().to_owned(),
            Rule::triple_quote_val => parse_triple_quote(v),
            Rule::simple_string_val => parse_string_val(v),
            Rule::datetime => parse_datetime(v),
            _ => {
                if DEBUG {
                    println!("{:?}", v);
                }
                String::new()
            },
        }
    }

    fn parse_array(p: Pair<Rule>) -> String {
        let mut s = String::from("[");
        for inner in p.into_inner() {
            match inner.as_rule() {
                Rule::any_simple_val => s.push_str(&parse_simple_val(inner)),
                Rule::comma => s.push_str(", "),
                Rule::newline => (),
                _ => {
                    if DEBUG {
                        println!("{:?}", inner);
                    }
                }
            }
        }
        s.push(']');
        s
    }

    fn parse_val(p: Pair<Rule>) -> String {
        let inner = p.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::any_simple_val => return parse_simple_val(inner),
            Rule::array => return parse_array(inner),
            _ => {
                if DEBUG {
                    println!("{:?}", inner)
                }
                String::new()
            }
        }
    }

    fn parse_kv(p: Pair<Rule>) -> RawKV {
        let mut namespace = Vec::new();
        let mut key: String = String::from("");
        let mut value: String = String::from("");
        for inner in p.into_inner() {
            match inner.as_rule() {
                Rule::any_key => {
                    let key_inner = inner.into_inner().next().unwrap();
                    match key_inner.as_rule() {
                        Rule::dotted_key => {
                            let res = parse_dotted_key(key_inner);
                            namespace = res.0;
                            key = res.1;
                        },
                        Rule::simple_key => {
                            key = parse_simple_key(key_inner);
                            namespace = Vec::new();
                        },
                        _ => {
                            if DEBUG {
                                println!("{:?}", key_inner);
                            }
                        },
                    };
                },
                Rule::any_val => value = parse_val(inner),
                _ => {
                    if DEBUG {
                        println!("{:?}", inner);
                    }
                }
            }
        }
        RawKV {
            namespace,
            key,
            value,
        }
    }

    pub fn parse_s(s: String) -> Vec<RawKV> {
        let parsed = TomlParser::parse(Rule::file, &s).expect("parsing error").next().unwrap();
        let mut kvs = Vec::new();
        for line in parsed.into_inner() {
            match line.as_rule() {
                Rule::kv => kvs.push(parse_kv(line)),
                _ => {
                    if DEBUG {
                        println!("{:?}", line);
                    }
                }
            }
        }
        kvs
    }
}
