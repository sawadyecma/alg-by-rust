// https://github.com/E869120/kyopro_educational_90/blob/main/problem/002.jpg

fn main() {
    parentheses(4);
    parentheses(6);
    parentheses(8);
    parentheses(10);
    parentheses(12);
}

fn parentheses(n: i32) {
    let mut cnt = 0;
    for i in 0..1 << n {
        let mut candidate = String::from("");
        for j in (0..n).rev() {
            if i & (1 << j) == 0 {
                candidate += "("
            } else {
                candidate += ")"
            }
        }

        let judged = judge(&candidate);
        if judged {
            // println!("i:{}, candidate: {}, judged: {}", i, candidate, judged);
            cnt += 1;
        }
    }
    println!("cnt:{}", cnt)
}

fn judge(s: &String) -> bool {
    let mut dep = 0;

    for c in s.chars() {
        if c == '(' {
            dep += 1;
        } else if c == ')' {
            dep -= 1;
        }

        if dep < 0 {
            return false;
        }
    }

    dep == 0
}
