extern crate comcart;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let result = match args.get(1) {
        Some(path) => comcart::process(path),
        None => {
            println!("Must pass a path string");
            std::process::exit(1);
        }
    };
    match result {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e)
    };
}
