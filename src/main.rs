extern crate termion;

mod tap;

use crate::tap::Tap;
use std::io::{stdin, stdout, Write};
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn usage() {
    print!("[q]: quit | [c]: clear tap count | [h]: this help")
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "{}{}{}tap - bpm calculator",
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide,
        termion::clear::All
    )
    .unwrap();

    stdout.flush().unwrap();

    let mut tap = Tap::new();

    let reset_interval = Duration::from_secs(2);

    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}{}",
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide,
            termion::clear::All
        )
        .unwrap();

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('h') => usage(),
            Key::Char('c') => {
                tap.reset();
                print!("cleared count");
                ()
            }
            _ => {
                if tap.last_interval() > reset_interval {
                    tap.reset();
                    print!("cleared count");
                } else if tap.count() < 1 {
                    tap.tap();
                    print!("count: {} -- bpm: n/a", tap.count());
                } else {
                    tap.tap();
                    print!("count: {} -- bpm: {:.2}", tap.count(), tap.bpm().unwrap());
                }

                ()
            }
        }

        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
