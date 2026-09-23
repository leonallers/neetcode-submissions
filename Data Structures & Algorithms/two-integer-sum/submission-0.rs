impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut index_map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for (index, &element) in nums.iter().enumerate() {
            let i = index as i32;
            let complement = target - element;
            if let Some(&value) = index_map.get(&complement) {
                return vec![value, i];
            }
            index_map.insert(element, i);
        }
        vec![]
    }
}
