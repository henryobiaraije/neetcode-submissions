use itertools::Itertools;
struct Group {
    freq: usize,
    value: i32,
}
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {

    
    let hash: HashMap<i32, Group> =
        nums.iter()
            .fold(HashMap::<i32, Group>::new(), |mut acc, item| {
                if let Some(x) = acc.get(item) {
                    acc.insert(
                        *item,
                        Group {
                            freq: x.freq + 1,
                            value: *item,
                        },
                    );
                } else {
                    acc.insert(
                        *item,
                        Group {
                            freq: 1,
                            value: *item,
                        },
                    );
                }
                acc
            });

    hash.values()
        .clone()
        .sorted_by(|a, b| Ord::cmp(&b.freq, &a.freq))
        .take(usize::try_from(k).unwrap_or(1))
        .rev()
        .fold(Vec::<i32>::new(), |mut acc, item| {
            acc.push(item.value);
            acc
        })
    }
}
