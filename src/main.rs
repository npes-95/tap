extern crate termion;

mod convert;
mod tap;

use crate::convert::{Bpm, Notation};
use crate::tap::Tap;
use std::io::{stdin, stdout, Write};
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn usage() {
    print!("[q]: quit | [c|r]: clear/reset tap count | [h]: this help")
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
            Key::Char('c') | Key::Char('r') => {
                tap.reset();
                print!("reset count");
                ()
            }
            _ => {
                if tap.last_interval() > reset_interval {
                    tap.reset();
                    print!("reset count");
                } else if tap.count() < 1 {
                    tap.tap();
                    println!("count: {}", tap.count());
                } else {
                    tap.tap();

                    let pulse = tap.average_interval().unwrap();
                    let bpm = Bpm::new(pulse);
                    let notation = Notation::new(pulse);

                    println!("count: {}", tap.count());
                    println!("\r");
                    println!("\rbpm: {:.2}", bpm.value);
                    println!("\r");
                    println!("\rquarter: {} ms", notation.quarter.as_millis());
                    println!(
                        "\rdotted quarter: {} ms",
                        notation.dotted_quarter.as_millis()
                    );
                    println!("\r");
                    println!("\reighth: {} ms", notation.eighth.as_millis());
                    println!("\rdotted eighth: {} ms", notation.dotted_eighth.as_millis());
                    println!("\r");
                    println!("\rsixteenth: {} ms", notation.sixteenth.as_millis());
                    println!(
                        "\rdotted sixteenth: {} ms",
                        notation.dotted_sixteenth.as_millis()
                    );
                    println!("\r");
                    println!("\rthirty-second: {} ms", notation.thirtysecond.as_millis());
                    println!(
                        "\rdotted thirty-second: {} ms",
                        notation.dotted_thirtysecond.as_millis()
                    );
                }

                ()
            }
        }

        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
