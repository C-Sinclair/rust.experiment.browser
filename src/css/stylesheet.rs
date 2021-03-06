use std::fmt;
use std::default::Default;

pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

pub struct Selector {
    pub simple: Vec<SimpleSelector>,
    pub combinators: Vec<char>,
}

pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
}

pub struct Declaration {
    pub property: String,
    pub value: Value,
}

pub enum Value {
    Colour(Colour),
    Length(f32, Unit),
    Other(String),
} 

pub enum Unit {
    Em, 
    Ex,
    Ch, 
    Rem, 
    Vh, 
    Vw, 
    Vmin, 
    Vmax, 
    Px, 
    Mm, 
    Q, 
    Cm, 
    In, 
    Pt, 
    Pc, 
    Pct,
}

pub struct Colour {
    pub r: f32,
    pub b: f32,
    pub g: f32,
    pub a: f32,
}

impl Stylesheet {
    pub fn new(rules: Vec<Rule>) -> Stylesheet {
        Stylesheet { rules }
    }
}
impl Default for Stylesheet {
    fn default() -> Self {
        Stylesheet { rules: Vec::new() }
    }
}
impl fmt::Debug for Stylesheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rule_result = String::new();
        for rule in &self.rules {
            if rule_result.len() > 0 {
                rule_result.push_str("\n\n");
            }
            rule_result.push_str(&format!("{:?}", rule));
        }
        write!(f, "{}", rule_result)
    }
}