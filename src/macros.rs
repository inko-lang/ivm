#![macro_use]

macro_rules! usage {
    ($options:expr, $summary:expr) => {{
        let out = $options.usage_with_format(|opts| {
            format!(
                "{}\n\nOptions:\n\n{}",
                $summary,
                opts.collect::<Vec<String>>().join("\n")
            )
        });

        println!("{}", out);
    }};
}

#[cfg(not(windows))]
macro_rules! info {
    ($message:expr $(,$arg:expr)*) => {
        eprintln!(
            "\x1b[1minfo:\x1b[0m {}",
            format!($message $(,$arg)*)
        )
    };
}

#[cfg(not(windows))]
macro_rules! error {
    ($message:expr $(,$arg:expr)*) => {
        eprintln!(
            "\x1b[1m\x1b[31merror:\x1b[0m\x1b[0m {}",
            format!($message $(,$arg)*)
        )
    };
}

#[cfg(windows)]
macro_rules! info {
    ($message:expr $(,$arg:expr)*) => {
        eprintln!("info: {}", format!($message $(,$arg)*))
    };
}

#[cfg(windows)]
macro_rules! error {
    ($message:expr $(,$arg:expr)*) => {
        eprintln!("error: {}", format!($message $(,$arg)*))
    };
}
