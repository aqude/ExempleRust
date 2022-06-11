use std::fmt;

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}
impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{:?}", self)
    }
}

fn main() {
    let home  = IpAddr::V4(String::from("127.0.0.1"));
    let addres = IpAddr::V6(String::from("::1"));
    println!("{} \n {}", home, addres);
}

