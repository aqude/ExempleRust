use std::io;

fn celsfn(cels: u32, operation: fn (u32) -> u32) -> u32 {
    let result = operation(cels);
    result
}


fn main() {
    let mut cels = String::new();
    io::stdin().read_line(&mut cels).expect("Failed to read line");
    let cels: u32 = cels.trim().parse().expect("please give me correct string number!");
    // let cels = cels.parse::<u32>().unwrap();
    let fr: fn(u32) -> u32 = |cels| (cels  * 9/5) + 32;
    let f = celsfn(cels, fr);
    println!("{}far", f);    
}
