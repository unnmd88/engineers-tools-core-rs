//! Интерактивный тестер для DDR-выражений
//!
//! Запусти и вводи выражения, смотри результат

use std::io::{self, Write};
use traffic_core::conditions::{parse_ddr_expression, to_ddr_string};

fn main() -> io::Result<()> {
    println!("🔹 Интерактивный тестер DDR-выражений");
    println!("🔹 Вводи выражение (или 'exit' для выхода)\n");
    
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
                println!("   ❌ Ошибка: {}", e);
            }
        }
    }
    
    Ok(())
}