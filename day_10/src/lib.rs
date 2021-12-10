use std::collections::HashMap;

// ================================================== STRUCTS ==================================================

struct LineError {
    found:      char,
    expected:   Option<char>
}

struct Line {
    characthers:        Vec<char>,
    invalid_characther: Option<LineError>,
    completion_chars:   Vec<char>,
}

type Score = u64;

pub struct NavigationSubsystem {
    lines:              Vec<Line>
}

// ================================================== AUX FUNCTIONS ==================================================

// ================================================== IMPLEMENTATIONS ==================================================

impl Line {

    fn new(characthers: Vec<char>) -> Line {
        Line {
            characthers: characthers,
            invalid_characther: None,
            completion_chars: Vec::new(),
        }
    }

    fn compute_validity(&mut self) {

        let known_pairs : HashMap<char, char> = vec![
            ('(', ')'),
            ('{', '}'),
            ('[', ']'),
            ('<', '>'),
        ].into_iter().collect();

        let mut stack_to_close : Vec<char> = Vec::new();
        for &characther in self.characthers.iter() {

            let corresponding_closing_option = known_pairs.get(&characther);
            match corresponding_closing_option {

                None => {
                    let last_stack_element = stack_to_close.pop();
                    if last_stack_element.is_none() {

                        self.invalid_characther = Some(LineError{found: characther, expected: None});
                        return;
                    }
                    
                    let last_stack_characther = last_stack_element.unwrap();
                    if last_stack_characther != characther {

                        self.invalid_characther = Some(LineError{found: characther, expected: Some(last_stack_characther)});
                        return;
                    }
                },

                Some(&corresponding_closing) => {
                    stack_to_close.push(corresponding_closing);
                }
            }
        }

        stack_to_close.reverse();
        self.completion_chars = stack_to_close;
    }

    fn compute_error_score(&self) -> Score {

        let score_map : HashMap<char, Score> = vec![
            (')', 3),
            (']', 57),
            ('}', 1197),
            ('>', 25137),
        ].into_iter().collect();

        if self.invalid_characther.is_none() { return 0; }
        else {

            let invalid_info = self.invalid_characther.as_ref().unwrap();
            let found_characther = invalid_info.found;

            let score : Score = score_map.get(&found_characther).unwrap().clone();
            return score;
        }
    }

    fn compute_completion_score(&self) -> Option<Score> {

        let score_map : HashMap<char, Score> = vec![
            (')', 1),
            (']', 2),
            ('}', 3),
            ('>', 4),
        ].into_iter().collect();

        if self.invalid_characther.is_some() { return None }

        let mut score : Score = 0;
        for characther in self.completion_chars.iter() {
            
            let characther_score : Score = *score_map.get(characther).unwrap();
            score = score * 5 + characther_score;
        }

        return Some(score);
    }
}

impl NavigationSubsystem {

    pub fn new(lines_chars: Vec<Vec<char>>) -> NavigationSubsystem {

        let lines : Vec<Line> = lines_chars.into_iter()
            .map(|chars| Line::new(chars)).collect();

        NavigationSubsystem {
            lines: lines,
        }
    }

    pub fn evaluate_lines(&mut self) {
        for line in self.lines.iter_mut() {
            line.compute_validity();
        }
    }

    pub fn compute_syntax_error_score(&self) -> Score {

        let mut sum : Score = 0;
        for line in self.lines.iter() {
            sum = sum + line.compute_error_score();
        }

        return sum;
    }

    pub fn compute_middle_completion_score(&self) -> Score {

        let mut scores : Vec<Score> = Vec::new();
        for line in self.lines.iter() {
            let line_score_option = line.compute_completion_score();
            match line_score_option {
                None => (),
                Some(line_score) => scores.push(line_score),
            }
        }

        scores.sort();

        let number_scores = scores.len();
        return scores[(number_scores - 1) / 2];
    }
}