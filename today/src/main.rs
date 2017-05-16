extern crate time;

fn main() {
    let tm = time::now();
    println!("{}",
             tm.strftime("%Y-%m-%d").expect("strftime must succeed"));
}
