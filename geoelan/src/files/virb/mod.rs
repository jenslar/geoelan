//! Filtering FIT data on recording session.

use std::io::Write;
use fit_rs::{FitError, Fit, FitSession, FitSessions};

/// Select session from those present in FIT-file
/// by returning UUID for first clip in session
// pub fn select_session(fitfile: &Fit) -> std::io::Result<String> {
// pub fn select_session(fitfile: &Fit) -> Result<String, FitError> {
pub fn select_session(fit: &Fit) -> Result<FitSession, FitError> {
    // let sessions = fit.sessions()?;
    let sessions = FitSessions::from_fit(fit)?;
    if sessions.is_empty() {
        return Err(FitError::NoSuchSession)
    }

    println!(" Session | Clips | First UUID in session");
    println!(".......................{}", ".".repeat(100));

    for (i, session) in sessions.iter().enumerate() {
        print!(
            " {:2}.     | {:2}    ",
            i + 1,
            session.len(),
        );
        for (i, u) in session.iter().enumerate() {
            let prefix = if i == 0 {
                "".to_owned()
            } else {
                format!("         |{}", " ".repeat(7))
            };
            println!("{prefix}| {u}");
        }
    }

    println!(".......................{}", ".".repeat(100));

    loop {
        print!("Select session: ");
        std::io::stdout().flush()?;
        let mut select = String::new();
        std::io::stdin().read_line(&mut select)?;
        let num = match select.trim().parse::<usize>() {
            Ok(n) => n - 1,
            Err(_) => {
                println!("Not a number");
                continue;
            }
        };
        match sessions.sessions().get(num) {
            Some(s) => {
                return Ok(s.to_owned())
            },
            None => {
                println!("No such item");
                continue;
            }
        }
    }
}