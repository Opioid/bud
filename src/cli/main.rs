extern crate bud;

use bud::base::random::generator::Generator;

fn main() {
    let mut rng = Generator::new(0, 0);

    rng.start(0, 0);

    for x in 0..10 {
        println!("Random number {}: {}", x, rng.random_float());
    }
}
