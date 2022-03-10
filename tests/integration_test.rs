use arbuscular::adder::add;

#[test]
fn test_add() {
    assert_eq!(add(3, 2), 5);
}

// #[test]
// // you shall not pass!
// fn test_add_fail() {
//     assert_eq!(add(3, 2), 0);
// }
