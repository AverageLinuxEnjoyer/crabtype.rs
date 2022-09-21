use std::fmt;

pub enum Language {
    English1K,
    English10K,
    English30K,
    Rust,
}

impl Default for Language {
    fn default() -> Self {
        Language::English1K
    }
}

impl fmt::Display for Language {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use Language::*;
        let as_str = match self {
            English1K => "english_1k",
            English10K => "english_10k",
            English30K => "english_30k",
            Rust => "rust",
        };

        fmt.write_str(as_str)?;

        Ok(())
    }
}

impl Language {
    pub fn from(st: &str) -> Language {
        use Language::*;
        match st {
            "english_1k" => English1K,
            "english_10k" => English10K,
            "english_30k" => English30K,
            _ => Rust,
        }
    }
}
