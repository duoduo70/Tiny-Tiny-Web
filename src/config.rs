/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */
use crate::drop::{http::HttpResponse, log::LogLevel::*};
use crate::marco::*;
use std::sync::atomic::AtomicU32;
use std::sync::{Arc, Mutex};
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};

pub static USE_LOCALTIME: AtomicBool = AtomicBool::new(true);
pub static ENABLE_DEBUG: AtomicBool = AtomicBool::new(true);
pub static THREADS_NUM: AtomicU32 = AtomicU32::new(2);
pub static XRPS_COUNTER_CACHE_SIZE: AtomicU32 = AtomicU32::new(8);
pub static BOX_NUM_PER_THREAD_MAG: AtomicU32 = AtomicU32::new(1000);
pub static BOX_NUM_PER_THREAD_INIT_MAG: AtomicU32 = AtomicU32::new(1000);
pub static XRPS_PREDICT_MAG: AtomicU32 = AtomicU32::new(1100);
pub static mut GLOBAL_CONFIG: Option<Arc<Mutex<Config>>> = None;
#[derive(Clone)]
pub struct ServeFilesCustomExtra {
    pub content_type: String,
    pub replace: Option<Vec<(String, (usize, usize))>>,
}
#[derive(Clone)]
pub struct Config {
    pub use_localtime: bool,
    pub enable_debug: bool,
    pub addr_bind: Vec<String>,
    pub serve_files_custom: HashMap<String, (String, Option<ServeFilesCustomExtra>)>,
    pub response_404: Option<HttpResponse>,
}
impl Config {
    pub fn new() -> Self {
        Config {
            use_localtime: true,
            enable_debug: false,
            addr_bind: vec![],
            serve_files_custom: HashMap::new(),
            response_404: None,
        }
    }
    pub fn sync_static_vars(&self) {
        USE_LOCALTIME.store(self.use_localtime, Ordering::Relaxed);
        ENABLE_DEBUG.store(self.enable_debug, Ordering::Relaxed);
        unsafe { GLOBAL_CONFIG = Some(Arc::new(Mutex::new(self.clone()))) };
    }
    pub fn check(&self) {
        if self.serve_files_custom.is_empty() {
            log!(Warn, LOG[13]);
        }
    }
}

pub fn read_config() -> Result<Config, ()> {
    let lines = if let Ok(lines) = read_lines("config/main.gc") {
        lines
    } else {
        log!(Error, format!("{}{}", LOG[9], "config/main.gc"));
        return Err(());
    };
    let mut config = Config::new();

    let mut line_number = 1;
    for line in lines {
        match line {
            Ok(str) => parse_line(str, &mut config, "config/main.gc", line_number),
            Err(_) => log!(
                Error,
                format!(
                    "{}{}{} {}{}",
                    LOG[10], LOG[11], "config/main.gc", LOG[12], line_number
                )
            ),
        }
        line_number += 1;
    }

    Ok(config)
}
struct MethodArgs<'a> {
    config: &'a mut Config,
    line_splitted: &'a mut std::str::Split<'a, &'a str>,
    file: &'a str,
    line_number: i32,
}

fn method_add(args: MethodArgs) {
    if let Some(head2) = args.line_splitted.next() {
        if let Some(head3) = args.line_splitted.next() {
            method_add_head3_ext(args, head2, head3);
            return;
        }
        if !Path::new(&("export/".to_owned() + head2)).is_file() {
            syntax_error(args.file, args.line_number, LOG[20]);
            return;
        }
        args.config.serve_files_custom.insert(
            "/".to_owned() + &head2.to_string(),
            ("/".to_owned() + head2, None),
        );
        return;
    } else {
        syntax_error(args.file, args.line_number, LOG[18]);
        return;
    }
}
fn method_add_head3_ext(args: MethodArgs, head2: &str, head3: &str) {
    args.config.serve_files_custom.insert(
        "/".to_owned() + {
            if head3 == "/" {
                ""
            } else {
                head3
            }
        },
        (
            "/".to_owned() + head2,
            Some(ServeFilesCustomExtra {
                content_type: if let Some(head4) = args.line_splitted.next() {
                    head4.to_string()
                } else {
                    "text/html; charset=utf-8".to_string()
                },
                replace: None,
            }),
        ),
    );
}
fn method_remove(args: MethodArgs) {
    if let Some(head2) = args.line_splitted.next() {
        method_remove_head3_ext(args, head2);
    } else {
        syntax_error(args.file, args.line_number, LOG[18]);
        return;
    }
}
fn method_remove_head3_ext(args: MethodArgs, head2: &str) {
    if head2 == "/" {
        if let Some(_) = args.config.serve_files_custom.remove("/") {
        } else {
            syntax_error(args.file, args.line_number, LOG[19]);
        }
    } else {
        if let Some(_) = args
            .config
            .serve_files_custom
            .remove(&("/".to_owned() + &head2.to_string()))
        {
        } else {
            syntax_error(args.file, args.line_number, LOG[19]);
        }
    };
}
fn method_set(args: MethodArgs) {
    if let Some(head2) = args.line_splitted.next() {
        if let Some(head3) = args.line_splitted.next() {
            if head2 == "localtime" {
                pas_bool_option(
                    &mut args.config.use_localtime,
                    head3,
                    args.file,
                    args.line_number,
                );
                return;
            } else if head2 == "debug" {
                pas_bool_option(
                    &mut args.config.enable_debug,
                    head3,
                    args.file,
                    args.line_number,
                );
                return;
            } else if head2 == "404page" {
                page_404_option(args, head3);
                return;
            } else if head2 == "+addr" {
                args.config.addr_bind.push(head3.to_owned());
                return;
            } else if head2 == "threads" {
                THREADS_NUM.store(
                    if let Ok(a) = head3.parse() {
                        a
                    } else {
                        syntax_error(
                            args.file,
                            args.line_number,
                            &format!("{}{}", LOG[17], head3),
                        );
                        THREADS_NUM.load(Ordering::Relaxed)
                    },
                    Ordering::Relaxed,
                );
                return;
            } else if head2 == "xrps-counter-cache-size" {
                XRPS_COUNTER_CACHE_SIZE.store(
                    if let Ok(a) = head3.parse() {
                        a
                    } else {
                        syntax_error(
                            args.file,
                            args.line_number,
                            &format!("{}{}", LOG[17], head3),
                        );
                        XRPS_COUNTER_CACHE_SIZE.load(Ordering::Relaxed)
                    },
                    Ordering::Relaxed,
                );
                return;
            } else if head2 == "box-num-per-thread-mag" {
                BOX_NUM_PER_THREAD_MAG.store(
                    if let Ok(a) = head3.parse::<f32>() {
                        (a * 1000.0) as u32
                    } else {
                        syntax_error(
                            args.file,
                            args.line_number,
                            &format!("{}{}", LOG[17], head3),
                        );
                        BOX_NUM_PER_THREAD_MAG.load(Ordering::Relaxed)
                    },
                    Ordering::Relaxed,
                );
                return;
            } else if head2 == "box-num-per-thread-init-mag" {
                BOX_NUM_PER_THREAD_INIT_MAG.store(
                    if let Ok(a) = head3.parse::<f32>() {
                        (a * 1000.0) as u32
                    } else {
                        syntax_error(
                            args.file,
                            args.line_number,
                            &format!("{}{}", LOG[17], head3),
                        );
                        BOX_NUM_PER_THREAD_INIT_MAG.load(Ordering::Relaxed)
                    },
                    Ordering::Relaxed,
                );
                return;
            } else if head2 == "xrps-predict-mag" {
                XRPS_PREDICT_MAG.store(
                    if let Ok(a) = head3.parse::<f32>() {
                        (a * 1000.0) as u32
                    } else {
                        syntax_error(
                            args.file,
                            args.line_number,
                            &format!("{}{}", LOG[17], head3),
                        );
                        XRPS_PREDICT_MAG.load(Ordering::Relaxed)
                    },
                    Ordering::Relaxed,
                );
                return;
            } else {
                syntax_error(
                    args.file,
                    args.line_number,
                    &format!("{}{}", LOG[17], head3),
                );
            }
            return;
        }
        syntax_error(args.file, args.line_number, LOG[18]);
        return;
    } else {
        syntax_error(args.file, args.line_number, LOG[18]);
        return;
    }
}
fn page_404_option(args: MethodArgs, head3: &str) {
    let mut res = HttpResponse::new();
    res.set_content(
        if let Ok(a) = std::fs::read_to_string("export/".to_owned() + head3) {
            a
        } else {
            log!(Error, format!("{}{}", LOG[22], head3));
            return;
        },
    );
    args.config.response_404 = Some(res);
}
fn pas_bool_option(option: &mut bool, opt_str: &str, file: &str, line_number: i32) {
    if opt_str == "yes" || opt_str == "auto" {
        *option = true;
    } else if opt_str == "no" {
        *option = false;
    } else {
        syntax_error(file, line_number, &format!("{}{}", LOG[17], opt_str));
    }
}
fn method_compile(args: MethodArgs) {
    if let Some(head2) = args.line_splitted.next() {
        compile(args, head2);
    } else {
        syntax_error(args.file, args.line_number, LOG[18]);
    }
}
fn compile(args: MethodArgs, head2: &str) {
    let lines = match read_lines("export/".to_owned() + head2) {
        Err(_) => {
            syntax_error(args.file, args.line_number, LOG[20]);
            return;
        }
        Ok(a) => a,
    };

    let mut flags: Vec<(usize, usize)> = vec![];

    let mut linenumber = 1;
    for l in lines {
        match l {
            Err(_) => {
                syntax_error(
                    args.file,
                    args.line_number,
                    &format!("{}{}", LOG[22], "export/".to_owned() + head2),
                );
                return;
            }
            Ok(_) => (),
        }

        if let Some(pos) = l.unwrap().find("$_gcflag") {
            flags.push((linenumber, pos));
        }
        linenumber += 1;
    }

    if flags.is_empty() {
        return;
    }

    match std::fs::write(
        "temp/".to_owned() + head2,
        flags
            .into_iter()
            .map(|x| x.0.to_string() + " " + &x.1.to_string())
            .collect::<Vec<_>>()
            .join("\n")
            .as_bytes(),
    ) {
        Ok(_) => {
            log!(Debug, format!("{}{}", LOG[24], "temp/".to_owned() + head2));
        }
        Err(_) => {
            syntax_error(
                args.file,
                args.line_number,
                &format!("{}{}", LOG[23], "temp/".to_owned() + head2),
            );
            return;
        }
    }
}
fn method_import(mut args: MethodArgs) {
    if method_import_haserr(&mut args) == Err(()) {
        syntax_error(args.file, args.line_number, LOG[25]);
        return;
    }
}
fn method_import_haserr(args: &mut MethodArgs) -> Result<(), ()> {
    let pathname = if let Some(a) = args.line_splitted.next() {
        a
    } else {
        return Err(());
    };
    let conf_serve_value = if let Some(a) = args
        .config
        .serve_files_custom
        .get_mut(&("/".to_owned() + pathname))
    {
        a
    } else {
        return Err(());
    };
    let filename = &conf_serve_value.0;
    if !Path::new(&("temp/".to_string() + &filename)).is_file() {
        return Err(());
    }

    let lines = if let Ok(a) = read_lines("temp/".to_owned() + &filename) {
        a
    } else {
        return Err(());
    };
    let mut linenumbers: Vec<(usize, usize)> = vec![];
    for e in lines {
        if let Ok(line) = e {
            linenumbers.push(match line.split_once(' ') {
                Some((a, b)) => match (a.parse(), b.parse()) {
                    (Ok(a), Ok(b)) => (a, b),
                    _ => return Err(()),
                },
                None => return Err(()),
            });
        } else {
            return Err(());
        };
    }

    let mut linenumber = 0;
    loop {
        if let Some(a) = &mut conf_serve_value.1 {
            if let Some(ori_tur) = &mut a.replace {
                ori_tur.push((
                    if let Some(f) = args.line_splitted.next() {
                        match std::fs::read_to_string("export/".to_owned() + f) {
                            Ok(a) => a,
                            _ => return Err(()),
                        }
                    } else {
                        return Err(());
                    },
                    if let Some(f) = linenumbers.get(linenumber) {
                        *f
                    } else {
                        return Err(());
                    },
                ));
            } else {
                a.replace = Some(vec![(
                    if let Some(f) = args.line_splitted.next() {
                        match std::fs::read_to_string("export/".to_owned() + f) {
                            Ok(a) => a,
                            _ => return Err(()),
                        }
                    } else {
                        return Err(());
                    },
                    if let Some(f) = linenumbers.get(linenumber) {
                        *f
                    } else {
                        return Err(());
                    },
                )]);
            }
        } else {
            return Err(());
        }
        linenumber += 1;
        if linenumber == linenumbers.len() {
            break;
        }
    }

    Ok(())
}
fn parse_line(line: String, config: &mut Config, file: &str, line_number: i32) {
    let mut line_splitted = line.split(" ");
    if let Some(head) = line_splitted.next() {
        if head == "+" {
            method_add(MethodArgs {
                config,
                line_splitted: &mut line_splitted,
                file,
                line_number,
            });
            return;
        }
        if head == "-" {
            method_remove(MethodArgs {
                config,
                line_splitted: &mut line_splitted,
                file,
                line_number,
            });
            return;
        }
        if head == "$" {
            method_set(MethodArgs {
                config,
                line_splitted: &mut line_splitted,
                file,
                line_number,
            });
            return;
        }
        if head == "#" {
            return;
        }
        if head == "compile" {
            method_compile(MethodArgs {
                config,
                line_splitted: &mut line_splitted,
                file,
                line_number,
            });
            return;
        }
        if head == "inject" {
            method_import(MethodArgs {
                config,
                line_splitted: &mut line_splitted,
                file,
                line_number,
            });
            return;
        }
    }

    if line.trim() != "" {
        syntax_error(file, line_number, LOG[16]);
    }
}

fn syntax_error(file: &str, line_number: i32, error: &str) {
    log!(
        Error,
        format!(
            "[{}] [{}\"{}\", {}{}] {}{}",
            LOG[21], LOG[11], file, LOG[12], line_number, LOG[10], error
        )
    );
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

crate::marco::create_static_string_list!(
    LOG,
    "Tiny-Tiny-Web Started (Ver.",
    "Can not listen: ",
    "Can not open TCP Steam.",
    "Connection established: \n",
    "Connection handle: Can not read a buffer, was skipped.",
    "Malformed or unsupported request: ",
    "Connection handle: Can not write a buffer to the stream, was skipped.",
    "Connection request header: \n",
    "Connection response: \n",
    "Can not read configure file: ",
    "Can not parse configure: Syntax Error: ", //10
    "file:",
    "line:",
    "No routes are set.",
    "Router: Pushed a file: ",
    "Loading configure finished. ",
    "Void command. ",
    "Void item: ", //17
    "Not enough items. ",
    "This mapping does not exist. ",
    "This is not a file. ",
    "config-loader",
    "Can not read file: ", //22
    "Can not write to ",
    "Compiled successfully: ",
    "Can not import.", //25
    "Can not select TCP listener to non-blocking mode",
    "Can not read a TCP Stream, the server may be about to crash."
);
