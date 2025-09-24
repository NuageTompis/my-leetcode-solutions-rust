struct MinStack {
    min: i32,
    stack: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            min: i32::MAX,
            stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.min = self.min.min(val);
        self.stack.push((val, self.min));
    }

    fn pop(&mut self) {
        self.stack.pop();
        match self.stack.last() {
            Some((_, val)) => self.min = *val,
            None => self.min = i32::MAX,
        }
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.min
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut my_class: MinStack = MinStack::new();
        let val: i32 = -2;
        my_class.push(val);
        let val: i32 = 0;
        my_class.push(val);
        let val: i32 = -3;
        my_class.push(val);
        let res = my_class.get_min();
        let expected: i32 = -3; // Fill in this value
        assert_eq!(res, expected);
        my_class.pop();
        let res = my_class.top();
        let expected: i32 = 0; // Fill in this value
        assert_eq!(res, expected);
        let res = my_class.get_min();
        let expected: i32 = -2; // Fill in this value
        assert_eq!(res, expected);
    }
}
