mod occu_core;
use occu_core::Event;

fn main() {
    println!("Hello, world!");
    let mut event = Event::new("Test".to_string(), "No Desc".to_string());
    event.occur("Occur 1".to_string(), "Blank".to_string());
    println!("{:?}", &event)
}
