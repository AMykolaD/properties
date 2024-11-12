use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[derive(Debug)]
pub struct Property {
    pub key: String,
    pub value: String,
}

impl PartialEq for Property {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value == other.value
    }
}

pub fn parse_properties(unparsed: &str) -> anyhow::Result<Vec<Property>>{
    let parsed = Grammar::parse(Rule::file, unparsed)?.next();

    let mut properties = vec![];

    for property_pair in parsed.unwrap().into_inner() {
        let mut inner = property_pair.into_inner();
        properties.push(Property {key: inner.next().unwrap().as_str().to_string(),
            value: inner.next().unwrap().as_str().to_string()});
    }
    Ok(properties)
}

pub fn parse_properties_as_string(unparsed: &str) -> anyhow::Result<String>{
    let parsed = parse_properties(unparsed)?;
    let mut res = String::new();
    for i in 0..parsed.len(){
        res += "key: ";
        res += parsed[i].key.as_str();
        res += ", value: ";
        res += parsed[i].value.as_str();
        if i < parsed.len() - 1 {
            res += "\n";
        }
    }
    Ok(res)
}