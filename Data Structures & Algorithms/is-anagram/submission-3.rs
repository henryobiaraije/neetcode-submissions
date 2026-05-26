impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
            let is_empty = s.is_empty() || t.is_empty();
    let is_not_same_length = s.len() != t.len();
    if is_empty || is_not_same_length {
        return false;
    }

    let s_hash = match s
        .chars()
        .try_fold(HashMap::<char, u64>::new(), |mut acc, item| {
            *acc.entry(item).or_insert(0) += 1;

            Some(acc)
        }) {
        Some(v) => v,
        _ => return false,
    };

    let t_hash = match t
        .chars()
        .try_fold(HashMap::<char, u64>::new(), |mut acc, item| {
            // let one = acc.entry(item);
            let one_value = *s_hash.get(&item).unwrap_or(&0);
            let has_more_than_s = *acc.entry(item).or_default() > one_value;
            if has_more_than_s {
                return None;
            }
            acc.insert(item, acc.get(&item).ok_or(0).unwrap() + 1);
            Some(acc)
        }) {
        Some(v) => v,
        _ => return false,
    };

    println!("s_hash = {:?} | s={} | t={}", s_hash, s, t);
    println!("t_hash = {:?} | s={} | t={}", t_hash, s, t);

    s_hash == t_hash

    }
}
