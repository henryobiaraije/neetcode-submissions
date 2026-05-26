impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        if nums.len() < 2 {
        return false;
    }
    //let mut others: Vec<i32> = [nums[0]].to_vec();
    let mut others: HashSet<i32> = HashSet::from([nums[0]]);
    nums.iter()
        .skip(1)
        .find(|item| {
            if others.contains(*item) {
                true
            } else {
                others.insert(**item);
                false
            }
        })
        .is_some()

    }

}
