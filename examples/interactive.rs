//! Интерактивный тестер для DDR-выражений
//!
//! Запусти и вводи выражения, смотри результат

use std::io::{self, Write};
use traffic_core::conditions::{parse_ddr_expression, to_ddr_string, ParseError};

fn print_error_with_hint(e: ParseError) {
    println!("   ❌ Ошибка: {}", e);
    
    // Добавляем подсказки в зависимости от типа ошибки
    match e {
        ParseError::MissingOperand(op) => {
            println!("     Подсказка: после '{}' нужно выражение, например: {} (1-3)", 
                op, op);
        }
        ParseError::UnclosedParen => {
            println!("     Подсказка: добавь закрывающую скобку ')'");
        }
        ParseError::ExtraClosingParen => {
            println!("     Подсказка: лишняя закрывающая скобка");
        }
        ParseError::EmptyParens => {
            println!("     Подсказка: в скобках должно быть выражение, например: (1-3)");
        }
        ParseError::InvalidRange => {
            println!("     Подсказка: диапазон должен быть в формате 'число-число' (например: 1-3)");
        }
        ParseError::RangeStartGreaterThanEnd(start, end) => {
            println!("     Подсказка: начало диапазона ({}) должно быть меньше или равно концу ({})", 
                start, end);
        }
        ParseError::ExpectedNumber(ctx) => {
            println!("     Подсказка: ожидалось число в '{}'", ctx);
        }
        ParseError::UnknownOperator(op) => {
            println!("     Подсказка: используй and/or или &/|, например: (1-3) and (4-6)");
        }
        ParseError::ExtraInput(rest) => {
            println!("     Подсказка: лишние символы в конце: '{}'", rest);
        }
        ParseError::UnexpectedChar(ch, pos) => {
            println!("     Подсказка: неожиданный символ '{}' на позиции {}", ch, pos);
        }
        ParseError::InternalError(msg) => {
            println!("     Подсказка: внутренняя ошибка, сообщи разработчику: {}", msg);
        }
    }
}

fn main() -> io::Result<()> {
    println!("🔹 Интерактивный тестер DDR-выражений");
    println!("🔹 Вводи выражение (или 'exit' для выхода)\n");
    println!("Примеры:");
    println!("  1-3");
    println!("  or 1-3");
    println!("  |1-3");
    println!("  (or 1-3) and (or 4-6)");
    println!();
    
    loop {
        print!("> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input == "exit" || input == "quit" {
            break;
        }
        
        if input.is_empty() {
            continue;
        }
        
        match parse_ddr_expression(input) {
            Ok(expr) => {
                println!("   ✅ {}", to_ddr_string(&expr));
            }
            Err(e) => {
                print_error_with_hint(e);
            }
        }
        println!();
    }
    
    Ok(())
}