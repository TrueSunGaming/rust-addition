mod factory;

use factory::Factory;

fn main() {
    let mut factory = Factory::<u64>::new();

    factory.register("balls", 69);

    println!("{}", factory.generate("balls").unwrap());
}
