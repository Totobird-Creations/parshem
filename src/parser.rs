#[macro_export]
macro_rules! parser {

    [$parser_name:ident($($token_type:ident)::*) = $entry_level_name:ident {
        $($level_name:ident $(($($level_arg_name:ident : $level_arg_type:ty),*))? {
            $($rule:tt)+
        };)*
    }]
    
    => {
        #[allow(non_snake_case)]
        mod $parser_name {
            use super::$($token_type)::* as Token;
            mod rules {$(
                pub fn $level_name(mut tokens : super::Token::List, $($($level_arg_name : $level_arg_type),*)?) -> Result<(), $crate::error::ParseError> {
                    let mut expected = Vec::new();
                    $crate::parshem_proc::generate_rule!({$($rule)+});
                    return Err($crate::error::ParseError::MissingToken(expected));
                }
            )*}
            pub fn parse(mut tokens : Token::List) -> Result<(), $crate::error::ParseError> {
                tokens.reset();
                return rules::$entry_level_name(tokens);
            }
        }
    };
}