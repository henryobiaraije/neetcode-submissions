impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
      let mut result = [].to_vec();

    let mut store: HashMap<i32, i32> = HashMap::new();
    for (ind, item) in nums.iter().enumerate() {
        let index = ind.try_into().unwrap_or(0);
        let wants_item = store.contains_key(&item);
        if wants_item {
            let index_1 = store.get(item).unwrap_or(&0);
            let index_2 = index;
            println!(
                "RETURN: nums = {:?}, index = {}, item={}, target={}, store={:?}, index_1 = {}, index_2 = {}",
                nums, index, item, target, store, index_1, index_2,
            );
            return [*index_1, index_2].to_vec();
        }
        let we_need = target - (*item);
        store.insert(we_need, index);
        println!(
            "nums = {:?}, index = {}, item={}, target={}, we_need={}, store={:?}",
            nums, index, item, target, we_need, store,
        );
    }

    result
    }
}
