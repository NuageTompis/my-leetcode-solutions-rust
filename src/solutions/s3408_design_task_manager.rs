use std::collections::{BinaryHeap, HashMap};

pub struct TaskManager {
    map: HashMap<i32, (i32, i32)>, // task id -> (user id, priority)
    heap: BinaryHeap<(i32, i32)>,  // (priority, task id)
}

impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut manager = TaskManager {
            map: HashMap::new(),
            heap: BinaryHeap::new(),
        };
        for t in &tasks {
            manager.add(t[0], t[1], t[2]);
        }
        manager
    }

    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.map.insert(task_id, (user_id, priority));
        self.heap.push((priority, task_id));
    }

    fn edit(&mut self, task_id: i32, new_priority: i32) {
        let user_id = self.map.get(&task_id).unwrap().0;
        self.add(user_id, task_id, new_priority);
    }

    fn rmv(&mut self, task_id: i32) {
        self.map.remove(&task_id).unwrap();
    }

    fn exec_top(&mut self) -> i32 {
        while let Some((priority, task_id)) = self.heap.pop() {
            if let Some(&(uid, prio)) = self.map.get(&task_id) {
                if prio == priority {
                    self.map.remove(&task_id);
                    return uid;
                }
            }
        }
        -1
    }
}

/**
 * Your TaskManager object will be instantiated and called as such:
 * let obj = TaskManager::new(tasks);
 * obj.add(userId, taskId, priority);
 * obj.edit(taskId, newPriority);
 * obj.rmv(taskId);
 * let ret_4: i32 = obj.exec_top();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let tasks: Vec<Vec<i32>> = vec![vec![1, 101, 10], vec![2, 102, 20], vec![3, 103, 15]];
        let mut my_class: TaskManager = TaskManager::new(tasks);
        let user_id: i32 = 4;
        let task_id: i32 = 104;
        let priority: i32 = 5;
        my_class.add(user_id, task_id, priority);
        let task_id: i32 = 102;
        let new_priority: i32 = 8;
        my_class.edit(task_id, new_priority);
        let res = my_class.exec_top();
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
        let task_id: i32 = 101;
        my_class.rmv(task_id);
        let user_id: i32 = 5;
        let task_id: i32 = 105;
        let priority: i32 = 15;
        my_class.add(user_id, task_id, priority);
        let res = my_class.exec_top();
        let expected: i32 = 5; // Fill in this value
        assert_eq!(res, expected);
    }
}
