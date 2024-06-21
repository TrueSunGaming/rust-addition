mod factory;
mod signal;

use signal::Signal;

fn main() {
    let mut signal = Signal::<()>::new();

    signal.connect(&|_| println!("hi"), Some("callback"));
    signal.connect(&|_| println!("hi2"), Some("callback2"));
    signal.emit(&());

    signal.disconnect("callback");
    signal.emit(&());
    
    signal.disconnect("callback2");
    signal.emit(&());
}
