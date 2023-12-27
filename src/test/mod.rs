pub struct Test123Plugin;
use crate::engine::Board;

#[cfg(test)]

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this
// example.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::*;

#[test]
fn test_init_board() {
    println!("wtf");
    let board = Board::new(6, 6);
    assert_eq!(board.get_board()[[0, 0]], 0)
}

#[test]
fn test_bad_add() {
    // This assert would fire and test will fail.
    // Please note, that private functions can be tested too!
    assert_eq!(bad_add(1, 2), 3);
}
