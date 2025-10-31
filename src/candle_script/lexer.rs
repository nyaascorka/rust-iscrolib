use std::fmt::{self, Display, Debug, Formatter};
use lazy_static::lazy_static;

// LEXICAL ANALYSIS
pub enum Token<T> {
    Num(T),
    Str(T),
    Var(T),

    Op(T), // +, -, *, etc.
    LBr(T), // Left bracket
    RBr(T),
    Keyword(T), // reserved words
    InstrEnd,
}

/*
// <ONLY-FOR-DEBUG!>
*/
impl Debug for Token<Vec<char>> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        return match self {
            Self::Num(token) =>
                write!(f, "Token::Num({:?})", token.iter().collect::<String>()),
            Self::Str(token) =>
                write!(f, "Token::Str({:?})", token.iter().collect::<String>()),
            Self::Var(token) =>
                write!(f, "Token::Var({:?})", token.iter().collect::<String>()),
            // Self::CoreVal(token) =>
            //     write!(f, "Token::CoreVal({:?})", token.iter().collect::<String>()),

            Self::Op(token) =>
                write!(f, "Token::Op({:?})", token.iter().collect::<String>()),
            Self::LBr(token) =>
                write!(f, "Token::LBr({:?})", token.iter().collect::<String>()),
            Self::RBr(token) =>
                write!(f, "Token::RBr({:?})", token.iter().collect::<String>()),
            Self::Keyword(token) =>
                write!(f, "Token::Keyword({:?})", token.iter().collect::<String>()),
            Self::InstrEnd => write!(f, "Token::InstrEnd"),
        };
    }
}
/*
// </ONLY-FOR-DEBUG!>
*/

lazy_static! {
    static ref CORE_VALS: Vec<Vec<char>> = ["input", "print", "help"]
        .into_iter()
        .map(|token| token.chars().collect())
        .collect();
    static ref OPS: Vec<Vec<char>> = [
        "+", "-", "*", "/",
        ">=", "<=", ">", "<", "==", "!=",
        "=",
        "add", "sub", "mul", "div",
    ]
        .into_iter()
        .map(|token| token.chars().collect())
        .collect();
    static ref LBRS: Vec<Vec<char>> = ["("]
        .into_iter()
        .map(|token| token.chars().collect())
        .collect();
    static ref RBRS: Vec<Vec<char>> = [")"]
        .into_iter()
        .map(|token| token.chars().collect())
        .collect();
    static ref QUOTES: Vec<Vec<char>> = ["'", "\""]
        .into_iter()
        .map(|token| token.chars().collect())
        .collect();
    static ref KEYWORDS: Vec<Vec<char>> = [
        "candle",
        "end",
        "if", "else",
        "while",
    ]
        .iter()
        .map(|token| token.chars().collect())
        .collect();
}

impl Token<Vec<char>> {
    // Some(Token::[Num, Str, Var]) or None
    pub fn starts_with(mut input_ref: &[char]) -> (&[char], Option<Self>) {
        match input_ref[0] {
            // SEARCH FOR Token::Num
            ('0'..='9' | '.') => for (i, _) in input_ref.iter().enumerate() {
                let (j, d_option) = (i + 1, input_ref.get(i + 1));
                match d_option {
                    Some('0'..='9' | '.') => continue,
                    _ => return (&input_ref[j..], Some(Self::Num(input_ref[..j].to_vec()))),
                }
            }
            // SEARCH FOR Token::Str
            '"' | '\'' => for (i, _) in input_ref.iter().enumerate() {
                let (j, d_option) = (i + 1, input_ref.get(i + 1));
                match d_option {
                    Some('"') => return (&input_ref[j..], Some(Self::Str(input_ref[..j].to_vec()))),
                    None => {
                        eprintln!("Error: quotation mark never closed. - Iscra-chan. (>_<)");
                        std::process::exit(-1);
                    }
                    _ => continue,
                }
            }
            // SEARCH FOR Token::Var
            '_' | 'a'..='z' | 'A'..='Z' | 'а'..='я' | 'А'..='Я' =>
                for (i, _) in input_ref.iter().enumerate() {
                    let (j, d_option) = (i + 1, input_ref.get(i + 1));
                    match d_option {
                        Some('0'..='9' | '_' | 'a'..='z' | 'A'..='Z' | 'а'..='я' | 'А'..='Я') => continue,
                        _ => return (&input_ref[j..], Some(Self::Var(input_ref[..j].to_vec()))),
                    }
                }
        }
        (input_ref, None)
    }

    pub fn get_vec(mut input_ref: &[char]) -> Vec<Self> {
        let mut tokens_vec = Vec::new();
        // REVEAL ALL THE TOKENS
        'find_another_token:
        while (!input_ref.is_empty()) {
            // <ONLY-FOR-DEBUG>
            // println!("{0:#?}", tokens_vec);
            // </ONLY-FOR-DEBUG>

            /*
            // WHITESPACES
            */
            match input_ref[0] {
                ' ' | '\t' | '\r' => {
                    input_ref = &input_ref[1..];
                    continue;
                }
                '\n' => {
                    tokens_vec.push(Self::InstrEnd);
                    input_ref = &input_ref[1..];
                    continue;
                },
                _ => {},
            }

            /*
            * BUILT-IN-CORE TOKENS, WHICH ARE A PART OF Token::Var:
            */
            for core_vals in CORE_VALS.iter() {
                if input_ref.starts_with(core_vals) {
                    input_ref = &input_ref[core_vals.len()..];
                    tokens_vec.push(Self::Var(core_vals.clone()));
                    continue 'find_another_token;
                }
            }
            /*
            * AUXILIARY TOKENS (ALL THOSE THAT ARE NOT Token::Atom)
            */
            for op in OPS.iter() {
                if input_ref.starts_with(op) {
                    input_ref = &input_ref[op.len()..];
                    tokens_vec.push(Self::Op(op.clone()));
                    continue 'find_another_token;
                }
            }
            for lbr in LBRS.iter() {
                if input_ref.starts_with(lbr) {
                    input_ref = &input_ref[lbr.len()..];
                    tokens_vec.push(Self::LBr(lbr.clone()));
                    continue 'find_another_token;
                }
            }
            for rbr in RBRS.iter() {
                if input_ref.starts_with(rbr) {
                    input_ref = &input_ref[rbr.len()..];
                    tokens_vec.push(Self::RBr(rbr.clone()));
                    continue 'find_another_token;
                }
            }
            for keyword in KEYWORDS.iter() {
                if input_ref.starts_with(keyword) {
                    input_ref = &input_ref[keyword.len()..];
                    tokens_vec.push(Self::Keyword(keyword.clone()));
                    continue 'find_another_token;
                }
            }

            /*
            * FINALLY, ALL MISCELLANEOUS Token::Atom:
            */
            for (i, c) in input_ref.iter().enumerate() {

            }
        }
        return tokens_vec;
    }
}