use remouillage::database::*;
use remouillage::canvas::*;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: {} <path_to_database>", args[0]);
        return;
    }

    let mut ctx = dashi::Context::new(&Default::default()).unwrap();
    let database = Database::new(&args[1]).unwrap();
    let canvas = Canvas::from_json(&mut ctx, &format!("{}/canvas.json", &args[1]));
}
