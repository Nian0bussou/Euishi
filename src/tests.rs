#[test]
fn parseInput() {
    let input = "0";
    match input.parse::<usize>() {
        Ok(val) => println!("valid {val} : received : {input}"),
        Err(_) => panic!("err : input : {input}"),
    }
}
