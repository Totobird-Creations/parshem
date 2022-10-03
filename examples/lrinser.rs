use parshem::{tokens, parser};


tokens![LrinserTokens = {

    Identifier[String],
    Integer[i64],
    FloatPoint[f64],

    Plus,
    Minus,
    Astrisk,
    Slash,

    Equals,

    LParen,
    RParen,

    Eol,
    Eof
}];


parser![LrinserParser(LrinserTokens) = program {

    program {
        {equation() Eol}* Eof
    };

    equation {
        addition_expression() Equals addition_expression()
    };

    addition_expression {
        multiplication_expression() {{Plus | Minus} multiplication_expression()}*
    };

    multiplication_expression {
        atom() {{Astrisk | Slash} atom()}*
    };

    atom {
        {
              Identifier[_]
            | Integer[_]
            | FloatPoint[_]
        }
    };

}];


fn main() {
    let result = LrinserParser::parse(LrinserTokens::List::new());
    match result {
        Ok(())   => {},
        Err(err) => {println!("{}", err)}
    }
}
