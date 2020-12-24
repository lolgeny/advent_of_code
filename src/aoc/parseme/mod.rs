use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

pub enum ParseRule {
    Constant(String, bool),
    Word(String, bool),
    Condition(String, Box<dyn Fn(char)->bool>, bool),
    Integer(String, bool),
    Either(String, Vec<Parser>)
}
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ParseResult {
    String(String),
    Integer(i64),
    Compound(HashMap<String, ParseResult>)
}
impl ParseResult {
    pub fn as_str(&self) -> String {
        match self {
            Self::String(x) => x.clone(),
            _ => panic!("Cannot convert to string")
        }
    }
    pub fn as_int(&self) -> i64 {
        match self {
            Self::Integer(x) => *x,
            _ => panic!("Cannot convert to int")
        }
    }
    pub fn as_compound(&self) -> HashMap<String, ParseResult> {
        match self {
            Self::Compound(x) => x.clone(),
            _ => panic!("Cannot convert to compound")
        }
    }
}

pub struct Parser {
    pub rules: Vec<ParseRule>
}
impl Parser {
    pub fn parse(&self, input: String) -> Option<HashMap<String, ParseResult>> {
        self.parse_iter(&mut input.chars().peekable())
    }
    fn parse_iter(&self, i: &mut Peekable<Chars>) -> Option<HashMap<String, ParseResult>> {
        let mut vars = HashMap::new();
        for rule in self.rules.iter() {
            match rule {
                ParseRule::Constant(val, space) => {
                    let mut read = String::new();
                    for _ in 0..val.len() {
                        read.push(i.next()?);
                    }
                    if read != *val {
                        return None
                    }
                    if *space {
                        if let Some(next) = i.next() {
                            if next != ' ' {return None}
                        }
                    }
                }
                ParseRule::Word(name, space) => {
                    let mut value = String::new();
                    while let Some(&next) = i.peek() {
                        if !next.is_alphabetic() {break}
                        value.push(next);
                        i.next();
                    }
                    vars.insert(name.clone(), ParseResult::String(value));
                    if *space {
                        if let Some(next) = i.next() {
                            if next != ' ' {return None}
                        }
                    }
                }
                ParseRule::Integer(name, space) => {
                    let mut value = String::new();
                    while let Some(&next) = i.peek() {
                        if !next.is_digit(10) {break}
                        value.push(next);
                        i.next();
                    }
                    vars.insert(name.clone(), ParseResult::Integer(value.parse().unwrap()));
                    if *space {
                        if let Some(next) = i.next() {
                            if next != ' ' {return None}
                        }
                    }
                }
                ParseRule::Condition(name, cond, space) => {
                    let mut value = String::new();
                    while let Some(&next) = i.peek() {
                        if !(*cond)(next) {break}
                        value.push(next);
                        i.next();
                    }
                    vars.insert(name.clone(), ParseResult::String(value));
                    if *space {
                        if let Some(next) = i.next() {
                            if next != ' ' {return None}
                        }
                    }
                }
                ParseRule::Either(name, options) => {
                    if let Some(a) = options[0].parse_iter(i) {
                        vars.insert(name.clone(), ParseResult::Compound(a));
                    } else {
                        vars.insert(name.clone(), ParseResult::Compound(options[1].parse_iter(i)?));
                    }
                }
            }
        }
        Some(vars)
    }
}

#[macro_export]
macro_rules! parse {
    (% rule $rules:ident $constant:literal nospace $($rest: tt)*) => { // constant rule
        $rules.push($crate::parseme::ParseRule::Constant($constant.into(), false));
        parse!(%rule $rules $($rest)*)
    };
    (% rule $rules:ident $constant:literal $($rest: tt)*) => { // constant rule
        $rules.push($crate::parseme::ParseRule::Constant($constant.into(), true));
        parse!(%rule $rules $($rest)*)
    };
    (% rule $rules:ident word $name:ident nospace $($rest: tt)*) => { // word rule
        $rules.push($crate::parseme::ParseRule::Word(stringify!($name).into(), false));
        parse!(%rule $rules $($rest)*)
    };
    (% rule $rules:ident word $name:ident $($rest: tt)*) => { // word rule
        $rules.push($crate::parseme::ParseRule::Word(stringify!($name).into(), true));
        parse!(%rule $rules $($rest)*)
    };
    (% rule $rules:ident custom $name:ident ($fn:expr) nospace $($rest: tt)*) => { // cond rule
        $rules.push($crate::parseme::ParseRule::Condition(stringify!($name).into(), Box::new($fn), false));
        parse!(%rule $rules $($rest)*)
    };
    (% rule $rules:ident custom $name:ident ($fn:expr) $($rest: tt)*) => { // cond rule
        $rules.push($crate::parseme::ParseRule::Condition(stringify!($name).into(), Box::new($fn), true));
        parse!(%rule $rules $($rest)*)
    };
    (% rule $rules:ident either $name:ident {$($a: tt)*} or {$($b: tt)*} $($rest: tt)*) => {
        let values = Vec::new();
        parse!(%rule values $($a)*);
        parse!(%rule values $($b)*);
        $rules.push(
            $crate::parseme::ParseRule::Either(
                stringify!($name).into(),
                Vec::from(
                    values.into_iter().map(|x| $crate::parseme::Parser {rules: Vec::from([x])}).collect()
                )
            )
        );
        parse!(%rule $rules $($rest)*)
    };
    (% rule $rules:ident int $name:ident nospace $($rest: tt)*) => { // int rule
        $rules.push($crate::parseme::ParseRule::Integer(stringify!($name).into(), false));
        parse!(%rule $rules $($rest)*)
    };
    (% rule $rules:ident int $name:ident $($rest: tt)*) => { // int rule
        $rules.push($crate::parseme::ParseRule::Integer(stringify!($name).into(), true));
        parse!(%rule $rules $($rest)*)
    };
    (% rule $rules:ident) => {};
    (% rule $rule:tt $($rest: tt)*) => {compile_error!(format!("Unknown rule {}", $rule))};
    // main rule
    {
        $($rules: tt)*
    } => {
        $crate::parseme::Parser {
            rules: {
                let mut rules = Vec::new();
                parse!(%rule rules $($rules)*);
                rules
            }
        }
    };
}


#[cfg(test)]
mod test {

    #[test]
    fn test_parse() {
        let parser = parse!{
            "Hello"
            word name
            int age_min nospace
            "-" nospace
            int age_max
            custom mask (char::is_alphanumeric)
            either id {
                word name
            } or {
                int id
            }
        };
        let sample_text = "Hello World 42-46 X1X Ethan".into();
        let parse_result = parser.parse(sample_text).unwrap();
        assert_eq!(parse_result["name"].as_str(), "World".to_owned());
        assert_eq!(parse_result["age_min"].as_int(), 42);
        assert_eq!(parse_result["age_max"].as_int(), 46);
        assert_eq!(parse_result["mask"].as_str(), "X1X");
        assert_eq!(parse_result["id"].as_compound()["name"].as_str(), "Ethan");
    }
}