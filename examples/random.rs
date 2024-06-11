use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let choices = [1, 2, 4, 8, 16, 32];
    let mut rng = thread_rng();
    println!("{:?}", choices.choose(&mut rng));
    assert_eq!(choices[..0].choose(&mut rng), None);
}
