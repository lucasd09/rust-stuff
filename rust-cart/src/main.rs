use std::time::{Duration, Instant};
use std::thread;


struct Product {
    prod : String,
    price : u32,
}

fn cart_sum(prod1 : Product,prod2 : Product,prod3 : Product) -> u32{
    let mut sum = 0;
    let prods = [prod1,prod2,prod3];
    let mut i = 0;
    loop
    {
        if i == 3 {
            break;
        }
        sum += prods[i].price;
        i += 1;
    }
    return sum;
}

fn create_cart(){
    let prod1 = Product{prod :String::from("MacBookPro"),price : 1199};
    let prod2 = Product{prod :String::from("Kindle"),price :179};
    let prod3 = Product{prod :String::from("Iphone"),price :499};
    let total = cart_sum(prod1,prod2,prod3);
    println!("{}",total);
}

fn main() {
    let start = Instant::now();
    thread::spawn(||{
        create_cart()
    });
    thread::sleep(Duration::from_millis(3));
    thread::spawn(||{
        create_cart()
    });
    thread::sleep(Duration::from_millis(3));
    thread::spawn(||{
        create_cart()
    });
    thread::sleep(Duration::from_millis(3));
    thread::spawn(||{
        create_cart()
    });
    thread::sleep(Duration::from_millis(3));
    thread::spawn(||{
        create_cart()
    });
    thread::sleep(Duration::from_millis(3));
    let duration1 = start.elapsed();
    println!("{:?}", duration1);
}