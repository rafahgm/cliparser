use thiserror::Error;
pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum AppError {
    #[error("Comando não encontrado: {command}")]
    CommandNotFound { command: String },

    #[error("Subcomando não encontrado: {subcommand}")]
    SubcommandNotFound { subcommand: String },

    #[error("Flag obrigatória não fornecida: --{flag}")]
    RequiredFlagNotProvided { flag: String },

    #[error("Flag desconhecida: --{flag}")]
    UnknownFlag { flag: String },

    #[error("Valor inválido para flag: --{flag}: {value}. Esperado: {expected}")]
    InvalidFlagValue { flag: String, value: String, expected: String },

    #[error("Flag --{flag} requer um valor")]
    FlagValueMissing { flag: String },

    #[error("Muitos argumentos posicionais fornecidos")]
    TooManyArguments,

    #[error("Argumentos posicionais insuficientes. Esperado: {expected}, recebido: {received}")]
    NotEnoughArguments { expected: usize, received: usize },

    #[error("Erro de I/O: {0}")]
    IoError(String),

    #[error("Erro de parsing: {message}")]
    ParseError { message: String },

    #[error("Aplicação nâo configurada corretamente: {message}")]
    ConfigurationError {message: String}
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::IoError(error.to_string())
    }
}