fn main() {
    println!("[SHAKER SORT]");
    let mut case1 = [8, 4, 3, 7, 6, 5, 2, 1];

    println!("[INPUT]: {:?}", case1);
    shaker_sort(&mut case1);

    println!("[OUTPUT]: {:?}", case1);
}

fn shaker_sort(slice: &mut [i32]) {
    let mut swap_cnt = 0;
    let mut cmp_cnt = 0;

    let mut last_swap_index = slice.len() - 1;
    let mut top_swap_index = 0;

    loop {
        // 順方向
        for i in top_swap_index..last_swap_index {
            cmp_cnt += 1;
            if slice[i] > slice[i + 1] {
                slice.swap(i, i + 1);
                swap_cnt += 1;
            }
        }
        println!("{:?}", slice);
        last_swap_index -= 1;
        if top_swap_index == last_swap_index {
            break;
        }

        let mut i = last_swap_index;
        // 逆方向
        loop {
            cmp_cnt += 1;
            if slice[i] < slice[i - 1] {
                slice.swap(i, i - 1);
                swap_cnt += 1;
            }

            i -= 1;
            if i == top_swap_index {
                break;
            }
        }
        println!("{:?}", slice);

        top_swap_index += 1;

        if top_swap_index == last_swap_index {
            break;
        }
    }

    println!("swap_cnt is {}", swap_cnt);
    println!("cmp_cnt is {}", cmp_cnt);
}
