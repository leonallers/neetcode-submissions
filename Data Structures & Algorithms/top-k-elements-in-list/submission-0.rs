impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {

        let length = nums.len();
        let mut counts: HashMap<i32, i32> = HashMap::new();
        for num in nums {
           *counts.entry(num).or_insert(0) += 1
        }

        let mut bucket: Vec<Vec<i32>> = vec![vec![]; length + 1];
        for (key, value) in counts {
            bucket[value as usize].push(key);
        }

        let mut result: Vec<i32> = vec![];
        for i in (0..bucket.len()).rev() {
            for &inner in &bucket[i] {
                result.push(inner);
                if result.len() == k as usize {
                    return result;
                }
            }
        }
        result
    }
}
