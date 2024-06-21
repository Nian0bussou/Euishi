#[test]
fn parseSpaceInput() {
    _ = match " ".parse::<usize>() {
        Ok(val) => println!("valid {}", val),
        Err(_) => panic!("err : space"),
    };
}
#[test]
fn parseInput1() {
    _ = match "1".parse::<usize>() {
        Ok(val) => println!("valid {}", val),
        Err(_) => panic!("err : input : 1"),
    };
}
