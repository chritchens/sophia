use crate::error::{Error, ParsingError};
use crate::result::Result;
use crate::token::Token;
use crate::token::TokenKind;
use crate::tokens::Tokens;
use crate::value::Value;
use std::convert;
use std::fs;
use std::iter;
use std::ops;
use std::path::Path;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Values(Vec<Value>);

impl Values {
    pub fn new() -> Self {
        Values::default()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, value: Value) {
        self.0.push(value)
    }

    pub fn from_str(s: &str) -> Result<Self> {
        let mut tokens = Tokens::from_str(s)?;

        tokens = tokens
            .into_iter()
            .filter(|token| token.kind != TokenKind::Comment && token.kind != TokenKind::DocComment)
            .collect();

        let mut values = Values::new();
        let mut form: Vec<Token> = vec![];
        let mut form_count = 0;

        for token in tokens.into_iter() {
            match token.kind {
                TokenKind::Comment => {
                    return Err(Error::Parsing(ParsingError {
                        loc: Some(token.chunks.unwrap()[0].loc.clone()),
                        desc: "unexpected comment token".into(),
                    }));
                }
                TokenKind::DocComment => {
                    return Err(Error::Parsing(ParsingError {
                        loc: Some(token.chunks.unwrap()[0].loc.clone()),
                        desc: "unexpected doc comment token".into(),
                    }));
                }
                TokenKind::EmptyLiteral => {
                    if form_count != 0 {
                        form.push(token);
                    } else {
                        let value = Value::new_empty(token)?;
                        values.push(value);
                    }
                }
                TokenKind::Keyword => {
                    if form_count != 0 {
                        form.push(token);
                    } else {
                        let value = Value::new_keyword(token)?;
                        values.push(value);
                    }
                }
                TokenKind::UIntLiteral => {
                    if form_count != 0 {
                        form.push(token);
                    } else {
                        let value = Value::new_uint(token)?;
                        values.push(value);
                    }
                }
                TokenKind::IntLiteral => {
                    if form_count != 0 {
                        form.push(token);
                    } else {
                        let value = Value::new_int(token)?;
                        values.push(value);
                    }
                }
                TokenKind::FloatLiteral => {
                    if form_count != 0 {
                        form.push(token);
                    } else {
                        let value = Value::new_float(token)?;
                        values.push(value);
                    }
                }
                TokenKind::CharLiteral => {
                    if form_count != 0 {
                        form.push(token);
                    } else {
                        let value = Value::new_char(token)?;
                        values.push(value);
                    }
                }
                TokenKind::StringLiteral => {
                    if form_count != 0 {
                        form.push(token);
                    } else {
                        let value = Value::new_string(token)?;
                        values.push(value);
                    }
                }
                TokenKind::ValueSymbol | TokenKind::TypeSymbol => {
                    if form_count != 0 {
                        form.push(token);
                    } else {
                        let value = Value::new_symbol(token)?;
                        values.push(value);
                    }
                }
                TokenKind::FormStart => {
                    form_count += 1;
                    form.push(token);
                }
                TokenKind::FormEnd => {
                    form_count -= 1;
                    form.push(token);

                    if form_count == 0 {
                        let value = Value::new_app(form)?;
                        values.push(value);

                        form = Vec::new();
                    }
                }
            }
        }

        Ok(values)
    }

    pub fn from_string(s: String) -> Result<Self> {
        Self::from_str(&s)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::from_string(fs::read_to_string(path)?)
    }
}

impl ops::Index<usize> for Values {
    type Output = Value;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl iter::IntoIterator for Values {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl iter::FromIterator<Value> for Values {
    fn from_iter<I: iter::IntoIterator<Item = Value>>(iter: I) -> Self {
        let mut values = Values::new();

        for value in iter {
            values.push(value);
        }

        values
    }
}

impl convert::From<Vec<Value>> for Values {
    fn from(values: Vec<Value>) -> Self {
        Values(values)
    }
}

impl std::str::FromStr for Values {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Values::from_str(s)
    }
}

impl convert::TryFrom<String> for Values {
    type Error = Error;

    fn try_from(s: String) -> Result<Self> {
        Values::from_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ignore_comment_tokens() {
        use super::Values;

        let s = "# comment\n#! doc comment";

        let values = Values::from_str(s).unwrap();

        assert!(values.is_empty());
    }

    #[test]
    fn empty_value() {
        use super::Values;
        use crate::typing::Type;
        use crate::value::PrimValue;

        let s = "()";

        let values = Values::from_str(s).unwrap();

        assert_eq!(values.len(), 1);
        assert_eq!(values[0].typing, Some(Type::Empty));
        assert_eq!(values[0].value, Some(PrimValue::Empty));
    }

    #[test]
    fn keyword_value() {
        use super::Values;
        use crate::typing::Type;

        let s = "defsig";

        let values = Values::from_str(s).unwrap();

        assert_eq!(values.len(), 1);
        assert_eq!(values[0].name, Some(s.into()));
        assert_eq!(values[0].typing, Some(Type::Builtin));
    }

    #[test]
    fn uint_value() {
        use super::Values;
        use crate::typing::Type;
        use crate::value::PrimValue;

        let s = "b101010";

        let values = Values::from_str(s).unwrap();

        assert_eq!(values.len(), 1);
        assert_eq!(values[0].typing, Some(Type::UInt));
        assert_eq!(values[0].value, Some(PrimValue::new_uint(s)));
    }

    #[test]
    fn int_value() {
        use super::Values;
        use crate::typing::Type;
        use crate::value::PrimValue;

        let s = "-3290";

        let values = Values::from_str(s).unwrap();

        assert_eq!(values.len(), 1);
        assert_eq!(values[0].typing, Some(Type::Int));
        assert_eq!(values[0].value, Some(PrimValue::new_int(s)));
    }

    #[test]
    fn float_value() {
        use super::Values;
        use crate::typing::Type;
        use crate::value::PrimValue;

        let s = "+0.432E-100";

        let values = Values::from_str(s).unwrap();

        assert_eq!(values.len(), 1);
        assert_eq!(values[0].typing, Some(Type::Float));
        assert_eq!(values[0].value, Some(PrimValue::new_float(s)));
    }

    #[test]
    fn char_value() {
        use super::Values;
        use crate::typing::Type;
        use crate::value::PrimValue;

        let s = "'\''";

        let values = Values::from_str(s).unwrap();

        assert_eq!(values.len(), 1);
        assert_eq!(values[0].typing, Some(Type::Char));
        assert_eq!(values[0].value, Some(PrimValue::new_char("'")));
    }

    #[test]
    fn string_value() {
        use super::Values;
        use crate::typing::Type;
        use crate::value::PrimValue;

        let s = "\"\\\"\"";

        let values = Values::from_str(s).unwrap();

        assert_eq!(values.len(), 1);
        assert_eq!(values[0].typing, Some(Type::String));
        assert_eq!(values[0].value, Some(PrimValue::new_string("\\\"")));
    }

    #[test]
    fn symbol_value() {
        use super::Values;
        use crate::typing::Type;

        let mut s = "Int";

        let mut values = Values::from_str(s).unwrap();

        assert_eq!(values.len(), 1);
        assert_eq!(values[0].typing, Some(Type::Type));

        s = "square";

        values = Values::from_str(s).unwrap();

        assert_eq!(values.len(), 1);
        assert_eq!(values[0].typing, Some(Type::Unknown));
    }

    #[test]
    fn fun_value() {
        use super::Values;
        use crate::typing::Type;

        let s = "(+ 1 (sum (square 3) 4))";

        let values = Values::from_str(s).unwrap();

        assert_eq!(values.len(), 1);
        assert_eq!(values[0].name, Some("+".into()));
        assert_eq!(
            values[0].typing,
            Some(Type::App(vec![
                Type::Unknown,
                Type::UInt,
                Type::App(vec![
                    Type::Unknown,
                    Type::App(vec![Type::Unknown, Type::UInt]),
                    Type::UInt
                ])
            ]))
        );
        assert_eq!(values[0].value, None);
        assert_eq!(values[0].children.len(), 3);
    }

    #[test]
    fn values_from_file() {
        use super::Values;
        use crate::typing::Type;
        use std::path::Path;

        let path = Path::new("./examples/sum.sp");

        let values = Values::from_file(path).unwrap();

        assert_eq!(values.len(), 3);
        assert_eq!(
            values[2].typing,
            Some(Type::App(vec![
                Type::Unknown,
                Type::App(vec![Type::Unknown, Type::UInt, Type::UInt])
            ]))
        );
    }
}
