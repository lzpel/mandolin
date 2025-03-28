
pub fn jp_encode(content: &str) -> String {
    content.replace("~", "~0").replace("/", "~1") // RFC6901
}
pub fn jp_decode(content: &str) -> String {
    content.replace("~0", "~").replace("~1", "/") // RFC6901
}
pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut flag = false;
    for c in s.chars() {
        if c.is_ascii_alphanumeric() == false {
            flag = true; //前置判定
        } else {
            flag |= c.is_uppercase();
            flag &= !result.is_empty();
            //↑当該判定
            if flag {
                result.push('_')
            }
            result.push(c.to_ascii_lowercase());
            flag = false;
        }
    }
    result
}
pub fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut flag = false;
    for c in s.chars() {
        if c.is_ascii_alphanumeric() == false {
            flag = true; //前置判定
        } else {
            flag |= result.is_empty();
            //↑当該判定
            if flag {
                result.push(c.to_ascii_uppercase())
            } else {
                result.push(c)
            }
            flag = false;
        }
    }
    result
}