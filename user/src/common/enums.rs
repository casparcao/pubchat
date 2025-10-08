use std::fmt::Display;


#[derive(sqlx::Type, PartialEq, PartialOrd, Debug, Clone, Copy)]
#[repr(i32)]
pub enum Gender{
    M = 1,
    F = 2,
    U = 3
}

impl From<String> for Gender{
    fn from(s: String) -> Self {
        match s.as_str() {
            "M" => Gender::M,
            "F" => Gender::F,
            "U" => Gender::U,
            _ => Gender::U
        }
    }
}

impl From<&str> for Gender{
    fn from(s: &str) -> Self {
        match s {
            "M" => Gender::M,
            "F" => Gender::F,
            "U" => Gender::U,
            _ => Gender::U
        }
    }
}

impl Display for Gender{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::M => write!(f, "M"),
            Gender::F => write!(f, "F"),
            Gender::U => write!(f, "U")
        }
    }
}