use thiserror::Error;
pub type Result<T> = std::result::Result<T, CliError>;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum CliError {
    #[error("Comando não encontrado: {command}")]
    CommandNotFound { command: String },

    #[error("Subcomando não encontrado: {subcommand}")]
    SubcommandNotFound { subcommand: String },

    #[error("Flag obrigatória não fornecida: --{flag}")]
    RequiredFlagNotProvided { flag: String },

    #[error("Flag desconhecida: --{flag}")]
    UnknowFlag { flag: String },

    #[error("Valor inválido para flag: --{flag}: {value}. Esperado: {expected}")]
    InvalidFlagValue { flag: String, value: String, expected: String },

    #[error("Flag --{flag} requer um valor")]
    FlagValueMissing { flag: String },

    #[error("Muitos argumentos posicionais fornecidos")]
    TooManyArguments { flag: String },

    #[error("Argumentos posicionais insuficientes. Esperado: {expected}, recebido: {received}")]
    NotEnoughArguments { expected: usize, received: usize },

    #[error("Erro de I/O: {0}")]
    IoError(String),

    #[error("Erro de parsing: {message}")]
    ParseError { message: String },

    #[error("Aplicação nâo configurada corretamente: {message}")]
    ConfigurationError {message: String}
}

impl From<std::io::Error> for CliError {
    fn from(error: std::io::Error) -> Self {
        CliError::IoError(error.to_string())
    }
}