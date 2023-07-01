use std::{io::Write, time::Duration};

use clap::{arg, command, Parser};
use rand::Rng;

const SIMPLE_ANSWERS: &'static [&'static str] = &["yes.", "no."];

const OG_ANSWERS: &'static [&'static str] = &[
    // affirmitve
    "It is certain.",
    "It is decidedly so.",
    "Yes definetly.",
    "You mey rely on it.",
    "As I see it, yes.",
    "Most likely.",
    "Outlook good.",
    "Yes.",
    "Signs point to yes.",
    // non-commital
    "Reply hazy, try again.",
    "Ask again later.",
    "Better not tell you now.",
    "Cannot predict now.",
    "Concentrate and ask again.",
    // negative
    "Don't count on it.",
    "My reply is no.",
    "My sources say no.",
    "Outlook not so good.",
    "Very doubtful.",
];

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Print either yes or no
    #[arg(short, long)]
    pub simple: bool,

    /// Print result immediatly
    #[arg(short, long)]
    pub no_animations: bool,
}

fn main() {
    let cli = Cli::parse();

    let answers = match &cli.simple {
        true => SIMPLE_ANSWERS,
        false => OG_ANSWERS,
    };

    let mut idx = rand::thread_rng().gen_range(0..answers.len());
    if cli.no_animations {
        println!("{}", answers[idx]);
        return;
    }

    let animation_duration = Duration::from_secs(4);
    let start = std::time::SystemTime::now();

    let _hide_cursor = termion::cursor::HideCursor::from(std::io::stdout());

    loop {
        let duration = start.elapsed().unwrap();
        if duration >= animation_duration {
            break;
        }

        idx += 1;
        if idx >= answers.len() {
            idx = 0;
        }

        print!(
            "{}{}{}",
            termion::cursor::Left(99),
            termion::clear::CurrentLine,
            answers[idx]
        );
        _ = std::io::stdout().flush();

        let p = (duration.as_secs_f32() / animation_duration.as_secs_f32()).powf(2.0);
        std::thread::sleep(Duration::from_secs_f32(p));
    }
}
