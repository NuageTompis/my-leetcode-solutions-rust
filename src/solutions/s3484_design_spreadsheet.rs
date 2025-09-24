struct Spreadsheet {
    cells: Vec<i32>,
}

impl Spreadsheet {
    fn new(rows: i32) -> Self {
        Self {
            cells: vec![0; (26 * (rows + 1)) as usize],
        }
    }

    fn parse(cell: &str) -> usize {
        let column = (cell.as_bytes()[0] - b'A') as usize;
        let row = (cell[1..].parse::<usize>()).unwrap();
        row * 26 + column
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        let ndx = Self::parse(&cell);
        self.cells[ndx] = value;
    }

    fn reset_cell(&mut self, cell: String) {
        self.set_cell(cell, 0);
    }

    fn get_value(&self, formula: String) -> i32 {
        let mut sum = 0;
        for cell in formula[1..].split('+') {
            if cell.as_bytes()[0].is_ascii_digit() {
                sum += cell.parse::<i32>().unwrap();
            } else {
                sum += self.cells[Self::parse(cell)]
            }
        }

        sum
    }
}

/**
 * Your Spreadsheet object will be instantiated and called as such:
 * let obj = Spreadsheet::new(rows);
 * obj.set_cell(cell, value);
 * obj.reset_cell(cell);
 * let ret_3: i32 = obj.get_value(formula);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let rows: i32 = 3;
        let mut my_class: Spreadsheet = Spreadsheet::new(rows);
        let formula: String = "=5+7".into();
        let res = my_class.get_value(formula);
        let expected: i32 = 12; // Fill in this value
        assert_eq!(res, expected);
        let cell: String = "A1".into();
        let value: i32 = 10;
        my_class.set_cell(cell, value);
        let formula: String = "=A1+6".into();
        let res = my_class.get_value(formula);
        let expected: i32 = 16; // Fill in this value
        assert_eq!(res, expected);
        let cell: String = "B2".into();
        let value: i32 = 15;
        my_class.set_cell(cell, value);
        let formula: String = "=A1+B2".into();
        let res = my_class.get_value(formula);
        let expected: i32 = 25; // Fill in this value
        assert_eq!(res, expected);
        let cell: String = "A1".into();
        my_class.reset_cell(cell);
        let formula: String = "=A1+B2".into();
        let res = my_class.get_value(formula);
        let expected: i32 = 15; // Fill in this value
        assert_eq!(res, expected);
    }
}
