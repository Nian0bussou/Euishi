#[test]
fn parse_input() {
    let input = "1";
    match input.parse::<usize>() {
        Ok(val) => println!("valid {val} : received : {input}"),
        Err(_) => panic!("err : input : {input}"),
    }
}
