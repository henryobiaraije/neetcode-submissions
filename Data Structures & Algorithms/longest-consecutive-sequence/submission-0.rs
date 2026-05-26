impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
let mut hash_set: HashSet<i32> = HashSet::new();
    for a in 0..nums.len() {
        hash_set.insert(nums[a]);
    }

    let mut longest = 0;

    for a in nums {
        let has_left = hash_set.contains(&(a - 1));
        let is_sequanece_start = !has_left;
        if is_sequanece_start {

            let mut hold = a;
            let mut local_longest = 1;

            while hash_set.contains(&(hold + 1)) {
                hold += 1;
                local_longest += 1;
            }
            if local_longest > longest {
                longest = local_longest;
            }
        }
    }

    return longest;
    }
}
