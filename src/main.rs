use std::{io::Write, time::Duration};

fn print_answer(answer: bool) {
    let answer = match answer {
        true => "yes",
        false => "no ",
    };

    print!(
        "{}{}",
        termion::cursor::Left(3),
        answer,
    );

    _ = std::io::stdout().flush();
}

fn main() {
    let mut result: bool = rand::random();
    let animation_duration = Duration::from_secs(4);
    let start = std::time::SystemTime::now();

    loop {
        let duration = start.elapsed().unwrap();
        if duration >= animation_duration {
            break;
        }

        result = !result;
        print_answer(result);

        let p = (duration.as_secs_f32() / animation_duration.as_secs_f32()).powf(2.0);
        std::thread::sleep(Duration::from_secs_f32(p));
    }
}
