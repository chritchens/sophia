use super::{FunAppForm, FunAppFormParam};
use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct AttrsForm {
    pub tokens: Tokens,
    pub name: String,
    pub values: Vec<String>,
}

impl AttrsForm {
    pub fn new() -> AttrsForm {
        AttrsForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn from_fun_app(fun_app: &FunAppForm) -> Result<AttrsForm> {
        if fun_app.name != "attrs" {
            return Err(Error::Semantic(SemanticError {
                loc: fun_app.loc(),
                desc: "expected a attrs keyword".into(),
            }));
        }

        if fun_app.params.len() != 2 {
            return Err(Error::Semantic(SemanticError {
                loc: fun_app.loc(),
                desc: "expected a name and a product of symbols".into(),
            }));
        }

        let mut attrs = AttrsForm::new();
        attrs.tokens = fun_app.tokens.clone();

        match fun_app.params[0].clone() {
            FunAppFormParam::Symbol(symbol) => {
                attrs.name = symbol;
            }
            _ => {
                return Err(Error::Semantic(SemanticError {
                    loc: fun_app.loc(),
                    desc: "expected a symbol".into(),
                }));
            }
        }

        match fun_app.params[1].clone() {
            FunAppFormParam::FunApp(form) => {
                if form.name != "prod" {
                    return Err(Error::Semantic(SemanticError {
                        loc: fun_app.loc(),
                        desc: "expected a product of symbols".into(),
                    }));
                }

                for param in form.params.iter() {
                    match param {
                        FunAppFormParam::Symbol(symbol) => {
                            attrs.values.push(symbol.clone());
                        }
                        _ => {
                            return Err(Error::Semantic(SemanticError {
                                loc: fun_app.loc(),
                                desc: "expected a symbol".into(),
                            }));
                        }
                    }
                }

                Ok(attrs)
            }
            _ => Err(Error::Semantic(SemanticError {
                loc: fun_app.loc(),
                desc: "expected a product of symbols".into(),
            })),
        }
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<AttrsForm> {
        let fun_app = FunAppForm::from_tokens(tokens)?;

        AttrsForm::from_fun_app(&fun_app)
    }

    pub fn from_str(s: &str) -> Result<AttrsForm> {
        let tokens = Tokens::from_str(s)?;

        AttrsForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!(
            "(attrs {} (prod {}))",
            self.name.to_string(),
            self.values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl fmt::Display for AttrsForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn attrs_form_from_str() {
        use super::AttrsForm;

        let mut s = "(attrs x (prod attr))";

        let mut res = AttrsForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name, "x".to_string());
        assert_eq!(form.values, vec!["attr".to_string()]);
        assert_eq!(form.to_string(), s.to_string());

        s = "(attrs y (prod attr1 attr2 attr3))";

        res = AttrsForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "y".to_string());
        assert_eq!(
            form.values,
            vec![
                "attr1".to_string(),
                "attr2".to_string(),
                "attr3".to_string(),
            ]
        );
    }
}