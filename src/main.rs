mod state;

use state::State;
use std::io;
use std::io::BufRead;
use std::io::Write;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let state = State::new();

    let result = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok().and_then(|line| state.next(line)))
        .try_for_each(|line| writeln!(stdout, "{}", line));

    match result {
        Ok(_) => {}
        Err(e) if e.kind() == io::ErrorKind::BrokenPipe => {}
        Err(e) => {
            eprintln!("{:?}", e);
            std::process::exit(1);
        }
    }
}
