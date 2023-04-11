pub enum Color {
    Red(String),
    Purple(String),
    Green(String),
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Color::Red(txt) => write!(f, "\x1b[31m{}\x1b[0m", txt),
            Color::Purple(txt) => write!(f, "\x1b[34m{}\x1b[0m", txt),
            Color::Green(txt) => write!(f, "\x1b[1;32m{}\x1b[0m", txt),
        };
    } 
}
