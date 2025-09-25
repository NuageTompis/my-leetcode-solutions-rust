struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let l = height.len();
        if l < 3 {
            return 0;
        }

        let mut peaks = Vec::new();
        let mut max = 0;
        // left peaks
        for (i, &h) in height.iter().enumerate() {
            if h > max {
                max = h;
                peaks.push(i);
            }
        }

        let highest = match peaks.last() {
            Some(l) => *l,
            None => return 0,
        };

        // right peaks
        max = 0;
        let mut peaks_right = Vec::new();
        for (i, &h) in height.iter().enumerate().rev() {
            if i == highest {
                break;
            }
            if h > max {
                max = h;
                peaks_right.push(i);
            }
        }
        peaks_right.reverse();
        peaks.append(&mut peaks_right);

        let mut water = 0;
        for x in peaks.windows(2) {
            let ndx1 = x[0];
            let ndx2 = x[1];
            let min = height[ndx1].min(height[ndx2]);
            for h in height.iter().take(ndx2).skip(ndx1 + 1) {
                water += min - h;
            }
        }

        water
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let height: Vec<i32> = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let res = Solution::trap(height);
        let expected: i32 = 6; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let height: Vec<i32> = vec![4, 2, 0, 3, 2, 5];
        let res = Solution::trap(height);
        let expected: i32 = 9; // Fill in this value
        assert_eq!(res, expected);
    }
}
