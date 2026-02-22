use traffic_core::conditions::Parser;
use traffic_core::conditions::gen_condition::generate_condition;
use traffic_core::converters::gen_scn_from_chars;

fn view_flow_and_ast() {

        let examples = vec![
        "1",
        "1|3",
        "1&3",
        "1|3&5",
        "(1|3)",
        "(1|3)&5",
        "1|(3&5)",
    ];

    for example in examples {
        let mut parser = Parser::new(example);
        match parser.parse_with_visual() {
            Ok(expr) => {
                println!("УСПЕХ! Получили AST: {:?}", expr);
                println!("В виде ddr: {}", expr.to_ddr_string());
                println!();
            }
            Err(e) => {
                println!("ОШИБКА: {}\n", e);
            }
        }
    }

}


fn simple_gen_condotion() {

    let test_cases = vec!["1|3", "1|3, 24", "1&3, 14"];
    for input in test_cases {
        println!("{} -> {}", input, generate_condition(input));
    }
}


fn covert_scn() {
    let test_cases = vec!["   CO455 4", "         CJ4", "   ", "CC C 4 23      "];
    for test_case in test_cases {
        println!("Это сконверченный ascii: {}", gen_scn_from_chars(test_case));
    }


}


fn main() {

    // view_flow_and_ast();
    covert_scn();

}