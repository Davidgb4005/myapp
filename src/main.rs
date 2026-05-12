use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();
    let n1:u32 = rng.gen_range(0..10);
    if n1 > 5{
        println!("Greater than 5");
    }
    else {
        println!("Less than or Equal to 5")
    }
    0;
}
