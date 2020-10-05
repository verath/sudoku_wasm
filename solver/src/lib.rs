use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Sudoku {
    // 0  1  2  3  4  5  6  7  8
    // 9  10 11 12 13 14 15 16 17
    // 18 19 20 21 22 23 24 25 26
    // 27 28 29 30 31 32 33 34 35
    // 36 37 38 39 40 41 42 43 44
    // 45 46 47 48 49 50 51 52 53
    // 54 55 56 57 58 59 60 61 62
    // 63 64 65 66 67 68 69 70 71
    // 72 73 74 75 76 77 78 79 80
    cells: Vec<Option<u8>>,
}

impl Sudoku {
    pub fn new(cells: Vec<Option<u8>>) -> Self {
        Sudoku { cells }
    }

    pub fn solve(&self) -> Option<Sudoku> {
        if !self.is_valid() {
            return None;
        }
        self.do_solve()
    }

    fn do_solve(&self) -> Option<Sudoku> {
        if self.is_solved() {
            return Some(self.clone());
        }
        let (position, possible_values) = self.find_empty_cell();
        for v in possible_values {
            let mut next = self.clone();
            next.cells[position] = Some(v);
            if let Some(solved) = next.do_solve() {
                return Some(solved);
            }
        }
        None
    }

    fn is_solved(&self) -> bool {
        self.cells.iter().all(|v| v.is_some())
    }

    fn is_valid(&self) -> bool {
        let block_positions = [
            // Rows
            [0, 1, 2, 3, 4, 5, 6, 7, 8],
            [9, 10, 11, 12, 13, 14, 15, 16, 17],
            [18, 19, 20, 21, 22, 23, 24, 25, 26],
            [27, 28, 29, 30, 31, 32, 33, 34, 35],
            [36, 37, 38, 39, 40, 41, 42, 43, 44],
            [45, 46, 47, 48, 49, 50, 51, 52, 53],
            [54, 55, 56, 57, 58, 59, 60, 61, 62],
            [63, 64, 65, 66, 67, 68, 69, 70, 71],
            [72, 73, 74, 75, 76, 77, 78, 79, 80],
            // Cols
            [0, 9, 18, 27, 36, 45, 54, 63, 72],
            [1, 10, 19, 28, 37, 46, 55, 64, 73],
            [2, 11, 20, 29, 38, 47, 56, 65, 74],
            [3, 12, 21, 30, 39, 48, 57, 66, 75],
            [4, 13, 22, 31, 40, 49, 58, 67, 76],
            [5, 14, 23, 32, 41, 50, 59, 68, 77],
            [6, 15, 24, 33, 42, 51, 60, 69, 78],
            [7, 16, 25, 34, 43, 52, 61, 70, 79],
            [8, 17, 26, 35, 44, 53, 62, 71, 80],
            // Squares
            [0, 1, 2, 9, 10, 11, 18, 19, 20],     // Top Left
            [3, 4, 5, 12, 13, 14, 21, 22, 23],    // Top Mid
            [6, 7, 8, 15, 16, 17, 24, 25, 26],    // Top Right
            [27, 28, 29, 36, 37, 38, 45, 46, 47], // Mid Left
            [30, 31, 32, 39, 40, 41, 48, 49, 50], // Mid Mid
            [33, 34, 35, 42, 43, 44, 51, 52, 53], // Mid Right
            [54, 55, 56, 63, 64, 65, 72, 73, 74], // Bottom Left
            [57, 58, 59, 66, 67, 68, 75, 76, 77], // Bottom Mid
            [60, 61, 62, 69, 70, 71, 78, 79, 80], // Bottom Right
        ];

        for &block in block_positions.iter() {
            let block = block.iter().map(|&idx| &self.cells[idx]);
            let mut values = [false; 9];
            for &v in block {
                if v.is_none() {
                    continue;
                }
                let idx = (v.unwrap() as usize) - 1;
                if values[idx] {
                    return false;
                }
                values[idx] = true;
            }
        }
        true
    }

    fn possible_values(&self, position: usize) -> Vec<u8> {
        let mut used_values = [false; 9];

        // Exclude values already in same row.
        let row_idx = position / 9;
        self.cells[(row_idx * 9)..(row_idx * 9 + 9)]
            .iter()
            .filter_map(|&v| v)
            .for_each(|v| used_values[(v - 1) as usize] = true);

        // Exclude values already in same col.
        let col_idx = position % 9;
        for i in 0..9 {
            let p = col_idx + 9 * i;
            // Note value at given "position" assumed to be None.
            if let Some(v) = self.cells[p] {
                used_values[(v - 1) as usize] = true;
            }
        }
        // Exclude values already in same square.
        // square_pos is position of top-left position of square.
        let square_pos = (row_idx / 3) * 3 * 9 + (col_idx / 3) * 3;
        for r in 0..3 {
            for c in 0..3 {
                let p = square_pos + r * 9 + c;
                if let Some(v) = self.cells[p] {
                    used_values[(v - 1) as usize] = true;
                }
            }
        }

        used_values
            .iter()
            .enumerate()
            .filter(|&(_, is_used)| !is_used)
            .map(|(idx, _)| (idx + 1) as u8)
            .collect()
    }

    fn find_empty_cell(&self) -> (usize, Vec<u8>) {
        let empty_cells_itr = self
            .cells
            .iter()
            .enumerate()
            .filter(|(_, v)| v.is_none())
            .map(|(idx, _)| (idx, self.possible_values(idx)));

        let mut best_position = None;
        let mut best_possible = None;
        let mut best_num_possible = 10;
        for (idx, possible_values) in empty_cells_itr {
            let num_possible = possible_values.len();
            // We will not beat a cell with only one possible value.
            if num_possible == 1 {
                return (idx, possible_values);
            }
            if num_possible < best_num_possible {
                best_position = Some(idx);
                best_possible = Some(possible_values);
                best_num_possible = num_possible;
            }
        }
        (best_position.unwrap(), best_possible.unwrap())
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let cells_iter = self.cells.iter();
        for &cell_value in cells_iter {
            let cell_str = match cell_value {
                Some(v) if v > 0 && v < 10 => (b'0' + v) as char,
                Some(v) => panic!("unexpected cell value '{}'", v),
                None => '_',
            };
            write!(f, "{}", cell_str)?;
        }
        Ok(())
    }
}

impl FromStr for Sudoku {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.len() != (9 * 9) {
            return Err("Must be 9x9");
        }
        let cells: Result<Vec<Option<u8>>, _> = bytes
            .iter()
            .map(|&c| match c {
                b'1' => Ok(Some(1)),
                b'2' => Ok(Some(2)),
                b'3' => Ok(Some(3)),
                b'4' => Ok(Some(4)),
                b'5' => Ok(Some(5)),
                b'6' => Ok(Some(6)),
                b'7' => Ok(Some(7)),
                b'8' => Ok(Some(8)),
                b'9' => Ok(Some(9)),
                b'_' => Ok(None),
                _ => Err("Invalid char"),
            })
            .collect();
        Ok(Sudoku::new(cells?))
    }
}

pub fn solve(input: &str) -> Result<String, &'static str> {
    let sudoku_in = Sudoku::from_str(input)?;
    let sudoku_solved = sudoku_in.solve().ok_or("no solution")?;
    Ok(format!("{}", sudoku_solved))
}

pub const SIMPLE: &str = "\
289____4_\
1____4___\
__52____6\
8_____491\
____7____\
634_____7\
9____21__\
___3____2\
_2____539";
pub const SIMPLE_SOLUTION: &str = "\
289613745\
167954328\
345287916\
872536491\
591478263\
634129857\
953762184\
418395672\
726841539";
pub const HARD: &str = "\
___567__3\
2____3_1_\
_________\
_9_6____8\
__1___2__\
87___2_9_\
1________\
_357_1___\
72984___5";
pub const HARD_SOLUTION: &str = "\
918567423\
257483916\
643129587\
592614378\
361978254\
874352691\
186235749\
435791862\
729846135";
// https://www.sudokuwiki.org/Weekly_Sudoku.asp
// #420, October 3 - October 9: The Weekly Extreme 'Unsolveable' Sudoku Puzzle
pub const EXTREME: &str = "\
________1\
_8__5_76_\
6____4_3_\
_____7__2\
___3_____\
7___8_6__\
__6_7_98_\
85_4___1_\
97_______";
pub const EXTREME_SOLUTION: &str = "\
395768421\
184253769\
627914835\
518647392\
469325178\
732189654\
246571983\
853496217\
971832546";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sudoku_from_str() {
        // Empty string.
        let res = Sudoku::from_str("");
        assert_eq!(res.err(), Some("Must be 9x9"));

        // Valid sudoku.
        let sudoku_valid = "\
        289613745\
        167954328\
        345287916\
        872536491\
        591478263\
        634129857\
        953762184\
        418395672\
        726841539";
        let res = Sudoku::from_str(sudoku_valid);
        assert_eq!(format!("{}", res.unwrap()), sudoku_valid);

        // Invalid but parsable sudoku.
        let sudoku_invalid = "\
        289613745\
        167954328\
        345287916\
        872536491\
        591478263\
        634129857\
        953762184\
        418395672\
        726841538";
        let res = Sudoku::from_str(sudoku_invalid);
        assert_eq!(format!("{}", res.unwrap()), sudoku_invalid);

        // Invalid chars in input.
        let sudoku_invalid = "\
        _________\
        _________\
        _________\
        _________\
        _________\
        _________\
        _________\
        _________\
        ________!";
        let res = Sudoku::from_str(sudoku_invalid);
        assert_eq!(res.err(), Some("Invalid char"))
    }

    #[test]
    fn test_solve_solved() {
        let solved = "\
        289613745\
        167954328\
        345287916\
        872536491\
        591478263\
        634129857\
        953762184\
        418395672\
        726841539";
        assert_eq!(solve(solved).unwrap(), solved);
    }

    #[test]
    fn test_solve_simple() {
        assert_eq!(solve(SIMPLE).unwrap(), SIMPLE_SOLUTION);
    }

    #[test]
    fn test_solve_hard() {
        assert_eq!(solve(HARD).unwrap(), HARD_SOLUTION);
    }

    #[test]
    fn test_solve_extreme() {
        assert_eq!(solve(EXTREME).unwrap(), EXTREME_SOLUTION);
    }
}
