extern crate proc_macro;

use proc_macro::{Delimiter, TokenStream, TokenTree};

#[proc_macro]
pub fn messages(input: TokenStream) -> TokenStream {
    let mut defines = "#[derive(Debug, Clone, PartialEq)]\npub enum Message {\n".to_string();
    let mut num: u16 = 1;
    let mut to_u16 = "impl From<Message> for u16 {\n    fn from(message: Message) -> u16 {\n        use Message::*;\n        match message {\n".to_string();
    let mut from_vec = "impl From<Vec<u16>> for Message {\n    fn from(vec: Vec<u16>) -> Message {\n        use Message::*;\n        match vec[0] {\n".to_string();
    let mut to_vec = "impl From<Message> for Vec<u16> {\n    fn from(message: Message) -> Vec<u16> {\n        use Message::*;\n        let mut out: Vec<u16> = vec![message.clone().into()];\n        out.extend(match message {\n".to_string();

    let mut iter = input.into_iter().peekable();
    while let Some(token) = iter.next() {
        if let TokenTree::Ident(id) = &token {
            let mut blank_pattern = None;
            let mut decleration = token.to_string();
            if let Some(TokenTree::Group(g)) = iter.next_if(|t| matches!(t, TokenTree::Group(_))) {
                let mut blank_pattern_str = "_".to_string();
                if g.delimiter() != Delimiter::Parenthesis {
                    panic!("Expected parenthesis");
                }

                decleration.push('(');
                let mut iter = g.stream().into_iter().peekable();
                while let Some(token) = iter.next() {
                    if let TokenTree::Punct(p) = &token {
                        if p.as_char() == ')' {
                            break;
                        } else if p.as_char() == ',' {
                            decleration.push(p.as_char());
                            blank_pattern_str.push_str(", _");
                        } else {
                            panic!(
                                "Unexpected token (expected ',' or ')'), got '{}'",
                                p.as_char()
                            );
                        }
                    }
                    decleration.push_str(&token.to_string());
                }
                decleration.push(')');
                blank_pattern = Some(blank_pattern_str);
            }
            defines.push_str(&format!("    {},\n", decleration));

            if let Some(pat) = blank_pattern {
                to_u16.push_str(&format!("            {}({}) => {},\n", id, pat, num));
            } else {
                to_u16.push_str(&format!("            {} => {},\n", id, num));
            }

            if let Some(TokenTree::Punct(p)) = iter.next() {
                if p.as_char() == ',' {
                    from_vec.push_str(&format!("            {} => {},\n", num, id));
                    to_vec.push_str(&format!("            {} => vec![{}, 0],\n", id, num));
                } else if p.as_char() == '|' {
                    if let Some(TokenTree::Group(g)) = iter.next() {
                        if g.delimiter() != Delimiter::Brace {
                            panic!("Expected brace");
                        }
                        from_vec.push_str(&format!("            {} => {{\n                let data = &vec[1..];\n{}\n            }},\n", num, g.stream().to_string()));
                    } else {
                        panic!("Expected '{{'");
                    }

                    if let Some(TokenTree::Punct(p)) = iter.next() {
                        if p.as_char() == '|' {
                            if let Some(TokenTree::Group(g)) = iter.next() {
                                if g.delimiter() != Delimiter::Brace {
                                    panic!("Expected brace");
                                }
                                to_vec.push_str(&format!("{},\n", g.stream().to_string()));
                            } else {
                                panic!("Expected '{{'");
                            }
                        } else {
                            panic!("Expected '|'");
                        }
                    } else {
                        panic!("Expected '|'");
                    }
                } else {
                    panic!("Expected ',' or '|'");
                }
            } else {
                panic!("Expected ',' or '|'");
            }

            num += 1;
        } else {
            //panic!("Unexpected token, got '{}'", token.to_string());
        }
    }

    defines.push_str("}\n");
    to_u16.push_str("        }\n    }\n}\n");
    from_vec.push_str("            _ => panic!(\"Invalid message\"),\n        }\n    }\n}\n");
    to_vec.push_str("        });\n    out\n    }\n}\n");
    let out = format!("{}\n\n{}\n\n{}\n\n{}", defines, to_u16, from_vec, to_vec);

    //panic!("{}", out);

    out.parse().unwrap()
}
