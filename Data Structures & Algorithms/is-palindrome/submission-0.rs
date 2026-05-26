impl Solution {
    pub fn is_palindrome(s: String) -> bool {
     
        let all = s
            .chars()
            .filter(|item| item.is_alphanumeric())
            .flat_map(|item| item.to_lowercase())
            .collect::<Vec<char>>();

        if all.is_empty() {
            return true;
        }
        if all.len() == 2 {
            let is_same = all[0] == all[1];
            return is_same;
        }


        let len = all.len();
        let half = len / 2;

        for (a, left) in all.iter().enumerate() {
            if a >= half {
                return true;
            }
            let is_same = Some(left) == all.get(len - 1 - a);
            if !is_same {
                return false;
            }
        }

        true
    }
}
