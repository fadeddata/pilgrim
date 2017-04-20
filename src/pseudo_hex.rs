use std::collections::VecDeque;
use std::string::String;

pub fn num_to_pseudo_hex(num: usize) -> String {
    let new_base = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E',
                        'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
                        'W', 'X', 'Y', 'Z'];

    let mut value = num;

    if num == 0 {
        String::from("0")
    } else {
        let mut result = VecDeque::new();
        let numeric_base = new_base.len();
        while value > 0 {
            let remainder = value % numeric_base;
            value /= numeric_base;
            result.push_front(new_base[remainder]);
        }

        result.iter().cloned().collect::<String>()
    }
}

#[test]
fn test_num_to_pseudo_hex() {
    let base = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E',
                    'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
                    'W', 'X', 'Y', 'Z'];

    for idx in 0..base.len() {
        let mut str = String::new();
        str.push(base[idx]);
        assert_eq!(num_to_pseudo_hex(idx), str);
    }

    assert_eq!(num_to_pseudo_hex(34), "10");
    assert_eq!(num_to_pseudo_hex(35), "11");
    assert_eq!(num_to_pseudo_hex(44), "1A");
    assert_eq!(num_to_pseudo_hex(53), "1K");
    assert_eq!(num_to_pseudo_hex(67), "1Z");
    assert_eq!(num_to_pseudo_hex(101), "2Z");
}