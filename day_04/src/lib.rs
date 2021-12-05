use std::fmt;
use std::collections::HashMap;

// ================================================== STRUCTS ==================================================

type PositionUnit = usize;
type Number = i64;
type Score = Number;

struct BingoSpot {
    number:     Number,
    marked:     bool,
}

struct BingoCard  {
    size:           (usize, usize),
    slots:          HashMap<(PositionUnit, PositionUnit), BingoSpot>,
    last_marked:    Option<Number>,
    won:            bool,
}

pub struct Game {
    calls:          Vec<Number>,
    called_index:   usize,
    boards:         Vec<BingoCard>,
}

// ================================================== AUX FUNCTIONS ==================================================

// ================================================== IMPLEMENTATIONS ==================================================

impl BingoCard {

    fn new(slot_numbers: Vec<Vec<Number>>) -> BingoCard {

        let (mut max_rows, mut max_columns) = (0, 0); 
        let mut slots : HashMap<(PositionUnit, PositionUnit), BingoSpot> = HashMap::new();

        for (row_index, row) in slot_numbers.into_iter().enumerate() {

            if row_index + 1 > max_rows { max_rows = row_index + 1 }
            for (column_index, number) in row.into_iter().enumerate() {

                if column_index + 1 >max_columns {max_columns = column_index + 1 }
                slots.insert((row_index, column_index), BingoSpot{number: number, marked: false});
            }
        }

        BingoCard {
            size: (max_rows, max_columns),
            slots: slots,
            last_marked: None,
            won: false,
        }
    }

    fn mark_number(&mut self, number: Number) {

        for (_, spot) in self.slots.iter_mut() {
            if spot.number == number {
                spot.marked = true;
                self.last_marked = Some(number);
            }
        }
    }

    fn verify_won(&mut self) {

        for index in 0..self.size.0 {
            let mut won_row : bool = true;
            for sub_index in 0..self.size.1 {
                if !self.slots.get(&(index, sub_index)).unwrap().marked { won_row = false; }
            }
    
            self.won = self.won || won_row;
        }

        for index in 0..self.size.1 {
            let mut won_col : bool = true;
            for sub_index in 0..self.size.0 {
                if !self.slots.get(&(sub_index, index)).unwrap().marked { won_col = false; }
            }
    
            self.won = self.won || won_col;
        }
    }

    fn compute_score(&self) -> Option<Score> {

        if !self.won { return None; }
        if self.last_marked.is_none() { return None; }

        let mut sum_score : Score = 0;
        for (_, spot) in self.slots.iter() {
            if !spot.marked { sum_score = sum_score + spot.number; }
        }

        return Some(sum_score * self.last_marked.unwrap());
    }
}

impl fmt::Display for BingoCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut line : String = String::new();
        for row_index in 0..self.size.0 {

            for col_index in 0..self.size.1 {

                let slot = self.slots.get(&(row_index, col_index)).unwrap();
                if slot.marked { line = format!("{}\t{}:X", line, slot.number) }
                else { line = format!("{}\t{}: ", line, slot.number) }
            }

            line = format!("{}\n", line);

        }

        return write!(f, "{}", line);
    }
}

impl Game {

    pub fn new(called_numbers: Vec<Number>, cards_numbers: Vec<Vec<Vec<Number>>>) -> Game {

        let mut cards : Vec<BingoCard> = Vec::new();
        for card_numbers in cards_numbers.into_iter() {
            cards.push(BingoCard::new(card_numbers))
        }

        Game {
            calls: called_numbers,
            called_index: 0,
            boards: cards,
        }
    }

    pub fn do_iterations_until_one_won(&mut self, debug: bool) -> Option<Score> {

        let mut board_won_index : Option<usize> = None;
        for call_index in self.called_index..self.calls.len() {

            let call = self.calls[call_index];
            self.called_index = call_index + 1;

            if debug {
                println!("================================ CALLING {} ================================", call);
                println!();
            }

            for (board_index, board) in self.boards.iter_mut().enumerate() {

                board.mark_number(call);

                if debug {
                    println!("{}", board);
                    println!();
                }

                board.verify_won();
                if board.won { board_won_index = Some(board_index) }
            }

            if board_won_index.is_some() { return self.boards[board_won_index.unwrap()].compute_score() }
        }

        return None;
    }

    pub fn do_iterations_until_all_but_one_won(&mut self, debug: bool) -> Option<Score> {

        let mut last_to_win : Option<usize> = None;
        for call_index in self.called_index..self.calls.len() {

            let call = self.calls[call_index];
            self.called_index = call_index + 1;

            if debug {
                println!("================================ CALLING {} ================================", call);
                println!();
            }

            let mut board_not_won_indexes : Vec<usize> = Vec::new();

            for (board_index, board) in self.boards.iter_mut().enumerate() {

                board.mark_number(call);

                if debug {
                    println!("{}", board);
                    println!();
                }

                board.verify_won();
                if !board.won && last_to_win.is_none() { board_not_won_indexes.push(board_index); }
                if board.won && last_to_win.is_some() && board_index == last_to_win.unwrap() { return board.compute_score(); }
            }

            if board_not_won_indexes.len() == 1 { last_to_win = Some(board_not_won_indexes[0]); }
        }

        return None;
    }
}