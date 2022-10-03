#![allow(unused_parens)]


use std::fmt;


pub const ALPHABETIC : &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_";


#[derive(Debug)]
enum ParsingSyntaxToken {
    LBrace,
    RBrace,
    Pipe,
    Astrisk,
    Plus,
    Question,
    LayerArgs(String),
    TokenArgs(String),
    Identifier(String)
}


#[derive(Debug, Clone)]
pub struct ParsingSyntaxTree {
    piece : Box<ParsingSyntaxTreePiece>,
    next  : Option<Box<ParsingSyntaxTree>>
}
impl ParsingSyntaxTree {
    pub fn new(piece : ParsingSyntaxTreePiece) -> ParsingSyntaxTree {
        return ParsingSyntaxTree {
            piece : Box::new(piece),
            next  : None
        }
    }
    pub fn put_next(&mut self, piece : ParsingSyntaxTree) {
        match (self.next) {
            Some(ref mut next) => next.put_next(piece),
            None               => self.next = Some(Box::new(piece))
        }
    }
    pub fn gen_snippet(&self) -> String {
        return self.piece.gen_snippet() +
            match (&self.next) {
                Some(next) => next.gen_snippet(),
                None       => String::new()
            }.as_str();
    }
}
impl fmt::Display for ParsingSyntaxTree {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}{}",
            self.piece,
            match (&self.next) {
                Some(next) => format!(" {}", next),
                None       => String::new()
            }
        );
    }
}

#[derive(Debug, Clone)]
pub enum ParsingSyntaxTreePiece {
    OneOf(Vec<ParsingSyntaxTree>),
    ZeroOrMore(ParsingSyntaxTree),
    OneOrMore(ParsingSyntaxTree),
    Optional(ParsingSyntaxTree),
    Token(String, String),
    Layer(String, String)
}
impl ParsingSyntaxTreePiece {
    pub fn gen_snippet(&self) -> String {
        return match (self) {
            ParsingSyntaxTreePiece::OneOf(options) => {
                let mut snippet = format!("{}{}",
                    "let mut snapshot = tokens.snapshot();",
                    "let mut errors = Vec::new();"
                );
                for option in options {
                    snippet += format!("{}{{{}}}{}",
                        format!("match {{{}}}", option.gen_snippet()),
                        "Ok(v) => return Ok(v), Err(e) => return Err(e)",
                        "snapshot.restore();"
                    ).as_str();
                }
                snippet += "Err(super::ParseError::new(super::ParseErrorType::OneOf(Box::new(errors))))";
                snippet
            },
            ParsingSyntaxTreePiece::ZeroOrMore(_piece) => {
                format!("{}",
                    "let mut snapshot = tokens.snapshot();"
                )
            },
            ParsingSyntaxTreePiece::OneOrMore(_piece) => {
                format!("{}",
                    "let mut snapshot = tokens.snapshot();"
                )
            },
            ParsingSyntaxTreePiece::Optional(_piece) => {
                format!("{}",
                    "let mut snapshot = tokens.snapshot();"
                )
            },
            ParsingSyntaxTreePiece::Token(name, args) => {
                let missing_token = format!("Err(super::ParseError::new(super::ParseErrorType::MissingToken(Vec::new(super::Token::Type::{}({})))))", name, args);
                format!("if tokens.end() {{{missing_token}}} else if ! matches!(tokens.get().get_token(), super::Token::Type::{name}({args})) {{{missing_token}}} else {{tokens.next(); Ok(())}}")
            },
            ParsingSyntaxTreePiece::Layer(name, args) => {
                format!("{name}(tokens, {args});")
            },
        }
    }
}
impl fmt::Display for ParsingSyntaxTreePiece {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}",
            match (self) {
                ParsingSyntaxTreePiece::OneOf      (options        ) => format!("{{ {} }}", options.iter().map(|o| format!("{}", o)).collect::<Vec<String>>().join(" | ")),
                ParsingSyntaxTreePiece::ZeroOrMore (piece          ) => format!("{{ {} }}*", piece),
                ParsingSyntaxTreePiece::OneOrMore  (piece          ) => format!("{{ {} }}+", piece),
                ParsingSyntaxTreePiece::Optional   (piece          ) => format!("{{ {} }}?", piece),
                ParsingSyntaxTreePiece::Token      (name    , args ) => format!("{}[{}]", name, args),
                ParsingSyntaxTreePiece::Layer      (name    , args ) => format!("{}({})", name, args)
            }
        );
    }
}



pub fn generate_rule(text : String) -> ParsingSyntaxTree {
    let tokens = generate_rule_tokens(text);
    let tree   = generate_rule_tree(tokens);
    return tree;
}



fn generate_rule_tokens(text : String) -> Vec<ParsingSyntaxToken> {
    let mut index  = 0;
    let mut tokens = Vec::new();
    while (index < text.len()) {
        let ch = text.chars().nth(index).unwrap();
        match (ch) {
            '\n' => {},
            ' '  => {},
            '\t' => {},
            '{'  => tokens.push(ParsingSyntaxToken::LBrace),
            '}'  => tokens.push(ParsingSyntaxToken::RBrace),
            '|'  => tokens.push(ParsingSyntaxToken::Pipe),
            '*'  => tokens.push(ParsingSyntaxToken::Astrisk),
            '+'  => tokens.push(ParsingSyntaxToken::Plus),
            '?'  => tokens.push(ParsingSyntaxToken::Question),
            '('  => tokens.push(generate_rule_tokens_arguments('(', ')', |s| ParsingSyntaxToken::LayerArgs(s), &text, &mut index)),
            '['  => tokens.push(generate_rule_tokens_arguments('[', ']', |s| ParsingSyntaxToken::TokenArgs(s), &text, &mut index)),
            _    => {
                if (ALPHABETIC.contains(ch)) {
                    tokens.push(generate_rule_tokens_identifier(&text, &mut index));
                } else {
                    panic!("Unrecognised character: `{ch}`.");
                }
            }
        }
        index += 1;
    }
    return tokens;
}


fn generate_rule_tokens_identifier(text : &String, index : &mut usize) -> ParsingSyntaxToken {
    let mut identifier = String::new(); 
    while (*index < text.len()) {
        let ch = text.chars().nth(*index).unwrap();
        if (! ALPHABETIC.contains(ch)) {
            break;
        }
        *index += 1;
        identifier += ch.to_string().as_str();
    }
    *index -= 1;
    return ParsingSyntaxToken::Identifier(identifier);
}


fn generate_rule_tokens_arguments<F>(opener : char, closer : char, instantiator : F, text : &String, index : &mut usize) -> ParsingSyntaxToken
    where F : Fn(String) -> ParsingSyntaxToken
{
    *index += 1;
    let mut arguments = String::new();
    let mut depth     = 1;
    while (*index < text.len()) {
        let ch = text.chars().nth(*index).unwrap();
        if (ch == opener) {
            depth += 1;
        } else if (ch == closer) {
            depth -= 1;
            if (depth <= 0) {
                break;
            }
        }
        *index += 1;
        arguments += ch.to_string().as_str();
    }
    return instantiator(arguments);
}



fn generate_rule_tree(tokens : Vec<ParsingSyntaxToken>) -> ParsingSyntaxTree {
    return generate_rule_tree_part(&tokens, &mut 0);
}

fn generate_rule_tree_part(tokens : &Vec<ParsingSyntaxToken>, index : &mut usize) -> ParsingSyntaxTree {
    match (&tokens[*index]) {

        ParsingSyntaxToken::LBrace => {
            *index += 1;
            let mut options = vec![generate_rule_tree_part(tokens, index)];
            while (
                   *index < tokens.len()
                && ! matches!(tokens[*index], ParsingSyntaxToken::RBrace)
            ) {
                if (matches!(tokens[*index], ParsingSyntaxToken::Pipe)) {
                    *index += 1;
                    options.push(generate_rule_tree_part(tokens, index))
                } else {
                    let last = options.len() - 1;
                    options[last].put_next(generate_rule_tree_part(tokens, index));
                }
            }
            if (*index >= tokens.len() || ! matches!(tokens[*index], ParsingSyntaxToken::RBrace)) {
                panic!("Unclosed `{{...` found.");
            }
            *index += 1;
            if (options.len() > 1) {
                ParsingSyntaxTree::new(ParsingSyntaxTreePiece::OneOf(options))
            } else if (*index < tokens.len()) {
                if (matches!(tokens[*index], ParsingSyntaxToken::Astrisk)) {
                    *index += 1;
                    ParsingSyntaxTree::new(ParsingSyntaxTreePiece::ZeroOrMore(options[0].clone()))
                } else if (matches!(tokens[*index], ParsingSyntaxToken::Plus)) {
                    *index += 1;
                    ParsingSyntaxTree::new(ParsingSyntaxTreePiece::OneOrMore(options[0].clone()))
                } else if (matches!(tokens[*index], ParsingSyntaxToken::Question)) {
                    *index += 1;
                    ParsingSyntaxTree::new(ParsingSyntaxTreePiece::Optional(options[0].clone()))
                } else {
                    options[0].clone()
                }
            } else {
                options[0].clone()
            }
        },

        ParsingSyntaxToken::Identifier(identifier) => {
            *index += 1;
            return match (&tokens[*index]) {
                ParsingSyntaxToken::LayerArgs(args) => {
                    *index += 1;
                    ParsingSyntaxTree::new(ParsingSyntaxTreePiece::Layer(String::from(identifier), String::from(args)))
                },
                ParsingSyntaxToken::TokenArgs(args) => {
                    *index += 1;
                    ParsingSyntaxTree::new(ParsingSyntaxTreePiece::Token(String::from(identifier), String::from(args)))
                },
                _ => {
                    ParsingSyntaxTree::new(ParsingSyntaxTreePiece::Token(String::from(identifier), String::new()))
                }
            }
        },

        _ => panic!("Expected `{{...` or identifier not found.")

    }
}
