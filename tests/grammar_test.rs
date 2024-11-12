#[cfg(test)]
mod tests {
    use pest::Parser;
    use properties_file_parser::{parse_properties, Grammar, Property, Rule};

    #[test]
    fn test_comment_parsing() -> anyhow::Result<()> {
        assert!(!Grammar::parse(Rule::comment, "# abcde\n something_else").is_err());
        assert!(!Grammar::parse(Rule::comment, "! abc").is_err());
        assert!(Grammar::parse(Rule::comment, "abcde \n something_else").is_err());
        let parsed = parse_properties("#abcde \n key:val")?;
        let true_value = vec![Property{key: "key".to_string(),
            value: "val".to_string()}];
        assert_eq!(parsed, true_value);
        Ok(())
    }

    #[test]
    fn test_spaces_parsing() -> anyhow::Result<()> {
        assert!(!Grammar::parse(Rule::spaces, " ").is_err());
        assert!(!Grammar::parse(Rule::spaces, "     ").is_err());
        assert!(Grammar::parse(Rule::spaces, "row").is_err());
        Ok(())
    }

    #[test]
    fn test_property_parsing() -> anyhow::Result<()> {
        assert!(!Grammar::parse(Rule::property, "key :  val").is_err());
        assert!(!Grammar::parse(Rule::property, "key=val").is_err());
        assert!(!Grammar::parse(Rule::property, "key val").is_err());
        assert!(!Grammar::parse(Rule::property, "key").is_err());
        assert!(Grammar::parse(Rule::property, "\n").is_err());
        let parsed = parse_properties("key:val")?;
        let true_value = parse_properties("key = val")?;
        assert_eq!(parsed, true_value);
        Ok(())
    }

    #[test]
    fn test_key_parsing() -> anyhow::Result<()>{
        assert!(!Grammar::parse(Rule::key, "key").is_err());
        assert!(Grammar::parse(Rule::key, " ").is_err());
        Ok(())
    }

    #[test]
    fn test_value_parsing() -> anyhow::Result<()>{
        assert!(!Grammar::parse(Rule::value, "value").is_err());
        assert!(!Grammar::parse(Rule::value, "value\n").is_err());
        assert!(!Grammar::parse(Rule::value, "").is_err());
        Ok(())
    }

    #[test]
    fn test_file_parsing() -> anyhow::Result<()>{
        assert!(!Grammar::parse(Rule::file, "key1  = value1\n\
        key2 value2\n\
        key3:\n\
        key4").is_err());
        assert!(!Grammar::parse(Rule::file, "").is_err());
        assert!(!Grammar::parse(Rule::file, "#comment\n\
        \n\
        key = value").is_err());

        let parsed = parse_properties("key1  = val1\n\
        key2 val2\n\
        key3:\n\
        key4")?;
        let true_value = vec![Property{key: "key1".to_string(), value: "val1".to_string()},
                              Property{key: "key2".to_string(), value: "val2".to_string()},
                              Property{key: "key3".to_string(), value: "".to_string()},
                              Property{key: "key4".to_string(), value: "".to_string()}];
        assert_eq!(parsed, true_value);
        Ok(())
    }

    #[test]
    fn test_silent_eoi_parsing() -> anyhow::Result<()>{
        assert!(!Grammar::parse(Rule::silentEOI, "").is_err());
        assert!(Grammar::parse(Rule::silentEOI, "i").is_err());
        assert!(Grammar::parse(Rule::silentEOI, " ").is_err());
        Ok(())
    }
}