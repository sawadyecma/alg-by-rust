use std::collections::VecDeque;

fn main(){
    println!("stack-queue sample");

    stack_example(); 
    queue_example();
}

fn stack_example(){
    // let mut stack: Vec<i32> = (0..10).collect();
    let mut stack = vec![1,2,3];

    println!("{:?}",stack);

    let popped = stack.pop().unwrap();
    println!("{:?}",popped);

    stack.push(11);
    stack.push(12);
    let popped = stack.pop().unwrap();
    println!("{:?}",popped);
    let popped = stack.pop().unwrap();
    println!("{:?}",popped);

    println!("loop start");
    for item in stack.iter(){
        println!("{}",item);
    }
    println!("loop end");

    println!("loop start(mutable)");
    for item in stack.iter_mut(){
        println!("{}",item);
        *item = *item*3;
    }
    println!("loop end(mutable)");
    println!("{:?}",stack);


}

fn queue_example(){
    let mut queue:VecDeque<i32> = VecDeque::new();
    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);
    queue.push_front(0);
    println!("{:?}",queue);

    println!("popped: {}",queue.pop_front().unwrap());
    println!("popped: {}",queue.pop_front().unwrap());
    println!("popped: {}",queue.pop_front().unwrap());


    println!("{:?}",queue);

}