use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    println!("{}.{}.{}.{}", rng.gen_range(0..256), rng.gen_range(0..256), rng.gen_range(0..256), rng.gen_range(0..256));
}
