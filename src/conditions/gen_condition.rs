

pub fn generate_condition(input: &str) -> String {
    // 1. Парсим входную строку
    let parts: Vec<&str> = input.split(',')
                                .map(|s| s.trim())
                                .collect();
    
    match parts.as_slice() {
        [ddr_part] => parse_ddr(ddr_part),
        [ddr_part, mr_part] => {
            let ddr_cond = parse_ddr(ddr_part);
            format!("({}) and mr(G{})", ddr_cond, mr_part)
        }
        _ => panic!("Неверный формат: {}", input),
    }
}

fn parse_ddr(input: &str) -> String {
    // 2. Определяем оператор (| или &)
    let (op, nums): (char, Vec<&str>) = if input.contains('|') {
        ('|', input.split('|').collect())
    } else if input.contains('&') {
        ('&', input.split('&').collect())
    } else {
        panic!("Неверный формат DDR: {}", input);
    };
    
    // 3. Парсим числа
    let start: i32 = nums[0].parse().expect("Ожидалось число");
    let end: i32 = nums[1].parse().expect("Ожидалось число");
    
    // 4. Генерируем условия
    let conditions: Vec<String> = (start..=end)
        .map(|i| format!("ddr(D{})", i))
        .collect();
    
    // 5. Соединяем с правильным оператором
    let join_op = if op == '|' { " or " } else { " and " };
    format!("({})", conditions.join(join_op))
}

