use regex::Regex;
pub(crate) fn camel_case(text: &str) -> String {
    let re = Regex::new(r"_+(.)").unwrap();
    let text_with_capital = format!("_{}", text);
    let result = re.replace_all(text_with_capital.as_str(), |caps: &regex::Captures| {
        format!("{}", &caps[1].to_uppercase())
    });
    result.to_string()
}