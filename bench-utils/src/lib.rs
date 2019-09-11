#![allow(unused_imports)]
#[cfg(feature = "timer")]
extern crate colored;

pub use self::inner::*;

#[cfg(feature = "timer")]
#[macro_use]
pub mod inner {
    pub use colored::Colorize;
    use std::sync::atomic::AtomicUsize;
    pub static NUM_INDENT: AtomicUsize = AtomicUsize::new(0);
    pub const PAD_CHAR: &'static str = "·";
    use std::time::Instant;

    pub struct TimerInfo {
        pub msg: String,
        pub time: Instant,
    }

    #[macro_export]
    macro_rules! timer_start {
        ($msg:expr) => {{
            use $crate::{compute_indent, Colorize, NUM_INDENT, PAD_CHAR};
            use std::{sync::atomic::Ordering, time::Instant};

            let result = $msg();
            let start_info = "Start:".yellow().bold();
            let indent_amount = 2 * NUM_INDENT.fetch_add(0, Ordering::Relaxed);
            let indent = compute_indent(indent_amount);

            println!("{}{:8} {}", indent, start_info, result);
            NUM_INDENT.fetch_add(1, Ordering::Relaxed);
            $crate::TimerInfo { msg: result.to_string(), time: Instant::now() }
        }};
    }

    #[macro_export]
    macro_rules! timer_end {
        ($time:expr) => {{
            timer_end!($time, || "");
        }};
        ($time:expr, $msg:expr) => {{
            use $crate::{compute_indent, Colorize, NUM_INDENT, PAD_CHAR};
            use std::sync::atomic::Ordering;

            let time = $time.time;
            let final_time = time.elapsed();
            let final_time = {
                let secs = final_time.as_secs();
                let millis = final_time.subsec_millis();
                let micros = final_time.subsec_micros() % 1000;
                let nanos = final_time.subsec_nanos() % 1000;
                if secs != 0 {
                    format!("{}.{}s", secs, millis).bold()
                } else if millis > 0 {
                    format!("{}.{}ms", millis, micros).bold()
                } else if micros > 0 {
                    format!("{}.{}µs", micros, nanos).bold()
                } else {
                    format!("{}ns", final_time.subsec_nanos()).bold()
                }
            };

            let end_info = "End:".green().bold();
            let message = format!("{} {}", $time.msg, $msg());

            NUM_INDENT.fetch_sub(1, Ordering::Relaxed);
            let indent_amount = 2 * NUM_INDENT.fetch_add(0, Ordering::Relaxed);
            let indent = compute_indent(indent_amount);

            // Todo: Recursively ensure that *entire* string is of appropriate
            // width (not just message).
            println!(
                "{}{:8} {:.<pad$}{}",
                indent,
                end_info,
                message,
                final_time,
                pad = 75 - indent_amount
            );

        }};
        
    }

    impl Drop for TimerInfo {
        fn drop(&mut self) {
            timer_end!(self)
        }
    }


    pub fn compute_indent(indent_amount: usize) -> String {
        use std::env::var;
        let mut indent = String::new();
        let pad_string = match var("CLICOLOR") {
            Ok(val) => {
                if val == "0" {
                    " "
                } else {
                    PAD_CHAR
                }
            },
            Err(_) => PAD_CHAR,
        };
        for _ in 0..indent_amount {
            indent.push_str(&pad_string.white());
        }
        indent
    }
}

#[cfg(not(feature = "timer"))]
#[macro_use]
mod inner {
    pub struct TimerInfo;
    impl Drop for TimerInfo {
        fn drop(&mut self) {}
    }

    #[macro_export]
    macro_rules! timer_start {
        ($msg:expr) => {
            $crate::TimerInfo
        };
    }

    #[macro_export]
    macro_rules! timer_end {
        ($time:expr, $msg:expr) => {
            let _ = $msg;
            let _ = $time;
        };
        ($time:expr) => {
            let _ = $time;
        };
    }
}
