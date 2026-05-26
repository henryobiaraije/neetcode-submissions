impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    for row in 0..9 {
        let mut seen = HashSet::new();
        for i in 0..9 {
            if board[row][i] == '.' {
                continue;
            }
            if !seen.insert(board[row][i]) {
                return false;
            }
        }
    }

    for col in 0..9 {
        let mut seen = HashSet::new();
        for i in 0..9 {
            if board[i][col] == '.' {
                continue;
            }
            if !seen.insert(board[i][col]) {
                return false;
            }
        }
    }

    for square in 0..9 {
        let mut seen = HashSet::new();
        println!("square={}", square);
        for a in 0..3 {
            for b in 0..3 {
                let row = (square / 3) * 3 + a;
                let col = (square % 3) * 3 + b;

                let is_dot = board[row][col] == '.';

                // println!(
                //     "square={}, a={}, b={}, row={}, col={}, seen={:?} -- {}",
                //     square,
                //     a,
                //     b,
                //     row,
                //     col,
                //     seen,
                //     if is_dot {
                //         "EMPTY".to_string()
                //     } else {
                //         "".to_string()
                //     }
                // );
                // // println!(
                //     "      - row = (square / 3) * 3 + a = ({} / 3) * 3 + a = ({}) * 3 + a = {} + a = {}",
                //     square,
                //     square / 3,
                //     (square / 3) * 3,
                //     (square / 3) * 3 + a
                // );
                // println!(
                //     "      - col = (square % 3) * 3 + b = ({} % 3) * 3 + b = ({}) * 3 + b = {} + b = {}",
                //     square,
                //     square % 3,
                //     (square % 3) * 3,
                //     (square % 3) * 3 + b
                // );

                if is_dot {
                    continue;
                }
                if !seen.insert(board[row][col]) {
                    // println!("found duplicate: row={}, col={}", row, col);
                    return false;
                }
            }
        }
    }

    true
}
}
