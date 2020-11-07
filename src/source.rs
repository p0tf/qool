use std::fmt;

/// The Data Source
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Source {
    /// The text from the arguments.
    Text(String),
    /// Read from the standard input.
    Stdin,
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Source::Text(s) => write!(f, "{:?}", s),
            Source::Stdin => write!(f, "<stdin>")
        }
    }
}

impl Source {
    /// Create `Source` with the arguments.
    pub fn new(text: Option<String>) -> Self {
        if let Some(t) = text {
            Self::Text(t)
        } else {
            Self::Stdin
        }
    }

    /// Yield `Vec<u8>` from `Source`.
    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            Self::Text(s) => text(s),
            Self::Stdin => stdin().unwrap_or_else(|e| {
                log::error!("{}", e);
                std::process::exit(e.raw_os_error().unwrap_or(1));
            }),
        }
    }
}

fn text(s: String) -> Vec<u8> {
    s.into_bytes()
}

fn stdin() -> std::io::Result<Vec<u8>> {
    use std::io::{self, Write};

    print!("Text to convert: ");
    io::stdout().flush()?;

    let mut s = String::new();
    io::stdin().read_line(&mut s)?;

    Ok(s.into_bytes())
}