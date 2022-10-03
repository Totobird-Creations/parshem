use std::fmt;


pub enum ParseError {
    MissingToken(Vec<String>)
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", match (self) {
            ParseError::MissingToken(expected) => {
                let mut strings = expected
                    .iter().map(|v| format!("`{}`", v))
                    .collect::<Vec<String>>();
                if (strings.len() > 1) {
                    strings.insert(strings.len() - 1, String::from("or"));
                }
                format!("Token {} missing.", strings.join(" "))
            }
        });
    }
}

