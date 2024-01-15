use super::time::get_formatted_time;

pub fn log(lv: LogLevel, str: String, enable_debug: bool, fn_line_col_lctime:(&str, u32, u32, bool)) {
    if lv == LogLevel::Debug && !enable_debug {
        return;
    }
    let time = get_formatted_time(fn_line_col_lctime.3);

    if lv == LogLevel::Debug {
        match time {
            Ok(a) => put_text(format!("[{a}] [{lv}] [{}] [line:{}, column:{}] {str}", fn_line_col_lctime.0, fn_line_col_lctime.1, fn_line_col_lctime.2)),
            Err(_) => put_text(format!("[VOIDTIME] [{lv}] [{}] [line:{}, column:{}] {str}", fn_line_col_lctime.0, fn_line_col_lctime.1, fn_line_col_lctime.2)),
        }
    } else {
        match time {
            Ok(a) => put_text(format!("[{a}] [{lv}] {str}")),
            Err(_) => put_text(format!("[VOIDTIME] [{lv}] {str}")),
        }
    }
    
}

fn put_text(str: String) {
    println!("{str}");
}

macro_rules! enum_autoreflex {
    ($(#[$meta:meta])*$vis:vis $n:ident, $($e:ident),*, $($f:literal),*) => {
        $(#[$meta])*
        $vis enum $n {
            $($e,)*
        }
        impl std::fmt::Display for $n {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", {
                    match self {
                        $($n::$e =>$f,)*
                    }
                })
            }
        }
    };
}

enum_autoreflex! {
    #[derive(PartialEq)] pub LogLevel,
         Info,   Warn,   Error,    Fatal,   Debug,
        "INFO", "WARN", "ERROR",  "FATAL", "DEBUG"
}