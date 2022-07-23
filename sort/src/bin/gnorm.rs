fn main() {
    println!("[GNORM SORT]");
    let mut case1 = [8, 4, 3, 7, 6, 5, 2, 1];

    println!("[INPUT]: {:?}", case1);
    gnome_sort(&mut case1);

    println!("[OUTPUT]: {:?}", case1);
}

fn gnome_sort(slice: &mut [i32]) {
    let n = slice.len();

    for i in 1..n {
        if slice[i - 1] < slice[i] {
            continue;
        }

        slice.swap(i - 1, i);
        println!("i:{}       slice: {:?}", i, slice);

        let mut j = i - 1;
        loop {
            if j == 0 || slice[j - 1] < slice[j] {
                break;
            }

            slice.swap(j, j - 1);
            println!("i:{}, j:{}, slice: {:?}", i, j, slice);

            j -= 1;
        }
    }
}

// 8, 4, 3, 7, 6, 5, 2, 1
// 4, 8, 3, 7, 6, 5, 2, 1 i:1
// 4, 8, 3, 7, 6, 5, 2, 1 i:2

// 4, 3, 8, 7, 6, 5, 2, 1 i:2,swap
// 4, 3, 8, 7, 6, 5, 2, 1 i:2,j:1
// 4, 8, 3, 7, 6, 5, 2, 1 i:2,j:0
