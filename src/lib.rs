use crate::PropertyParseError::ParsingInputError;
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum PropertyParseError{
    #[error("Input is incorrect")]
    ParsingInputError
}

pub fn parse_properties(unparsed: &str) -> Result<Vec<Property>, PropertyParseError>{
    let parsed = Grammar::parse(Rule::file, unparsed);
    let parsed_unwrapped;
    match parsed {
        Ok(_) => { parsed_unwrapped = parsed.unwrap().next()}
        Err(_) => {return Err(ParsingInputError)}
    }

    let mut properties = vec![];

    for property_pair in parsed_unwrapped.unwrap().into_inner() {
        let mut inner = property_pair.into_inner();
        properties.push(Property {key: inner.next().unwrap().as_str().to_string(),
            value: inner.next().unwrap().as_str().to_string()});
    }
    Ok(properties)
}

pub fn parse_properties_as_string(unparsed: &str) -> Result<String, PropertyParseError>{
    let parsed = parse_properties(unparsed);
    let parsed_unwrapped;
    match parsed {
        Ok(_) => { parsed_unwrapped = parsed?}
        Err(_) => {return Err(ParsingInputError)}
    }
    let mut res = String::new();
    for i in 0..parsed_unwrapped.len(){
        res += "key: ";
        res += parsed_unwrapped[i].key.as_str();
        res += ", value: ";
        res += parsed_unwrapped[i].value.as_str();
        if i < parsed_unwrapped.len() - 1 {
            res += "\n";
        }
    }
    Ok(res)
}