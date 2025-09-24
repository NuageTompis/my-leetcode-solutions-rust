struct Solution;

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut heap = std::collections::BinaryHeap::new();
        heap.push((a, 'a'));
        heap.push((b, 'b'));
        heap.push((c, 'c'));

        let mut first = heap.pop().unwrap();
        let second = heap.pop().unwrap();
        let third = heap.pop().unwrap();
        first.0 = first.0.min((1 + second.0 + third.0) << 1);

        heap.push(first);
        heap.push(second);
        heap.push(third);

        let mut last = None;
        let mut res = String::new();
        while let Some(mut cv) = heap.pop() {
            if cv.0 == 0 {
                break;
            }
            let amt = if cv.0 == 1 { 1 } else { 2 };
            cv.0 -= amt;

            let mut cv_2 = heap.pop().unwrap();
            cv_2.0 -= 1;

            let invert = match last {
                None => {
                    last = Some(cv_2.1);
                    false
                }
                Some(c) => {
                    if c == cv.1 {
                        last = Some(cv.1);
                        true
                    } else {
                        last = Some(cv_2.1);
                        false
                    }
                }
            };

            if invert {
                if cv_2.0 >= 0 {
                    res.push(cv_2.1);
                }
                for _ in 0..amt {
                    res.push(cv.1);
                }
            } else {
                for _ in 0..amt {
                    res.push(cv.1);
                }
                if cv_2.0 >= 0 {
                    res.push(cv_2.1);
                }
            }

            heap.push(cv_2);
            heap.push(cv);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_2() {
        let a: i32 = 7;
        let b: i32 = 1;
        let c: i32 = 0;
        let res = Solution::longest_diverse_string(a, b, c);
        let expected: String = "aabaa".into(); // Fill in this value
        assert_eq!(res, expected);
    }
}
