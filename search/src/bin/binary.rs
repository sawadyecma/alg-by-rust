fn main(){
    println!("[BINARY SEARCH]");
    process(&[1,2,3,4,5,6,7,8,9,10], 0);
    process(&[1,2,3,4,5,6,7,8,9,10], 1);
    process(&[1,2,3,4,5,6,7,8,9,10],2);
    process(&[1,2,3,4,5,6,7,8,9,10], 3);
    process(&[1,2,3,4,5,6,7,8,9,10], 4);
    process(&[1,2,3,4,5,6,7,8,9,10], 5);
    process(&[1,2,3,4,5,6,7,8,9,10],6);
    process(&[1,2,3,4,5,6,7,8,9,10], 7);
    process(&[1,2,3,4,5,6,7,8,9,10], 8);
    process(&[1,2,3,4,5,6,7,8,9,10], 9);
    process(&[1,2,3,4,5,6,7,8,9,10], 10);
}

fn process(slice: &[i32],tar: i32){
    let result = binary_search(&slice, tar);
    println!("[OUTPUT] index of {} is ... {}",tar, result.unwrap_or(999));
}

fn binary_search(slice: &[i32],tar: i32)->Option<usize>{
    let mut st = 0;
    let mut ed = slice.len()-1;

    loop{
        let p: usize = (ed + st)/2;

        if slice[p] < tar{
            st = p+1;
        }else{
            ed = p;
        }

        if slice[st] == tar{
            return Some(st);
        }

        if st == ed {
            return None;
        }
    }
}

