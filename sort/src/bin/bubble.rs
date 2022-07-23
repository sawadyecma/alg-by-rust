fn main() {
    println!("[BABLE SORT]");
    let mut case1 = [8, 4, 3, 7, 6, 5, 2, 1];

    println!("[INPUT]: {:?}", case1);
    bubble_sort(&mut case1);

    println!("[OUTPUT]: {:?}", case1);
}

fn bubble_sort(slice: &mut [i32]) {
    let mut swap_cnt = 0;
    let mut cmp_cnt = 0;
    for i in 0..slice.len() - 1 {
        for j in 0..slice.len() - i - 1 {
            // println!("i:{},j:{}",i,j);
            if slice[j] > slice[j + 1] {
                let tmp = slice[j];
                slice[j] = slice[j + 1];
                slice[j + 1] = tmp;

                // println!("{:?}",slice);
                swap_cnt += 1;
            }
            cmp_cnt += 1;
        }
        println!("{:?}", slice);
    }
    println!("swap_cnt is {}", swap_cnt);
    println!("cmp_cnt is {}", cmp_cnt);
}

// n = 8
// cmp_cnt =
// i = 0 のとき (0,1,2,3,4,5,6)
// i = 1 のとき (0,1,2,3,4,5)
// i = 2 のとき (0,1,2,3,4)
// i = 3 のとき (0,1,2,3)
// i = 4 のとき (0,1,2)
// i = 5 のとき (0,1)
// i = 6 のとき (0)

// 1+2+3+4+...+n-1 = (n)*(n-1)/2
