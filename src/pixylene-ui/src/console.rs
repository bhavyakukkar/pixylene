use crate::{ LogType };

pub struct Console {
    pub cmdin: fn(String) -> Option<String>,
    pub cmdout: fn(String, LogType) -> (),
    //console_corner: Coord,
    //discard_key: event::KeyEvent,
}
/*
impl Console {
    /*
    pub fn new(console_corner: Coord) -> Console {
        Console { console_corner }
    }
    */
    pub fn cmdin(&self, message: &str) -> Option<String> {
        use terminal::{ Clear, ClearType };
        use cursor::{ MoveTo, MoveRight, MoveLeft, Show, Hide };
        use style::{ SetForegroundColor, Color, Print, ResetColor };
        use event::{ Event, KeyEvent, KeyCode, read };

        let mut out: Option<String> = None;

        execute!(
            std::io::stdout(),
            ResetColor,
            MoveTo(self.console_corner.y as u16, self.console_corner.x as u16),
            Clear(ClearType::UntilNewLine),
            SetForegroundColor(Color::Rgb{ r: 220, g: 220, b: 220 }),
            Print(&message),
            ResetColor,
            Show,
        ).unwrap();

        let mut input = String::new();
        loop {
            let event = read().unwrap();
            if let Event::Key(key) = event {
                if key == self.discard_key {
                    execute!(std::io::stdout(), Clear(ClearType::CurrentLine)).unwrap();
                    out = None;
                    break;
                }
                let KeyEvent { code, .. } = key;
                match code {
                    KeyCode::Enter => {
                        execute!(std::io::stdout(), Clear(ClearType::CurrentLine)).unwrap();
                        out = Some(input);
                        break;
                    },
                    KeyCode::Esc => {
                    },
                    KeyCode::Backspace => {
                        if input.len() > 0 {
                            execute!(std::io::stdout(), MoveLeft(1), Clear(ClearType::UntilNewLine)).unwrap();
                            input.pop();
                        }
                    },
                    KeyCode::Char(c) => {
                        execute!(std::io::stdout(), Print(c)).unwrap();
                        input.push(c);
                    },
                    _ => {},
                }
            }
        }
        execute!(std::io::stdout(), Hide).unwrap();
        out
    }
    pub fn cmdout(&self, message: &str, log_type: LogType) {
        use terminal::{ Clear, ClearType };
        use cursor::{ MoveTo };
        use style::{ SetForegroundColor, Color, Print, ResetColor };
        let corner = self.console_corner.add(Coord{ x: 0, y: 0 });
        execute!(
            std::io::stdout(),
            ResetColor,
            MoveTo(corner.y as u16, corner.x as u16),
            Clear(ClearType::UntilNewLine),
            SetForegroundColor(match log_type {
                LogType::Info => Color::Rgb{ r: 240, g: 240, b: 240 },
                LogType::Error => Color::Rgb{ r: 255, g: 70, b: 70 },
                LogType::Warning => Color::Rgb{ r: 70, g: 235, b: 235 },
                LogType::Success => Color::Rgb{ r: 70, g: 255, b: 70 },
            }),
            Print(&message),
            ResetColor,
        ).unwrap();
    }
}
*/
