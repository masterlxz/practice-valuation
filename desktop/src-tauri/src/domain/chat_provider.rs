use crate::error::AppError;

// The 3 AI chat providers the app is designed around (see Fase 7 in
// PROJECT_STATE.md). All 3 have real HTTP clients as of Fase 7.6/7.7.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider {
    Gemini,
    Claude,
    OpenAi,
}

impl Provider {
    pub fn as_str(&self) -> &'static str {
        match self {
            Provider::Gemini => "gemini",
            Provider::Claude => "claude",
            Provider::OpenAi => "openai",
        }
    }

    pub fn parse(raw: &str) -> Result<Self, AppError> {
        match raw {
            "gemini" => Ok(Provider::Gemini),
            "claude" => Ok(Provider::Claude),
            "openai" => Ok(Provider::OpenAi),
            other => Err(AppError::UnknownProvider(other.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_accepts_all_known_providers() {
        assert_eq!(Provider::parse("gemini").unwrap(), Provider::Gemini);
        assert_eq!(Provider::parse("claude").unwrap(), Provider::Claude);
        assert_eq!(Provider::parse("openai").unwrap(), Provider::OpenAi);
    }

    #[test]
    fn parse_rejects_unknown_provider() {
        let err = Provider::parse("mistral").unwrap_err();
        assert!(matches!(err, AppError::UnknownProvider(id) if id == "mistral"));
    }

    #[test]
    fn as_str_round_trips_through_parse() {
        for provider in [Provider::Gemini, Provider::Claude, Provider::OpenAi] {
            assert_eq!(Provider::parse(provider.as_str()).unwrap(), provider);
        }
    }
}
