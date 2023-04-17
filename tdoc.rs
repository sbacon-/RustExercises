fn main() {
    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];
    let gifts = [
        "a partrige in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a leaping",
        "eleven pipers piping",
        "twelve drummers drumming"
    ];
    let mut n = 1;
    loop{
        christmas_song(n,days,gifts);
        n+=1;
        println!("");
        if n==13 {
            break;
        }
    }
}

fn christmas_song(n:u32 ,days:[&str;12], gifts:[&str;12]){
    for day in (0..n).rev(){
        if day==n-1{
            println!("On the {} day of Christmas my true love sent to me ",days[day as usize]);
            println!("{}",gifts[day as usize]);
        }else if day==0{
            println!("and {}",gifts[day as usize]);
        }else{
            println!("{}",gifts[day as usize]);
        }
    }
}
