use permutation::next_permutation;

fn main() {
    // let result = next_permutation(vec![1, 3, 2]);
    // println!("result: {:?}", result);
    // let result = next_permutation(vec![2, 1, 3]);
    // println!("result: {:?}", result);
    // let result = next_permutation(vec![2, 3, 1]);
    // println!("result: {:?}", result);
    // let result = next_permutation(vec![3, 1, 2]);
    // println!("result: {:?}", result);

    // let result = next_permutation(vec![3, 2, 1]);
    // println!("result: {:?}", result);

    // let result = next_permutation(vec![3, 4, 2, 1]);
    // println!("result: {:?}", result);

    exec();
}

fn exec() {
    let input = vec![1, 2, 3, 4, 5];

    let mut result: Vec<i32> = input;
    let mut cnt = 0;
    while result.len() > 0 {
        println!("{:?}", result);
        cnt += 1;

        result = next_permutation(result);
    }

    println!("cnt:{}", cnt);
}
