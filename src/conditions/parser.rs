use crate::conditions::ast::Expr;

pub struct Parser {
    input: String,
    pos: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            pos: 0,
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.peek_char()?;
        self.pos += 1;
        Some(c)
    }

    /// Основная функция парсинга
    pub fn parse(&mut self) -> Result<Expr, String> {
        let expr = self.parse_or(0)?;
        
        // Проверяем, что дошли до конца строки
        if self.peek_char().is_some() {
            return Err(format!("Лишние символы после выражения: {:?}", self.peek_char()));
        }
        
        Ok(expr)
    }

    /// Парсинг с визуализацией (возвращает AST и печатает процесс)
    pub fn parse_with_visual(&mut self) -> Result<Expr, String> {
        println!("\n=== РАЗБИРАЕМ: '{}' ===", self.input);
        let expr = self.parse_or(0)?;
        println!("=== ГОТОВО, AST: {:?} ===\n", expr);
        Ok(expr)
    }

    fn parse_or(&mut self, depth: usize) -> Result<Expr, String> {
        let indent = "  ".repeat(depth);
        println!("{}→ parse_or (позиция={}, символ='{:?}')", 
            indent, self.pos, self.peek_char());

        // Разбираем первый and
        let mut children = vec![self.parse_and(depth + 1)?];

        // Пока видим '|', добавляем правые части
        while let Some('|') = self.peek_char() {
            println!("{}  ВСТРЕТИЛ '|' на позиции {}", indent, self.pos);
            self.next_char(); // съедаем '|'
            children.push(self.parse_and(depth + 1)?);
        }

        let expr = if children.len() == 1 {
            children.into_iter().next().unwrap()
        } else {
            Expr::or(children)
        };
        
        println!("{}← parse_or возвращает {:?}", indent, expr);
        Ok(expr)
    }

    fn parse_and(&mut self, depth: usize) -> Result<Expr, String> {
        let indent = "  ".repeat(depth);
        println!("{}⇒ parse_and (позиция={}, символ='{:?}')", 
            indent, self.pos, self.peek_char());

        // Разбираем первый атом
        let mut children = vec![self.parse_atom(depth + 1)?];

        // Пока видим '&', добавляем правые части
        while let Some('&') = self.peek_char() {
            println!("{}  ВСТРЕТИЛ '&' на позиции {}", indent, self.pos);
            self.next_char(); // съедаем '&'
            children.push(self.parse_atom(depth + 1)?);
        }

        let expr = if children.len() == 1 {
            children.into_iter().next().unwrap()
        } else {
            Expr::and(children)
        };
        
        println!("{}⇐ parse_and возвращает {:?}", indent, expr);
        Ok(expr)
    }

    fn parse_atom(&mut self, depth: usize) -> Result<Expr, String> {
        let indent = "  ".repeat(depth);
        println!("{}↦ parse_atom (позиция={}, символ='{:?}')", 
            indent, self.pos, self.peek_char());

        // Пропускаем пробелы
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }

        let expr = match self.peek_char() {
            Some('(') => {
                println!("{}  ОТКРЫЛ скобку", indent);
                self.next_char(); // съедаем '('
                
                let inner = self.parse_or(depth + 1)?;
                
                // Проверяем закрывающую скобку
                match self.peek_char() {
                    Some(')') => {
                        println!("{}  ЗАКРЫЛ скобку ')'", indent);
                        self.next_char();
                        inner
                    }
                    Some(c) => {
                        return Err(format!("Ожидалась ')', но найдено '{}' на позиции {}", c, self.pos));
                    }
                    None => {
                        return Err("Ожидалась ')', но строка закончилась".to_string());
                    }
                }
            }
            
            Some(c) if c.is_ascii_digit() => {
                self.next_char();
                let num = c.to_digit(10).unwrap();
                println!("{}  ПРОЧИТАЛ ЧИСЛО: {}", indent, num);
                Expr::number(num)
            }
            
            Some(c) => {
                return Err(format!("Неожиданный символ '{}' на позиции {}", c, self.pos));
            }
            
            None => {
                return Err("Неожиданный конец строки".to_string());
            }
        };

        println!("{}↤ parse_atom возвращает {:?}", indent, expr);
        Ok(expr)
    }
}