struct Sudoku {}

impl Sudoku {
    fn valid(board: &Vec<Vec<char>>, row: usize, col: usize, val: char) -> bool {
        // Check row collisions
        if board[row].iter().any(|&i| i == val) {
            return false;
        }

        // Check column collisions
        for r in 0..9 {
            if board[r][col] == val {
                return false;
            }
        }

        // Check Square collisions
        for r in row / 3 * 3..row / 3 * 3 + 3 {
            for c in col / 3 * 3..col / 3 * 3 + 3 {
                if board[r][c] == val {
                    return false;
                }
            }
        }

        // All check passed
        return true;
    }

    pub fn solve(board: &mut Vec<Vec<char>>) {
        for row in 0..9 {
            for col in 0..9 {
                if board[row][col] == '.' {
                    for val in ['1', '2', '3', '4', '5', '6', '7', '8', '9'] {
                        if Sudoku::valid(board, row, col, val) {
                            // Optimistically assign the possible value
                            board[row][col] = val;
                            // Recursively solve the next digit
                            Sudoku::solve(board);
                            // If we reach this point, backtrace the solution
                            board[row][col] = '.';
                        }
                    }
                    // No valid possibility found: Backtrace
                    return;
                }
            }
        }
        println!("Solution:");
        Sudoku::print(board);
    }

    pub fn print(board: & Vec<Vec<char>>) {
        for row in board.iter() {
            println!("{:?}", row);
        }
    }
}

fn main() {
    // Example "Hard" Sudoku solved in 0.33s
    let mut board: Vec<Vec<char>> = vec![
        vec!['9', '.', '6', '5', '7', '.', '4', '2', '.'],
        vec!['5', '.', '2', '1', '4', '.', '8', '.', '.'],
        vec!['.', '.', '4', '.', '.', '.', '.', '.', '5'],
        vec!['.', '.', '5', '.', '.', '.', '6', '.', '3'],
        vec!['8', '.', '3', '.', '.', '.', '2', '.', '4'],
        vec!['4', '.', '7', '.', '.', '.', '.', '.', '.'],
        vec!['2', '.', '.', '.', '.', '.', '5', '.', '.'],
        vec!['.', '.', '8', '.', '1', '6', '.', '.', '2'],
        vec!['.', '4', '.', '.', '2', '5', '.', '.', '9'],

    ];
    println!("Initial:");
    Sudoku::print(& board);


    Sudoku::solve(&mut board);
}
