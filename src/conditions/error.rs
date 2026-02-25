//! Типы ошибок для парсера DDR-выражений

use thiserror::Error;

/// Ошибки, которые могут возникнуть при парсинге
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ParseError {
    /// Ошибка: неожиданный символ
    #[error("Неожиданный символ '{0}' на позиции {1}")]
    UnexpectedChar(char, usize),
    
    /// Ошибка: ожидалось число
    #[error("Ожидалось число, получено '{0}'")]
    ExpectedNumber(String),
    
    /// Ошибка: неизвестный оператор
    #[error("Неизвестный оператор '{0}'. Используйте and/or или &/|")]
    UnknownOperator(String),
    
    /// Ошибка: незакрытая скобка
    #[error("Незакрытая скобка")]
    UnclosedParen,
    
    /// Ошибка: лишние символы после выражения
    #[error("Лишние символы после выражения: '{0}'")]
    ExtraInput(String),
    
    /// Ошибка: не хватает операнда
    #[error("После оператора '{0}' должно быть выражение")]
    MissingOperand(String),
    
    /// Ошибка: внутренняя ошибка парсера
    #[error("Внутренняя ошибка парсера")]
    InternalError,
}

impl ParseError {
    /// Создать ошибку из ошибки nom
    pub fn from_nom<E>(_err: E, _input: &str) -> Self {
        // TODO: нормальное преобразование ошибок nom
        ParseError::InternalError
    }
}