
fn main(){
    println!("[COMB SORT]");
    let mut case1 = [8, 4, 3, 7, 6, 5, 2, 1];
    
    println!("[INPUT]: {:?}",case1);
    comb_sort( &mut case1);

    println!("[OUTPUT]: {:?}", case1);
}

fn comb_sort(slice: &mut [i32]){
    let n = slice.len();
    let mut h = (n as f32 / 1.3).floor() as usize;
    
    loop{

        let mut i = 0;
        let mut swaps = 0;
        
        print!("[h: {}]===>",h);

        loop{
            if slice[i]>slice[i+h] {
                slice.swap(i, i+h);
                swaps += 1;
            }

            i+=1;
            if i+h >= n {
                break;
            }
        }

        println!("{:?}",slice);

        if h == 1 {
            if swaps == 0{
                break;
            }
        }else{
            h = (h as f32 / 1.3).floor() as usize;
        }
    }
    
}