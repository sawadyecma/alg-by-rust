fn main(){
    println!("[PART SUM]");
    let input = [1,2,4,7];
    let k = 13;
    can_part_sum(&input,&k);

    // let input = [1,2,4,1];
    // let k = 13;
    // can_part_sum(&input,&k);
}

fn can_part_sum(slice: &[i32], expect: &i32)->bool{
    println!("slice: {:?}",slice);
    println!("expect: {}",expect);
    let result = dfs_recursive(&0,&0, &slice.len(), expect,slice);
    println!("reault: {}",result);
    result
}

fn dfs_recursive(i: &usize,sum: &i32, n:&usize,expect: &i32,slice: &[i32])->bool{
    println!("dfs!, i: {}, sum: {}",i,sum);
    if i == n {
        return sum == expect
    }

    // kepp or add next node 
    if dfs_recursive(&(i+1),sum,n,expect,slice) || dfs_recursive(&(i+1),&(sum+slice[*i]),n,expect,slice){
        return true
    }

    false
}