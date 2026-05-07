use crate::db::Entry;
use crate::matcher;
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

pub fn select(all_entries: &[&Entry], initial_query: &str) -> io::Result<SelectResult> {
    let query = initial_query.to_string();
    let filtered = apply_filter(all_entries, &query);

    if filtered.is_empty() {
        return Ok(SelectResult::Cancelled);
    }
    if filtered.len() == 1 {
        return Ok(SelectResult::Selected(filtered[0].path.clone()));
    }

    let stderr = io::stderr();
    let mut out = stderr.lock();
    terminal::enable_raw_mode()?;
    execute!(out, cursor::Hide)?;

    let result = run_loop(&mut out, all_entries, query, filtered);

    execute!(out, cursor::Show)?;
    terminal::disable_raw_mode()?;

    result
}

fn apply_filter<'a>(entries: &[&'a Entry], query: &str) -> Vec<&'a Entry> {
    let keywords: Vec<&str> = query.split_whitespace().collect();
    matcher::filter(entries, &keywords)
}

fn total_lines(display_count: usize, total_filtered: usize) -> usize {
    // prompt line + entries + optional "他N件" line
    1 + display_count + if total_filtered > MAX_DISPLAY { 1 } else { 0 }
}

fn run_loop<'a>(
    out: &mut impl Write,
    all_entries: &[&'a Entry],
    mut query: String,
    mut filtered: Vec<&'a Entry>,
) -> io::Result<SelectResult> {
    let mut cursor_pos: usize = 0;
    let mut last_lines: usize = 0;

    loop {
        let display_count = filtered.len().min(MAX_DISPLAY);
        let cur_lines = total_lines(display_count, filtered.len());

        clear_area(out, last_lines)?;
        render(out, &filtered, display_count, cursor_pos, &query)?;
        last_lines = cur_lines;

        match event::read()? {
            Event::Key(key) => match (key.code, key.modifiers) {
                (KeyCode::Char('j'), KeyModifiers::NONE)
                | (KeyCode::Down, _)
                | (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                    if cursor_pos + 1 < display_count {
                        cursor_pos += 1;
                    }
                }
                (KeyCode::Char('k'), KeyModifiers::NONE)
                | (KeyCode::Up, _)
                | (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                    if cursor_pos > 0 {
                        cursor_pos -= 1;
                    }
                }
                (KeyCode::Char('g'), KeyModifiers::NONE) => {
                    cursor_pos = 0;
                }
                (KeyCode::Char('G'), _) => {
                    cursor_pos = display_count.saturating_sub(1);
                }
                (KeyCode::Enter, _) => {
                    clear_area(out, cur_lines)?;
                    return if display_count > 0 {
                        Ok(SelectResult::Selected(filtered[cursor_pos].path.clone()))
                    } else {
                        Ok(SelectResult::Cancelled)
                    };
                }
                (KeyCode::Char('q'), KeyModifiers::NONE)
                | (KeyCode::Esc, _)
                | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                    clear_area(out, cur_lines)?;
                    return Ok(SelectResult::Cancelled);
                }
                (KeyCode::Backspace, _) => {
                    query.pop();
                    filtered = apply_filter(all_entries, &query);
                    let max = filtered.len().min(MAX_DISPLAY).saturating_sub(1);
                    cursor_pos = cursor_pos.min(max);
                }
                (KeyCode::Char(c), mods)
                    if mods == KeyModifiers::NONE || mods == KeyModifiers::SHIFT =>
                {
                    query.push(c);
                    filtered = apply_filter(all_entries, &query);
                    cursor_pos = 0;
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
    query: &str,
) -> io::Result<()> {
    queue!(
        out,
        terminal::Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::Green),
        Print(format!("> {}\r\n", query)),
        ResetColor
    )?;

    for i in 0..display_count {
        queue!(out, terminal::Clear(ClearType::CurrentLine))?;
        if i == cursor_pos {
            queue!(
                out,
                SetForegroundColor(Color::Cyan),
                Print(format!("  > {}\r\n", entries[i].path)),
                ResetColor
            )?;
        } else {
            queue!(out, Print(format!("    {}\r\n", entries[i].path)))?;
        }
    }

    if entries.len() > MAX_DISPLAY {
        let rest = entries.len() - MAX_DISPLAY;
        queue!(
            out,
            terminal::Clear(ClearType::CurrentLine),
            SetForegroundColor(Color::DarkGrey),
            Print(format!("    ...他{}件\r\n", rest)),
            ResetColor
        )?;
    }

    let lines = total_lines(display_count, entries.len());
    queue!(out, cursor::MoveUp(lines as u16))?;

    out.flush()
}

fn clear_area(out: &mut impl Write, lines: usize) -> io::Result<()> {
    for _ in 0..lines {
        queue!(
            out,
            terminal::Clear(ClearType::CurrentLine),
            cursor::MoveDown(1)
        )?;
    }
    if lines > 0 {
        queue!(out, cursor::MoveUp(lines as u16))?;
    }
    out.flush()
}
