use crate::db::Entry;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{self, Write};

const MAX_DISPLAY: usize = 10;

pub enum SelectResult {
    Selected(String),
    Cancelled,
}

pub fn select(entries: &[&Entry]) -> io::Result<SelectResult> {
    if entries.is_empty() {
        return Ok(SelectResult::Cancelled);
    }
    if entries.len() == 1 {
        return Ok(SelectResult::Selected(entries[0].path.clone()));
    }

    let display_count = entries.len().min(MAX_DISPLAY);
    let mut cursor_pos: usize = 0;

    let stderr = io::stderr();
    let mut out = stderr.lock();

    terminal::enable_raw_mode()?;
    execute!(out, cursor::Hide)?;

    let result = run_loop(&mut out, entries, display_count, &mut cursor_pos);

    execute!(out, cursor::Show)?;
    terminal::disable_raw_mode()?;

    result
}

fn run_loop(
    out: &mut impl Write,
    entries: &[&Entry],
    display_count: usize,
    cursor_pos: &mut usize,
) -> io::Result<SelectResult> {
    loop {
        render(out, entries, display_count, *cursor_pos)?;

        match event::read()? {
            Event::Key(key) => match (key.code, key.modifiers) {
                (KeyCode::Char('j'), _)
                | (KeyCode::Down, _)
                | (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                    if *cursor_pos + 1 < display_count {
                        *cursor_pos += 1;
                    }
                }
                (KeyCode::Char('k'), _)
                | (KeyCode::Up, _)
                | (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                    if *cursor_pos > 0 {
                        *cursor_pos -= 1;
                    }
                }
                (KeyCode::Char('g'), _) => {
                    *cursor_pos = 0;
                }
                (KeyCode::Char('G'), _) => {
                    *cursor_pos = display_count - 1;
                }
                (KeyCode::Enter, _) => {
                    clear_lines(out, display_count, entries.len())?;
                    return Ok(SelectResult::Selected(entries[*cursor_pos].path.clone()));
                }
                (KeyCode::Char('q'), _)
                | (KeyCode::Esc, _)
                | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                    clear_lines(out, display_count, entries.len())?;
                    return Ok(SelectResult::Cancelled);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

fn render(
    out: &mut impl Write,
    entries: &[&Entry],
    display_count: usize,
    cursor_pos: usize,
) -> io::Result<()> {
    // カーソルを描画開始位置に戻す（再描画）
    queue!(out, cursor::MoveToColumn(0))?;

    for i in 0..display_count {
        queue!(out, terminal::Clear(ClearType::CurrentLine))?;
        if i == cursor_pos {
            queue!(
                out,
                SetForegroundColor(Color::Cyan),
                Print(format!("> {}\r\n", entries[i].path)),
                ResetColor
            )?;
        } else {
            queue!(out, Print(format!("  {}\r\n", entries[i].path)))?;
        }
    }

    if entries.len() > MAX_DISPLAY {
        let rest = entries.len() - MAX_DISPLAY;
        queue!(
            out,
            terminal::Clear(ClearType::CurrentLine),
            SetForegroundColor(Color::DarkGrey),
            Print(format!("  ...他{}件\r\n", rest)),
            ResetColor
        )?;
    }

    // 描画した行数分だけ上に戻る
    let total_lines = display_count + if entries.len() > MAX_DISPLAY { 1 } else { 0 };
    queue!(out, cursor::MoveUp(total_lines as u16))?;

    out.flush()
}

fn clear_lines(out: &mut impl Write, display_count: usize, total: usize) -> io::Result<()> {
    let total_lines = display_count + if total > MAX_DISPLAY { 1 } else { 0 };
    for _ in 0..total_lines {
        queue!(
            out,
            terminal::Clear(ClearType::CurrentLine),
            cursor::MoveDown(1)
        )?;
    }
    queue!(out, cursor::MoveUp(total_lines as u16))?;
    out.flush()
}
