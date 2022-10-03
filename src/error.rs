use std::fmt;


pub struct ParseError<T : fmt::Debug> {
    error : ParseErrorType<T>
}
impl<T : fmt::Debug> ParseError<T> {
    pub fn new(error : ParseErrorType<T>) -> ParseError<T> {
        return ParseError {
            error
        };
    }
}
impl<T : fmt::Debug> ParseError<T> {
    pub fn error_code(&self) -> u16 {
        return self.error.index() + 1;
    }
}
impl<T : fmt::Debug> fmt::Display for ParseError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.error);
    }
}


#[repr(u16)]
pub enum ParseErrorType<T : fmt::Debug> {
    MissingToken(Vec<T>)
}
impl<T : fmt::Debug> ParseErrorType<T> {
    fn index(&self) -> u16 {
        return unsafe {*(self as *const Self as *const u16)};
    }
}
impl<T : fmt::Debug> fmt::Display for ParseErrorType<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", match (self) {

            ParseErrorType::MissingToken(expected) => {
                let mut strings = expected
                    .iter().map(|v| format!("`{:?}`", v))
                    .collect::<Vec<String>>();
                if (strings.len() > 1) {
                    strings.insert(strings.len() - 1, String::from("or"));
                }
                format!("Token {} missing.", strings.join(" "))
            }
            
        });
    }
}

