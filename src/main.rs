mod state;

use state::State;
use std::io;
use std::io::BufRead;
use std::io::Write;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok().and_then(State::next))
        .try_for_each(|line| writeln!(stdout, "{}", line))
        .unwrap();
}
