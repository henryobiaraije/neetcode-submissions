impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();

    let mut result = vec![1; len];
    let mut result_index = 0;
    for i in 1..len {
        result[i] = nums[result_index] * result[result_index];
        result_index = i;
        // println!(
        //     "i={}, result_index={}, result={:?}",
        //     i, result_index, result
        // );
    }

    // [5, 4, 7, 6],
    // [1, 5, 20, 140]
    // [168, 210, 120, 140]

    // result_index = 3, n = 2,
    // result_index = 2, n = 1,
    // result_index = 1, n = 0,

    let mut suffix_product = nums.clone().pop().unwrap_or(1);
    // println!("before 2nd, suffix_product={:?}", suffix_product);
    for n in (0..len - 1).rev() {
        // println!("suffix product b={}", suffix_product);
        result[n] *= suffix_product;
        suffix_product *= nums[n];
        // println!(
        //     "n={},result={:?}, result_index={}, suffix_product={}",
        //     n, result, result_index, suffix_product
        // );
    }

    // println!("nums={:?},result={:?}", nums, result);

    // let _ = nums;
    // todo!()

    result
    }
}
