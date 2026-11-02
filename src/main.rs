use chrono::{Local, Timelike};
use crossterm::terminal::{self, enable_raw_mode, disable_raw_mode, Clear, ClearType};
use crossterm::event::{self, Event, KeyCode};
use crossterm::cursor::{Hide, Show,MoveTo};
use crossterm::ExecutableCommand;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

const DIGITS: [[&str; 5]; 10] = [
    [
        " ■■■ ",
        "■   ■",
        "■   ■",
        "■   ■",
        " ■■■ ",
    ],
    [
        "  ■  ",
        " ■■  ",
        "  ■  ",
        "  ■  ",
        " ■■■ ",
    ],
    [
        " ■■■ ",
        "■   ■",
        "    ■",
        "  ■■ ",
        "■■■■■",
    ],
    [
        "■■■■ ",
        "    ■",
        " ■■■ ",
        "    ■",
        "■■■■ ",
    ],
    [
        "■   ■",
        "■   ■",
        "■■■■■",
        "    ■",
        "    ■",
    ],
    [
        "■■■■■",
        "■    ",
        "■■■■ ",
        "    ■",
        "■■■■ ",
    ],
    [
        " ■■■ ",
        "■    ",
        "■■■■ ",
        "■   ■",
        " ■■■ ",
    ],
    [
        "■■■■■",
        "    ■",
        "   ■ ",
        "  ■  ",
        " ■   ",
    ],
    [
        " ■■■ ",
        "■   ■",
        " ■■■ ",
        "■   ■",
        " ■■■ ",
    ],
    [
        " ■■■ ",
        "■   ■",
        " ■■■■",
        "    ■",
        " ■■■ ",
    ],
];


fn clock() {
    let mut last_update = Instant::now();
    loop {
        if last_update.elapsed() >= Duration::from_secs(1) {
            last_update = Instant::now();

            print!("\x1B[2J\x1B[H");
            let now = Local::now();
    
            let terminal_size = terminal::size().unwrap();
            let terminal_width = terminal_size.0 as usize;
            let terminal_height = terminal_size.1 as usize;
    
            let clock_width = 43;
            let clock_height = 5;
            
            let x = (terminal_width.saturating_sub(clock_width)) / 2;
            let y = (terminal_height.saturating_sub(clock_height)) / 2;
    
            let digits = [
                now.hour() / 10,
                now.hour() % 10,
                now.minute() / 10,
                now.minute() % 10,
                now.second() / 10,
                now.second() % 10,
            ];
    
            
        
            for row in 0..5 {
                stdout()
                    .execute(MoveTo(x as u16, (y + row) as u16))
                    .unwrap();
    
                for (i, digit) in digits.iter().enumerate() {
                    print!("{}", DIGITS[*digit as usize][row]);
    
                    if i < 5 {
                        if i == 1 || i == 3 {
                            if row == 1 || row == 3 {
                                print!("  ■  ");
                            } else {
                                print!("     ");
                            }
                        } else {
                            print!(" ");
                        }
                    }
                }
            }
    
            stdout().flush().unwrap();
        }

        if event::poll(Duration::from_millis(50)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }
}

fn main() {
    enable_raw_mode().unwrap();
    stdout().execute(Hide).unwrap();

    clock();

    disable_raw_mode().unwrap();
    stdout().execute(Show).unwrap();
    stdout().execute(Clear(ClearType::All)).unwrap();
}
