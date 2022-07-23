fn main() {
    println!("[INSERTION SORT]");
    let mut case1 = [8, 4, 3, 7, 6, 5, 2, 1];

    println!("[INPUT]: {:?}", case1);
    insertion_sort(&mut case1);

    println!("[OUTPUT]: {:?}", case1);
}

fn insertion_sort(slice: &mut [i32]) {
    let n = slice.len();

    for i in 1..n {
        if slice[i - 1] > slice[i] {
            let tmp = slice[i];

            let mut j = i;
            loop {
                if j == 0 || slice[j - 1] <= tmp {
                    break;
                }

                slice[j] = slice[j - 1];
                println!("====> i:{},j:{} slice: {:?}", i, j, slice);
                j = j - 1;
            }
            slice[j] = tmp;
        }

        println!("i:{}, slice: {:?}", i, slice);
    }
}

// 8, 4, 3, 7, 6, 5, 2, 1 i:1
// 4, 8, 3, 7, 6, 5, 2, 1 i:2
// 3, 4, 8, 7, 6, 5, 2, 1 i:3

// 3, 4, 7, 8, 6, 5, 2, 1 i:4
// 3, 4, 7, 8, 8, 5, 2, 1 i:4 j:4 (keep: 6)
// 3, 4, 7, 7, 8, 5, 2, 1 i:4 j:3
// 3, 4, 6, 7, 8, 5, 2, 1 i:4 j:2
// break;

// 3, 4, 6, 7, 8, 5, 2, 1 i:5
// 3, 4, 5, 6, 7, 8, 2, 1 i:6
// 2, 3, 4, 5, 6, 7, 8, 1 i:7
// 1, 2, 3, 4, 5, 6, 7, 8 i:8
