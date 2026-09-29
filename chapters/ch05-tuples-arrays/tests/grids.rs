use ch05_tuples_arrays::*;

const M: [[i32; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
const IDENTITY: [[i64; 3]; 3] = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];

#[test]
fn transpose_normal() {
    assert_eq!(transpose(M), [[1, 4, 7], [2, 5, 8], [3, 6, 9]]);
}

#[test]
fn transpose_symmetric_is_unchanged() {
    let s = [[1, 2, 3], [2, 5, 6], [3, 6, 9]];
    assert_eq!(transpose(s), s);
}

#[test]
fn transpose_twice_is_the_original() {
    let m = [[0, -1, 2], [30, 4, 5], [-6, 70, 8]];
    assert_eq!(transpose(transpose(m)), m);
}

#[test]
fn transpose_leaves_its_argument_alone() {
    let m = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let t = transpose(m);
    // `m` is still here, unchanged: `transpose` got its own copy of all nine numbers.
    assert_eq!(m, M);
    assert_ne!(t, m);
}

#[test]
fn mat_mul_identity() {
    let a = [[2, -1, 0], [4, 3, 7], [-5, 6, 1]];
    assert_eq!(mat_mul(a, IDENTITY), a);
    assert_eq!(mat_mul(IDENTITY, a), a);
}

#[test]
fn mat_mul_normal() {
    let a = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let b = [[9, 8, 7], [6, 5, 4], [3, 2, 1]];
    assert_eq!(mat_mul(a, b), [[30, 24, 18], [84, 69, 54], [138, 114, 90]]);
}

#[test]
fn mat_mul_is_not_commutative() {
    let a = [[1, 1, 0], [0, 1, 0], [0, 0, 1]];
    let b = [[1, 0, 0], [1, 1, 0], [0, 0, 1]];
    assert_eq!(mat_mul(a, b), [[2, 1, 0], [1, 1, 0], [0, 0, 1]]);
    assert_eq!(mat_mul(b, a), [[1, 1, 0], [1, 2, 0], [0, 0, 1]]);
}

#[test]
fn mat_mul_zero() {
    let zero = [[0; 3]; 3];
    assert_eq!(mat_mul([[5; 3]; 3], zero), zero);
}

#[test]
fn tic_tac_toe_rows() {
    let top = [['O', 'O', 'O'], ['X', 'X', '.'], ['X', '.', '.']];
    let bottom = [['O', 'O', '.'], ['.', '.', '.'], ['X', 'X', 'X']];
    assert_eq!(tic_tac_toe_winner(top), 'O');
    assert_eq!(tic_tac_toe_winner(bottom), 'X');
}

#[test]
fn tic_tac_toe_columns() {
    let left = [['X', 'O', '.'], ['X', 'O', '.'], ['X', '.', '.']];
    let right = [['X', 'X', 'O'], ['.', '.', 'O'], ['X', '.', 'O']];
    assert_eq!(tic_tac_toe_winner(left), 'X');
    assert_eq!(tic_tac_toe_winner(right), 'O');
}

#[test]
fn tic_tac_toe_diagonals() {
    let main = [['X', 'O', '.'], ['O', 'X', '.'], ['O', '.', 'X']];
    let anti = [['X', 'X', 'O'], ['.', 'O', 'X'], ['O', '.', '.']];
    assert_eq!(tic_tac_toe_winner(main), 'X');
    assert_eq!(tic_tac_toe_winner(anti), 'O');
}

#[test]
fn tic_tac_toe_no_winner() {
    let draw = [['X', 'O', 'X'], ['X', 'O', 'O'], ['O', 'X', 'X']];
    assert_eq!(tic_tac_toe_winner(draw), '.');
}

#[test]
fn tic_tac_toe_empty_squares_dont_win() {
    let empty = [['.'; 3]; 3];
    assert_eq!(tic_tac_toe_winner(empty), '.');
    // Only the top row counts here: the other lines are mixed or contain empty squares.
    let x_top = [['X', 'X', 'X'], ['O', '.', 'O'], ['O', '.', '.']];
    assert_eq!(tic_tac_toe_winner(x_top), 'X');
}
