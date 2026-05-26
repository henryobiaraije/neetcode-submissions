impl Solution {

pub fn encode(strs: Vec<String>) -> String {
    strs.iter().fold(String::new(), |mut acc, item| {
        let len = item.len();
        let mut len_string = format!("{}", len);
        if len < 10 {
            len_string = format!("00{}", len);
        } else if len < 100 {
            len_string = format!("0{}", len);
        } else {
            len_string = format!("{}", len);
        }

        acc = format!("{}{}{}", acc, len_string, item);
        acc
    })
}

pub fn decode(s: String) -> Vec<String> {
    let mut temp_store = String::new();
    let mut len_string = "".to_string();
    let mut chars_iter = s.chars().peekable();
    let mut acc = Vec::<String>::new();

    while let Some(char) = chars_iter.next() {
        if len_string.len() < 3 {
            len_string.push(char);

            if len_string.len() == 3 {
                let len: usize = len_string.parse().unwrap_or(0);
                if len == 0 {
                    acc.push("".to_string());
                    len_string = String::new();
                } else {
                    for _ in 0..len {
                        if let Some(c) = chars_iter.next() {
                            temp_store.push(c);
                        }
                    }
                    acc.push(temp_store.clone());
                    temp_store = String::new();
                    len_string = String::new();
                }
            }
        }
    }
    acc
}
}