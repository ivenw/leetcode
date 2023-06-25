struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;

        const BOARD_SIZE: usize = 9;
        const GRID_SIZE: usize = 3;

        let mut rows = vec![HashSet::with_capacity(BOARD_SIZE); 9];
        let mut cols = vec![HashSet::with_capacity(BOARD_SIZE); 9];
        let mut squares = vec![HashSet::with_capacity(BOARD_SIZE); 9];

        for row_idx in 0..BOARD_SIZE {
            for col_idx in 0..BOARD_SIZE {
                let field = board.get(row_idx).unwrap().get(col_idx).unwrap();
                let square_idx = (row_idx / GRID_SIZE) * GRID_SIZE + (col_idx / GRID_SIZE);

                if *field == '.' {
                    continue;
                }
                if rows[row_idx].contains(field)
                    | cols[col_idx].contains(field)
                    | squares[square_idx].contains(field)
                {
                    return false;
                }

                rows[row_idx].insert(field);
                cols[col_idx].insert(field);
                squares[square_idx].insert(field);
            }
        }
        true
    }
}

fn main() {
    let input_board_1 = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    assert!(Solution::is_valid_sudoku(input_board_1));

    let input_board_2 = vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    assert!(!Solution::is_valid_sudoku(input_board_2));
}
