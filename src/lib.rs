use std::{convert::TryFrom, error, fmt, str::FromStr};

static RENAME_RULES: &[(&str, RenameRule)] = &[
    ("lowercase", RenameRule::LowerCase),
    ("UPPERCASE", RenameRule::UpperCase),
    ("PascalCase", RenameRule::PascalCase),
    ("camelCase", RenameRule::CamelCase),
    ("snake_case", RenameRule::SnakeCase),
    ("SCREAMING_SNAKE_CASE", RenameRule::ScreamingSnakeCase),
    ("kebab-case", RenameRule::KebabCase),
    ("SCREAMING-KEBAB-CASE", RenameRule::ScreamingKebabCase),
];

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum RenameRule {
    LowerCase,
    UpperCase,
    PascalCase,
    CamelCase,
    SnakeCase,
    ScreamingSnakeCase,
    KebabCase,
    ScreamingKebabCase,
}

impl RenameRule {
    pub fn from_rename_all_str(s: &str) -> Result<Self, ParseError> {
        for (name, rule) in RENAME_RULES {
            if s == *name {
                return Ok(rule.to_owned());
            }
        }
        Err(ParseError::Unknown(s.to_owned()))
    }
    pub fn to_rename_all_str(&self) -> &'static str {
        for (name, rule) in RENAME_RULES {
            if rule == self {
                return name;
            }
        }
        unreachable!()
    }
}

impl FromStr for RenameRule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_rename_all_str(s)
    }
}
impl TryFrom<&str> for RenameRule {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}
impl fmt::Display for RenameRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_rename_all_str())
    }
}

//
#[derive(Debug)]
pub enum ParseError {
    Unknown(String),
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_rename_all_str_and_to_rename_all_str() {
        for (name, rule) in RENAME_RULES {
            assert_eq!(&RenameRule::from_rename_all_str(name).unwrap(), rule);
            assert_eq!(&rule.to_rename_all_str(), name);
        }

        match RenameRule::from_rename_all_str("foo") {
            Ok(_) => panic!(""),
            Err(ParseError::Unknown(s)) => assert_eq!(s, "foo"),
        }
    }

    #[test]
    fn test_impl_trait() -> Result<(), Box<dyn error::Error>> {
        let (name, rule) = RENAME_RULES.first().unwrap().to_owned();
        assert_eq!(RenameRule::from_str(name)?, rule);
        assert_eq!(name.parse::<RenameRule>()?, rule);
        assert_eq!(RenameRule::try_from(name)?, rule);
        assert_eq!(rule.to_string(), name);

        Ok(())
    }
}
