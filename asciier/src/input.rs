use crate::misc;

use std::{io,io::{Result, stdout, Write}};

use crossterm::{
    terminal::{Clear, ClearType},
    ExecutableCommand,
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{enable_raw_mode, disable_raw_mode},
    cursor::{MoveUp, MoveDown, MoveToColumn, MoveRight, SavePosition, RestorePosition, MoveToPreviousLine},
    style::Print,
    execute,
};

pub fn warning(msg: Option<&'static str>) {
    let mut so = stdout();

    let d = if let Some(s) = msg {
        println!("{}", s);
        s.trim().len() as u64 * 70
    } else {
        println!("Invalid Input");
        910
    };

    misc::delay_millis(d);

    so.execute(MoveToPreviousLine(1)).unwrap();
    so.execute(Clear(ClearType::CurrentLine)).unwrap();
    so.execute(MoveToPreviousLine(1)).unwrap();
    so.execute(Clear(ClearType::CurrentLine)).unwrap();
    so.flush().unwrap();
}

pub fn enter_u16(min:u16, max:u16, empty_allowed: bool) -> u16{
    if min == 0{
        panic!("minimum argument cannot be less than one");
    }

////

    let mut input_buf = String::new();

    loop{
        input_buf.clear();

        io::stdin().read_line(&mut input_buf).unwrap();

        if input_buf.trim().is_empty() && empty_allowed{
            return 0
        }
        let _: u16 = match input_buf.trim().parse(){
            Ok(i) => {
                if i < min{
                    warning(Some("Entered value is too small"));
                    continue;
                }
                if i > max{
                    warning(Some("Entered value is to large"));
                    continue;
                }

                return i
            },
            Err(_) => {
                warning(None);
                continue;
            }
        };
    }
}

pub fn enter_str(min_len: usize, max_len: Option<usize>, empty_allowed: bool) -> Option<String> {
    let mut input_buf = String::new();

    loop {
        input_buf.clear();
        io::stdin().read_line(&mut input_buf).unwrap();

        let trimmed = input_buf.trim();

        if trimmed.is_empty() && empty_allowed {
            return None;
        }

        if trimmed.len() < min_len {
            warning(Some("Entered string is too small"));
            continue;
        }

        if let Some(ml) = max_len {
            if trimmed.len() > ml {
                warning(Some("Entered string is too large"));
                continue;
            }
        }

        return Some(trimmed.to_string());
    }
}

pub fn choose_string(strings: &[&str]) -> Result<usize> {
    if strings.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "String list cannot be empty",
        ));
    }

    let mut so = stdout();
    let ipl = strings.len();
    let mut ci = 0usize;

    println!();

    for s in strings {
        println!("[ ] {}", s);
    }

    execute!(
        so,
        SavePosition
    )?;

    execute!(
        so,
        MoveUp(ipl as u16),
        MoveToColumn(1),
        Print("X")
    )?;

    enable_raw_mode()?;

    let result = (|| -> std::io::Result<usize> {
        loop {
            match event::read()? {

                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    let next = match key.code {
                        KeyCode::Up | KeyCode::Char('w' | 'W') | KeyCode::Left | KeyCode::Char('a' | 'A') => {
                            Some(if ci == 0 { ipl - 1 } else { ci - 1 })
                        }

                        KeyCode::Down | KeyCode::Char('s' | 'S') | KeyCode::Right | KeyCode::Char('d' | 'D') => {
                            Some((ci + 1) % ipl)
                        }

                        KeyCode::Enter | KeyCode::Char(' ') => {
                            execute!(
                                so,
                                RestorePosition,
                            )?;
                            return Ok(ci)
                        }

                        KeyCode::Char('r' | 'R') => {
                            execute!(
                                so,
                                RestorePosition,
                            )?;
                            return Ok(usize::MAX)
                        }

                        _ => None,
                    };

                    if let Some(next) = next {
                        if next > ci {
                            execute!(
                                so,
                                MoveToColumn(1),
                                Print(" "),
                                MoveDown((next - ci) as u16),
                                MoveToColumn(1),
                                Print("X")
                            )?;
                        } else if next < ci {
                            execute!(
                                so,
                                MoveToColumn(1),
                                Print(" "),
                                MoveUp((ci - next) as u16),
                                MoveToColumn(1),
                                Print("X")
                            )?;
                        }

                        ci = next;
                    }
                }

                _ => {}
            }
        }
    })();

    disable_raw_mode()?;

    so.flush()?;

    result
}