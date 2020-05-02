mod state;

use state::State;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();

    stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok().map(State::next))
        .for_each(|line| println!("{}", line));
}
