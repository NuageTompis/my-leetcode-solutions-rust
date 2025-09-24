struct Solution;

impl Solution {
    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        // find earliest enf for both
        let mut end_land = i32::MAX;
        let mut end_water = i32::MAX;
        for (s, d) in land_start_time.iter().zip(land_duration.iter()) {
            end_land = end_land.min(s + d);
        }
        for (s, d) in water_start_time.iter().zip(water_duration.iter()) {
            end_water = end_water.min(s + d);
        }

        let mut ret = i32::MAX;
        for (s, d) in land_start_time.iter().zip(land_duration.iter()) {
            ret = ret.min(s.max(&end_water) + d);
        }
        for (s, d) in water_start_time.iter().zip(water_duration.iter()) {
            ret = ret.min(s.max(&end_land) + d);
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let land_start_time: Vec<i32> = vec![2, 8];
        let land_duration: Vec<i32> = vec![4, 1];
        let water_start_time: Vec<i32> = vec![6];
        let water_duration: Vec<i32> = vec![3];
        let res = Solution::earliest_finish_time(
            land_start_time,
            land_duration,
            water_start_time,
            water_duration,
        );
        let expected: i32 = 9; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let land_start_time: Vec<i32> = vec![5];
        let land_duration: Vec<i32> = vec![3];
        let water_start_time: Vec<i32> = vec![1];
        let water_duration: Vec<i32> = vec![10];
        let res = Solution::earliest_finish_time(
            land_start_time,
            land_duration,
            water_start_time,
            water_duration,
        );
        let expected: i32 = 14; // Fill in this value
        assert_eq!(res, expected);
    }
}
