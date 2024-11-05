#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use pest::Parser;
    use properties_file_parser::{Grammar, Rule};

    #[test]
    fn test_comment_rule() -> anyhow::Result<()> {
        let parsed_data = Grammar::parse(Rule::comment, "# abcde\n something_else")?.next().ok_or_else( || anyhow!("wrong"))?;
        assert_eq!("# abcde", parsed_data.as_str());
        let another_parsed_data = Grammar::parse(Rule::comment, "! abc")?.next().ok_or_else( || anyhow!("wrong"))?;
        assert_eq!("! abc", another_parsed_data.as_str());
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_wrong_comment_rule(){
        Grammar::parse(Rule::comment, "abcde \n something_else").unwrap();
    }
}