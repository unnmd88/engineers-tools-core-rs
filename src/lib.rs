// src/lib.rs
pub mod conditions;  // просто реэкспортируем весь модуль

// Теперь другие части проекта могут писать:
use crate::conditions::parse_ddr_expression;
pub mod converters;
