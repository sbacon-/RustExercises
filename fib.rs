use std::io;

fn main() {
    let mut input = String::new();
    let n: u32;
    loop{
        println!("Enter a number:");
        io::stdin().
            read_line(&mut input).
            expect("Unable to parse input!");
        n = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }
    for num in 0..=n{
        println!("F({}) = {}",num,fib(num));
    }
}

fn fib(n:u32)->u32{
    if n<=0 {
        return 0;
    }else if n==1{
        return 1;
    }
    fib(n-1) + fib(n-2)
}
