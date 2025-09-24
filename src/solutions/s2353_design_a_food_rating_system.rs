use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

struct FoodRatings {
    heaps: HashMap<String, BinaryHeap<(i32, Reverse<String>)>>, // cuisine -> heaps
    food_to_cuisine: HashMap<String, String>,                   // food -> cuisine
    food_to_rating: HashMap<String, i32>,                       // food -> rating
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut heaps = HashMap::new();
        let mut food_to_cuisine = HashMap::new();
        let mut food_to_rating = HashMap::new();
        for ((f, c), r) in foods.iter().zip(cuisines).zip(ratings) {
            heaps
                .entry(c.clone())
                .or_insert_with(BinaryHeap::new)
                .push((r, Reverse(f.clone())));
            food_to_cuisine.insert(f.clone(), c);
            food_to_rating.insert(f.to_owned(), r);
        }
        FoodRatings {
            heaps,
            food_to_cuisine,
            food_to_rating,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        self.food_to_rating.insert(food.clone(), new_rating);
        let heap = self
            .heaps
            .get_mut(self.food_to_cuisine.get(&food).unwrap())
            .unwrap();
        heap.push((new_rating, Reverse(food)));
    }

    fn highest_rated(&mut self, cuisine: String) -> String {
        let heap = self.heaps.get_mut(&cuisine).unwrap();
        while let Some((rating, food)) = heap.pop() {
            let true_rating = *self.food_to_rating.get(&food.0).unwrap();
            if true_rating == rating {
                heap.push((rating, food.clone()));
                return food.0;
            }
        }
        panic!()
    }
}

/**
 * Your FoodRatings object will be instantiated and called as such:
 * let obj = FoodRatings::new(foods, cuisines, ratings);
 * obj.change_rating(food, new_rating);
 * let ret_2: String = obj.highest_rated(cuisine);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let foods: Vec<String> = vec![
            "kimchi".into(),
            "miso".into(),
            "sushi".into(),
            "moussaka".into(),
            "ramen".into(),
            "bulgogi".into(),
        ];
        let cuisines: Vec<String> = vec![
            "korean".into(),
            "japanese".into(),
            "japanese".into(),
            "greek".into(),
            "japanese".into(),
            "korean".into(),
        ];
        let ratings: Vec<i32> = vec![9, 12, 8, 15, 14, 7];
        let mut my_class: FoodRatings = FoodRatings::new(foods, cuisines, ratings);
        let cuisine: String = "korean".into();
        let res = my_class.highest_rated(cuisine);
        let expected: String = "kimchi".into(); // Fill in this value
        assert_eq!(res, expected);
        let cuisine: String = "japanese".into();
        let res = my_class.highest_rated(cuisine);
        let expected: String = "ramen".into(); // Fill in this value
        assert_eq!(res, expected);
        let food: String = "sushi".into();
        let new_rating: i32 = 16;
        my_class.change_rating(food, new_rating);
        let cuisine: String = "japanese".into();
        let res = my_class.highest_rated(cuisine);
        let expected: String = "sushi".into(); // Fill in this value
        assert_eq!(res, expected);
        let food: String = "ramen".into();
        let new_rating: i32 = 16;
        my_class.change_rating(food, new_rating);
        let cuisine: String = "japanese".into();
        let res = my_class.highest_rated(cuisine);
        let expected: String = "ramen".into(); // Fill in this value
        assert_eq!(res, expected);
    }
}
