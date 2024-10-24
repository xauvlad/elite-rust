use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n: i32 = input.trim().parse().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let sequence: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect(); 
    
    let mut max_sum = sequence[0];
    let mut current_sum = 0;
    for num in sequence {
        current_sum += num;
        current_sum = current_sum.max(0);
        max_sum = max_sum.max(current_sum);
    }
    println!("{}", max_sum);
}
