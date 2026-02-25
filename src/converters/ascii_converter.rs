use std::fmt::format;


struct ConvertedString {
    raw_str: String,
    str_as_scn: String,
    str_as_chars: String,
}


// impl ConvertedString {
//     pub fn new() -> Self {

//     }
// }


pub fn gen_scn_from_chars(string: &str) -> String {
    // Функция генерирует строку ASCII с префиксом на основе входящей.
    // Пример: gen_scn_from_chars("CO4554") => ".1.6.67.79.52.53.53.52"


    // Если нужно просто получить вектор чисел
    let ascii_codes: Vec<u8> = string.trim().bytes().collect();
    let len = ascii_codes.len();


    println!("{:?}", ascii_codes);
    
    // Если нужна строка с кодами через пробел
    let codes_str = string
        .bytes()
        .map(|b| b.to_string())
        .collect::<Vec<_>>()
        .join(".");
    println!("codes_str={codes_str}");
    let result = format!(".1.{}.{}", len, codes_str);
    println!("SCN: {result}");
    result
}