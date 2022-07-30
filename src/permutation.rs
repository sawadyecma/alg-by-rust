#[allow(dead_code)]
pub fn next_permutation(cur: Vec<i32>) -> Vec<i32> {
    let mut is_last = true;
    for i in 1..cur.len() {
        if cur[i - 1] < cur[i] {
            is_last = false;
            break;
        }
    }
    if is_last {
        return vec![];
    }

    let len = cur.len();

    let mut copied = cur.clone();

    let mut left = 0;
    let mut right = len;

    for i in (0..len).rev() {
        if i == 0 {
            break;
        }

        if cur[i - 1] < cur[i] {
            left = i - 1;
            break;
        }
    }

    for i in left..len {
        if cur[left] > cur[i] {
            right = i;
            break;
        }
    }

    copied.swap(left, right - 1);

    let mut tar = copied.get(left + 1..len).unwrap().to_vec();

    // println!(
    //     "left:{}({}),right:{}({})",
    //     cur[left],
    //     left,
    //     cur[right - 1],
    //     right
    // );
    // println!("copied:{:?}", copied);
    // println!("tar: {:?}", tar);

    tar.reverse();

    for i in 0..tar.len() {
        copied[left + 1 + i] = tar[i];
    }

    if copied.eq(&cur) {
        return vec![];
    };

    return copied;
}

// [1,2,3]の場合

// [1,2,3]
// [1,3,2]
// [2,1,3]
// [2,3,1]
// [3,1,2]
// [3,2,1]

// [1,2,3,4]の場合
// 4 * 3 * 2 = 24

// まずは先頭を固定
// 残りを並び替える
//   ますは先頭を固定
//   残りを並び替える
//   並べ替え終わったら
//   次の並べ替えはやらない

// [1,2,3,4]
// [1,2,4,3]
// [1,3,2,4]
// [1,3,4,2]
// [1,4,2,3]
// [1,4,3,2]

// [2,1,3,4]
// [2,1,4,3]
// [2,3,1,4]
// [2,3,4,1]
// [2,4,1,3]
// [2,4,3,1]

// [3,1,2,4]
// [3,1,4,2]
// [3,2,1,4]
// [3,2,4,1]
// [3,4,1,2]
// [3,4,2,1]

// [4,1,2,3]
// [4,1,3,2]
// [4,2,1,3]
// [4,2,3,1]
// [4,3,1,2]
// [4,3,2,1]
