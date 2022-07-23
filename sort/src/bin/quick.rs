fn main() {
    println!("[QUICK SORT]");
    let mut case1 = [8, 4, 3, 7, 6, 5, 2, 1];
    // let mut case1 = [8,2,4,5,7,3,6,11,1,9,10];

    println!("[INPUT]: {:?}", case1);
    let end = case1.len() - 1;
    quick_sort(&mut case1, 0, end);

    println!("[OUTPUT]: {:?}", case1);
}

// returns index of pivot
fn partition(slice: &mut [i32], start: usize, end: usize) -> usize {
    // use the end element as pivot
    let pivot = slice[end];

    let mut p_index = start;

    for i in start..end {
        if slice[i] <= pivot {
            slice.swap(i, p_index);
            p_index = p_index + 1;
        }
    }
    slice.swap(p_index, end);

    return p_index;
}

fn quick_sort(slice: &mut [i32], start: usize, end: usize) {
    let p = partition(slice, start, end);
    println!("start:{}, end:{}, p:{}, slice: {:?}", start, end, p, slice);

    if start + 1 < p {
        quick_sort(slice, start, p - 1);
    }
    if p + 1 < end {
        quick_sort(slice, p + 1, end);
    }
}

// 8, 4, 3, 7, 6, 5, 2, 1
// 1, 4, 3, 7, 6, 5, 2, 8 p -> 0
// 1, 4, 3, 7, 6, 5, 2, 8
