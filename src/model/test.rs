struct Test {
    value: Vec<u8>;
}

fn caller() {
    let t = Test {value: Vec::new()};
    test_pass(&t)
}
fn test_pass(test: &Test) {
    let &n = &test.value;
    
}