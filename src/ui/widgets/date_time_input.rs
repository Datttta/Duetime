pub struct DateTimeInput {
    pub value: String,
    pub cursor: usize,
    pub editable_positions: &'static [usize],
}

impl DateTimeInput {
    pub fn move_left(&mut self) {
        if let Some(position) = self
            .editable_positions
            .iter()
            .position(|&pos| pos == self.cursor)
        {
            if position > 0 {
                self.cursor = self.editable_positions[position - 1];
            }
        }
    }

    pub fn move_right(&mut self) {
        if let Some(position) = self
            .editable_positions
            .iter()
            .position(|&pos| pos == self.cursor)
        {
            if position + 1 < self.editable_positions.len() {
                self.cursor = self.editable_positions[position + 1];
            }
        }
    }

    pub fn zero_backspace(&mut self) {
        self.value
            .replace_range(self.cursor..self.cursor + 1, &0.to_string());

        self.move_left();
    }
    
    pub fn time_backspace(&mut self) {
        self.value
            .replace_range(self.cursor..self.cursor + 1, &'-'.to_string());

        self.move_left();
    }

    pub fn insert_digit(&mut self, digit: char) {
        if !digit.is_ascii_digit() {
            return;
        }

        self.value
            .replace_range(self.cursor..self.cursor + 1, &digit.to_string());

        self.move_right();
    }
}
