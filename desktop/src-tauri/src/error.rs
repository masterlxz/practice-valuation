use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("yield desejado precisa ser maior que zero")]
    InvalidGuard,
    #[error("erro de banco de dados: {0}")]
    Database(#[from] sea_orm::DbErr),
}

impl AppError {
    fn code(&self) -> &'static str {
        match self {
            AppError::InvalidGuard => "INVALID_GUARD",
            AppError::Database(_) => "DATABASE_ERROR",
        }
    }
}

// Tauri serializes command errors to the frontend as JSON, so this shapes
// them as `{ code, message }` instead of a bare string the UI can't switch on.
impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("AppError", 2)?;
        state.serialize_field("code", self.code())?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}
