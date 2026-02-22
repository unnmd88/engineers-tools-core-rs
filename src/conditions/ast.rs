//! AST (абстрактное синтаксическое дерево) для логических условий

/// Выражение условия
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Просто число: 1, 2, 3...
    Number(u32),

    /// OR операция: A | B | C
    Or(Vec<Expr>),

    /// AND операция: A & B & C
    And(Vec<Expr>),
}

impl Expr {
    /// Создать число
    pub fn number(n: u32) -> Self {
        Expr::Number(n)
    }

    /// Создать OR выражение (уплощает вложенные OR)
    pub fn or(children: Vec<Expr>) -> Self {
        // Собираем все вложенные OR в один вектор
        let mut flat = Vec::new();
        for child in children {
            match child {
                Expr::Or(mut subs) => flat.append(&mut subs),
                other => flat.push(other),
            }
        }

        if flat.len() == 1 {
            flat.into_iter().next().unwrap()
        } else {
            Expr::Or(flat)
        }
    }

    /// Создать AND выражение (уплощает вложенные AND)
    pub fn and(children: Vec<Expr>) -> Self {
        let mut flat = Vec::new();
        for child in children {
            match child {
                Expr::And(mut subs) => flat.append(&mut subs),
                other => flat.push(other),
            }
        }

        if flat.len() == 1 {
            flat.into_iter().next().unwrap()
        } else {
            Expr::And(flat)
        }
    }

    /// Преобразовать в формат ddr(D...)
    pub fn to_ddr_string(&self) -> String {
        match self {
            Expr::Number(n) => format!("ddr(D{})", n),
            Expr::Or(children) => {
                let inner: Vec<String> = children.iter()
                    .map(|c| c.to_ddr_string())
                    .collect();
                format!("({})", inner.join(" or "))
            }
            Expr::And(children) => {
                let inner: Vec<String> = children.iter()
                    .map(|c| c.to_ddr_string())
                    .collect();
                format!("({})", inner.join(" and "))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_number() {
        let expr = Expr::number(5);
        assert_eq!(expr, Expr::Number(5));
        assert_eq!(expr.to_ddr_string(), "ddr(D5)");
    }

    #[test]
    fn test_create_or() {
        let expr = Expr::or(vec![
            Expr::number(1),
            Expr::number(3),
        ]);
        
        match expr {
            Expr::Or(children) => {
                assert_eq!(children.len(), 2);
            }
            _ => panic!("Expected Or expression"),
        }
        
        assert_eq!(expr.to_ddr_string(), "(ddr(D1) or ddr(D3))");
    }

    #[test]
    fn test_create_and() {
        let expr = Expr::and(vec![
            Expr::number(1),
            Expr::number(3),
        ]);
        
        assert_eq!(expr.to_ddr_string(), "(ddr(D1) and ddr(D3))");
    }

    #[test]
    fn test_nested() {
        let expr = Expr::or(vec![
            Expr::number(1),
            Expr::and(vec![
                Expr::number(2),
                Expr::number(3),
            ]),
        ]);
        
        assert_eq!(expr.to_ddr_string(), "(ddr(D1) or (ddr(D2) and ddr(D3)))");
    }
}