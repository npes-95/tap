extern crate termion;

mod convert;
mod tap;

use crate::convert::{Bpm, Notation};
use crate::tap::Tap;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn usage() {
    print!("[q]: quit | [c|r]: clear/reset tap count | [h]: this help")
}

fn main() -> anyhow::Result<()> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?;
    write!(
        stdout,
        "{}{}{}tap - bpm calculator",
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide,
        termion::clear::All
    )?;

    stdout.flush()?;

    let mut tap = Tap::new();

    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}{}",
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide,
            termion::clear::All
        )?;

        match c? {
            Key::Char('q') => break,
            Key::Char('h') => usage(),
            Key::Char('c') | Key::Char('r') => {
                tap.reset();
                print!("reset count");
                ()
            }
            _ => {
                if tap.count() < 1 {
                    tap.tap();
                    println!("count: {}", tap.count());
                } else {
                    tap.tap();

                    let pulse = tap.average_interval();
                    let bpm = Bpm::new(pulse);
                    let notation = Notation::new(pulse);

                    println!("count: {}\n", tap.count());
                    println!("\rbpm: {}\n", bpm);
                    println!("\r{}", notation);
                }

                ()
            }
        }

        stdout.flush()?;
    }

    write!(stdout, "{}", termion::cursor::Show)?;

    Ok(())
}
