// Пример 1: Простой диапазон
// "1-3" → range_parser → Expr::Range

// Пример 2: Диапазон с оператором
// "and 1-3" → range_parser → Expr::Range(Range { op: And, ... })

// Пример 3: Скобки с диапазоном
// "(1-3)" → parens_parser → expr → range_parser → Expr::Range

// Пример 4: AND двух диапазонов
// "(1-3) and (4-6)" 
// parens_parser → Expr::Range
// binary_op_parser(And)
// parens_parser → Expr::Range
// → Expr::Binary { op: And, left: Range(1-3), right: Range(4-6) }

// Пример 5: Цепочка с приоритетом
// "(1-3) and (4-6) or (7-9)"
// precedence расставит приоритеты:
// → Expr::Binary { 
//     op: Or,
//     left: Expr::Binary { op: And, left: Range(1-3), right: Range(4-6) },
//     right: Range(7-9)
//   }

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0},
    combinator::{map, opt, value},
    sequence::delimited,
    Parser,
    error::Error,
};

use crate::conditions::ast::*;
use crate::conditions::error::ParseError;

/// Основная функция для внешнего использования
pub fn parse_ddr_expression(input: &str) -> Result<Expr, ParseError> {
    let input = input.trim();
    
    match expr_parser().parse(input) {
        Ok(("", expr)) => Ok(expr),
        Ok((remaining, _)) => Err(ParseError::ExtraInput(remaining.to_string())),
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
            // Попробуем сделать ошибку понятнее
            let input_preview = if input.len() > 20 {
                format!("{}...", &input[..20])
            } else {
                input.to_string()
            };
            
            match e.code {
                nom::error::ErrorKind::Digit => {
                    Err(ParseError::ExpectedNumber(format!("в '{}'", input_preview)))
                }
                nom::error::ErrorKind::Char => {
                    Err(ParseError::UnexpectedChar(
                        input.chars().next().unwrap_or('?'), 
                        0
                    ))
                }
                nom::error::ErrorKind::Tag => {
                    Err(ParseError::UnknownOperator(input_preview))
                }
                _ => {
                    eprintln!("Ошибка парсинга: {:?}", e);
                    Err(ParseError::InternalError)
                }
            }
        }
        Err(_) => Err(ParseError::InternalError),
    }
}

/// Парсер числа
fn number_parser<'a>() -> impl Parser<&'a str, Output = u32, Error = Error<&'a str>> {
    map(digit1, |s: &str| s.parse().unwrap())
}

/// Обёртка для игнорирования пробелов
fn ws<'a, Par>(parser: Par) -> impl Parser<&'a str, Output = Par::Output, Error = Error<&'a str>>
where
    Par: Parser<&'a str, Error = Error<&'a str>>,
{
    delimited(multispace0, parser, multispace0)
}

/// Парсер оператора внутри диапазона (or/and/|/&) - опционально
fn range_op_parser<'a>() -> impl Parser<&'a str, Output = Option<RangeOp>, Error = Error<&'a str>> {
    opt(ws(alt((
        value(RangeOp::And, alt((tag("and"), tag("&")))),
        value(RangeOp::Or, alt((tag("or"), tag("|")))),
    ))))
}

/// Парсер диапазона: [or/and] число-число
fn range_parser<'a>() -> impl Parser<&'a str, Output = Range, Error = Error<&'a str>> {
    move |input: &'a str| {
        let (input, op) = range_op_parser().parse(input)?;
        let (input, start) = ws(number_parser()).parse(input)?;
        let (input, _) = ws(char('-')).parse(input)?;
        let (input, end) = ws(number_parser()).parse(input)?;
        
        Ok((input, Range::new(start, end, op.unwrap_or(RangeOp::Or))))
    }
}

/// Парсер выражения в скобках
fn parens_parser<'a>() -> impl Parser<&'a str, Output = Expr, Error = Error<&'a str>> {
    move |input: &'a str| {
        let (input, _) = ws(char('(')).parse(input)?;
        let (input, expr) = expr_parser().parse(input)?;
        let (input, _) = ws(char(')')).parse(input)?;
        
        Ok((input, expr))
    }
}

/// Парсер бинарного оператора (and/or/&/|)
fn binary_op_parser<'a>() -> impl Parser<&'a str, Output = BinaryOp, Error = Error<&'a str>> {
    ws(alt((
        value(BinaryOp::And, alt((tag("and"), tag("&")))),
        value(BinaryOp::Or, alt((tag("or"), tag("|")))),
    )))
}

/// Парсер атомарного выражения (скобки или диапазон)
fn atom_parser<'a>() -> impl Parser<&'a str, Output = Expr, Error = Error<&'a str>> {
    alt((
        parens_parser(),
        map(range_parser(), Expr::Range),
    ))
}

/// Парсер выражения (с левой ассоциативностью)
fn expr_parser<'a>() -> impl Parser<&'a str, Output = Expr, Error = Error<&'a str>> {
    move |mut input: &'a str| {
        let (rest, mut left) = atom_parser().parse(input)?;
        input = rest;
        
        loop {
            match binary_op_parser().parse(input) {
                Ok((rest, op)) => {
                    let (rest, right) = atom_parser().parse(rest)?;
                    left = Expr::Binary {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                    input = rest;
                }
                Err(nom::Err::Error(_)) => break,
                Err(e) => return Err(e),
            }
        }
        
        Ok((input, left))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_range() {
        let (_, range) = range_parser().parse("1-3").unwrap();
        assert_eq!(range.start, 1);
        assert_eq!(range.end, 3);
        assert_eq!(range.operator, RangeOp::Or);
        
        let (_, range) = range_parser().parse("or 1-3").unwrap();
        assert_eq!(range.operator, RangeOp::Or);
        
        let (_, range) = range_parser().parse("and 4-6").unwrap();
        assert_eq!(range.operator, RangeOp::And);
    }
    
    #[test]
    fn test_parens() {
        let (_, expr) = parens_parser().parse("(1-3)").unwrap();
        match expr {
            Expr::Range(range) => {
                assert_eq!(range.start, 1);
                assert_eq!(range.end, 3);
            }
            _ => panic!("Expected range"),
        }
    }
    
    #[test]
    fn test_binary() {
        let (_, expr) = expr_parser().parse("(1-3) and (4-6)").unwrap();
        match expr {
            Expr::Binary { op, left, right } => {
                assert_eq!(op, BinaryOp::And);
                match *left {
                    Expr::Range(range) => {
                        assert_eq!(range.start, 1);
                        assert_eq!(range.end, 3);
                    }
                    _ => panic!("Expected range"),
                }
                match *right {
                    Expr::Range(range) => {
                        assert_eq!(range.start, 4);
                        assert_eq!(range.end, 6);
                    }
                    _ => panic!("Expected range"),
                }
            }
            _ => panic!("Expected binary expression"),
        }
    }
    
    #[test]
    fn test_chain() {
        let (_, expr) = expr_parser().parse("(1-3) and (4-6) or (7-9)").unwrap();
        match expr {
            Expr::Binary { op, left, right } => {
                assert_eq!(op, BinaryOp::Or);
                match *left {
                    Expr::Binary { op: and_op, .. } => {
                        assert_eq!(and_op, BinaryOp::And);
                    }
                    _ => panic!("Expected AND as left operand"),
                }
                match *right {
                    Expr::Range(range) => {
                        assert_eq!(range.start, 7);
                        assert_eq!(range.end, 9);
                    }
                    _ => panic!("Expected range as right operand"),
                }
            }
            _ => panic!("Expected OR at top level"),
        }
    }
}