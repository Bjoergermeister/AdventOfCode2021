pub struct Board {
    field: [i32; 25],
}

impl Board {
    pub fn new(lines: &Vec<String>) -> Self {
        let mut board = Board { field: [0; 25] };

        for i in 0..lines.len() {
            parse_line(&mut board, &lines[i], i);
        }

        return board;
    }

    pub fn remove_drawn_number(&mut self, number: i32) {
        for i in 0..25 {
            if self.field[i] == number {
                self.field[i] = 0;
            }
        }
    }

    fn is_winner_horizontal(&self) -> bool {
        'outer: for row in 0..5 {
            for index_in_row in 0..5 {
                if self.field[row * 5 + index_in_row] != 0 { //If any index in the row is not zero, the row is not a winner
                    continue 'outer;
                }
            }
            return true;
        } 
        return false;
    }

    pub fn is_winner_vertical(&self) -> bool {
        'outer: for column in 0..5 {
            for index_in_column in 0..5 {
                if self.field[column + index_in_column * 5] != 0 { //If any index in the column is not zero, the column is not a winner
                    continue 'outer;
                }            
            }
            return true;
        }
        return false;
    }

    pub fn is_winner(&self) -> bool {
        return self.is_winner_horizontal() || self.is_winner_vertical();
    }

    pub fn get_score(&self) -> i32 {
        let mut score = 0;
        for i in 0..25 {
            if self.field[i] != 0 {
                score += self.field[i];
            }
        }
        return score;
    }

}

fn parse_line(board: &mut Board, line: &String, index: usize) {
    let mut i = 0;
    for splitted_line in line.split_whitespace() {        
        let number = splitted_line.parse::<i32>().unwrap();
        board.field[index * 5 + i] = number;
        i += 1;
    }
}