const games: &str = include_str!("../answers.txt");
fn main() {
    let guesser = todo!();
    for answer in games.split_whitespace() {
        play(answer, guesser);
    }
    println!("Hello, world!");
}

fn play<G: Guesser>(answer: &'static str, guesser: G) {}
enum Correctness {
    correct,
    misplaced,
    wrong,
}
struct Guess {
    word: String,
    mask: [Correctness; 5],
}

trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}
