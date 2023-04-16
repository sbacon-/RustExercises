use std::io;

fn main() {
    let mut temp = String::new();
    let tmp: i32;
    loop{
        println!("Enter a number: ");
        io::stdin()
            .read_line(&mut temp)
            .expect("Unable to read input!");
        tmp = match temp.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };
        break;
    }
    
    println!("{}F is {}C",tmp,f_to_c(tmp));
    println!("{}C is {}F",tmp,c_to_f(tmp));
}

fn f_to_c(temp: i32) -> f32{
    let temp = temp - 32;
    let temp = temp as f32 / 1.8;
    temp
}

fn c_to_f(temp: i32) -> f32{
    let temp = temp as f32 * 1.8;
    temp + 32.00
}
