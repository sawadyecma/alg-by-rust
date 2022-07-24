fn main() {
    println!("[BINARY SEARCH]");
    search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 0);
}

pub fn search(slice: &[i32], tar: i32) -> usize {
    let result = binary_search(&slice, tar);
    println!("[OUTPUT] index of {} is ... {}", tar, result.unwrap_or(999));

    result.unwrap_or(999)
}

fn binary_search(slice: &[i32], tar: i32) -> Option<usize> {
    let mut st = 0;
    let mut ed = slice.len() - 1;

    loop {
        let p: usize = (ed + st) / 2;

        if slice[p] < tar {
            st = p + 1;
        } else {
            ed = p;
        }

        if slice[st] == tar {
            return Some(st);
        }

        if st == ed {
            return None;
        }
    }
}

mod tests {
    #[test]
    fn test_search() {
        assert_eq!(crate::search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 0), 999);
        assert_eq!(crate::search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1), 0);
        assert_eq!(crate::search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 2), 1);
        assert_eq!(crate::search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3), 2);
        assert_eq!(crate::search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4), 3);
        assert_eq!(crate::search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), 4);
        assert_eq!(crate::search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 6), 5);
        assert_eq!(crate::search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 7), 6);
        assert_eq!(crate::search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 8), 7);
        assert_eq!(crate::search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 9), 8);
        assert_eq!(crate::search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10), 9);
    }
}
