// https://github.com/E869120/kyopro_educational_90/blob/main/problem/001.jpg

struct Case {
    l: i32,
    k: usize,
    a: Vec<i32>,
    expected: i32,
}

fn main() {
    let cases = [
        Case {
            l: 45,
            k: 2,
            a: vec![7, 11, 16, 20, 28, 34, 38],
            expected: 12,
        },
        Case {
            l: 100,
            k: 1,
            a: vec![28, 54, 81],
            expected: 46,
        },
        Case {
            l: 100,
            k: 2,
            a: vec![28, 54, 81],
            expected: 26,
        },
        Case {
            l: 1000,
            k: 4,
            a: vec![
                51, 69, 102, 127, 233, 295, 350, 388, 417, 466, 469, 523, 553, 587, 720, 739, 801,
                855, 926, 954,
            ],
            expected: 170,
        },
    ];

    for i in 0..cases.len() {
        let case = &cases[i];
        let res = yokan_party(case.k, case.l, &case.a);
        if case.expected != res {
            println!("expected: {}, res:{}", case.expected, res)
        } else {
            println!("case{} is ok!", i + 1);
        }
    }
}

fn yokan_party(
    k: usize,     // 選ぶ切れ目数
    l: i32,       // ようかんの長さ
    a: &Vec<i32>, // ようかんの切れ目の位置
) -> i32 {
    let mut left: i32 = -1;
    let mut right: i32 = l + 1;

    while right - left > 1 {
        let split_len = left + (right - left) / 2;

        let solved = solve(split_len, k, l, a);

        if solved {
            left = split_len;
        } else {
            right = split_len;
        }
    }

    return left;
}

// 特定の長さ以上で分割して、数がk以上になればOK
fn solve(split_len: i32, k: usize, l: i32, a: &Vec<i32>) -> bool {
    let mut pre = 0;
    let mut cnt = 0;

    for i in 0..a.len() {
        let cur_len = a[i] - pre;
        let rest_len = l - a[i];

        if cur_len >= split_len && rest_len >= split_len {
            cnt += 1;
            pre = a[i];
        }
    }

    cnt >= k
}
