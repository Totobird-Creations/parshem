use parshem::{tokens, parser};


tokens![VesuviusTokens = {
    Hash,

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
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    LAngle,
    RAngle,

    Dot,
    Colon,
    DoubleColon,
    Pipe,

    Semicolon,
    Eof
}];


parser![VesuviusParser(VesuviusTokens) = program {

    program {
        {declaration() Semicolon}* Eof
    };

    declaration {
        headers() declaration_visibility() {
              library_declaration("extern")
            | library_declaration("module")
            | variable_declaration("cst")
        }
    };

    headers {
        {Hash LBracket header() {Comma header()}* RBracket}*
    };
    header {
        Identifier[_] {LParen Identifier[_] {Comma Identifier[_]}* RParen}?
    };

    declaration_visibility {
        {
              Identifier["public"]
            | Identifier["private"]
            | Identifier["file"]
            | Identifier["project"]
        }
    };

    library_declaration(keyword : String) {
        Identifier[keyword] Identifier[_] {DoubleColon Identifier[_]}*
    };

    variable_declaration(keyword : String) {
        Identifier[keyword] Identifier[_] Equals expression()
    };

    expression {
        {
              function_expression()
            | addition_expression()
        }
    };

    addition_expression {
        multiplication_expression() {{Plus | Minus} multiplication_expression()}*
    };

    multiplication_expression {
        call_or_property_expression() {{Astrisk | Slash} call_or_property_expression()}*
    };

    call_or_property_expression {
        atom() {{
              LParen {expression() {Comma expression()}*}? RParen
            | DoubleColon Identifier[_]
            | Dot Identifier[_]
        }}*
        
    };

    atom {
        {
              Integer[_]
            | FloatPoint[_]
            | Identifier[_]
        }
    };

    function_expression {
        Pipe {Identifier[_] Colon object_type()}* Pipe object_type() block()
    };

    block {
        LBrace {command() Semicolon}* RBrace
    };

    command {
        {
              variable_declaration(_)
            | expression
            | block
        }
    };

    object_type {
        Identifier[_] {DoubleColon Identifier[_]}*
        {LAngle object_type() {Comma object_type()}* RAngle}?
    };

}];


fn main() {
    let result = VesuviusParser::parse(VesuviusTokens::List::new());
    match result {
        Ok(())   => {},
        Err(err) => {println!("{}", err)}
    }
}
