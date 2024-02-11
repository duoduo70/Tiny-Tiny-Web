/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

use crate::config::*;
use crate::drop::tool::ShouldResult;
use std::fs::read_to_string;
use std::path::Path;
use std::process::exit;
use self::vars::method_set;

pub struct MethodArgs<'a> {
    pub config: &'a mut Config,
    pub line_splitted: &'a mut std::str::Split<'a, &'a str>,
    pub file: &'a str,
    pub line_number: i32,
}

pub fn parse_line(line: String, config: &mut Config, file: &str, line_number: i32) {
    #[allow(clippy::single_char_pattern)]
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
            method_inject(MethodArgs {
                config,
                line_splitted: &mut line_splitted,
                file,
                line_number,
            });
            return;
        }
        if head == "@" {
            method_import(MethodArgs {
                config,
                line_splitted: &mut line_splitted,
                file,
                line_number,
            });
            return;
        }
        #[cfg(not(feature = "no-glisp"))]
        if head == "@gl" {
            method_import_gl(MethodArgs {
                config,
                line_splitted: &mut line_splitted,
                file,
                line_number,
            });
            return;
        }
        #[cfg(not(feature = "no-glisp"))]
        if head == "@pipe" {
            method_import_pipe(MethodArgs {
                config,
                line_splitted: &mut line_splitted,
                file,
                line_number,
            });
            return;
        }
        if head == ">" {
            method_log(MethodArgs {
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

pub fn syntax_error(file: &str, line_number: i32, error: &str) {
    log!(
        Error,
        format!(
            "[{}] [{}\"{}\", {}{}] {}{}",
            LOG[21], LOG[11], file, LOG[12], line_number, LOG[10], error
        )
    );
}

pub fn read_lines<P>(
    filename: P,
) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(std::io::BufRead::lines(std::io::BufReader::new(file)))
}

fn method_import(args: MethodArgs) -> &mut Config {
    if let Some(head2) = args.line_splitted.next() {
        read_config(head2.to_owned(), args.config).result_shldfatal(-1, || {})
    } else {
        log!(Fatal, LOG[18]);
        exit(-1);
    }
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
        args.config.router_config.serve_files_info.insert(
            "/".to_owned() + head2,
            ServeFileData::from("/".to_owned() + head2, args.config),
        );
    } else {
        syntax_error(args.file, args.line_number, LOG[18]);
    }
}
fn method_add_head3_ext(args: MethodArgs, head2: &str, head3: &str) {
    args.config.router_config.serve_files_info.insert(
        "/".to_owned() + {
            if head3 == "/" {
                ""
            } else {
                head3
            }
        },
        ServeFileData::from_with_content_type(
            "/".to_owned() + head2,
            if let Some(head4) = args.line_splitted.next() {
                head4.to_string()
            } else {
                "text/html; charset=utf-8".to_string()
            },
        ),
    );
}
fn method_remove(args: MethodArgs) {
    if let Some(head2) = args.line_splitted.next() {
        method_remove_head2_ext(args, head2);
    } else {
        syntax_error(args.file, args.line_number, LOG[18]);
    }
}
fn method_remove_head2_ext(args: MethodArgs, head2: &str) {
    if head2 == "/" {
        if args
            .config
            .router_config
            .serve_files_info
            .remove("/")
            .is_some()
        {
        } else {
            syntax_error(args.file, args.line_number, LOG[19]);
        }
    } else if args
        .config
        .router_config
        .serve_files_info
        .remove(&("/".to_owned() + head2))
        .is_some()
    {
    } else {
        syntax_error(args.file, args.line_number, LOG[19]);
    };
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
        if l.is_err() {
            syntax_error(
                args.file,
                args.line_number,
                &format!("{}{}", LOG[22], "export/".to_owned() + head2),
            );
            return;
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
        }
    }
}
fn method_inject(mut args: MethodArgs) {
    if method_inject_haserr(&mut args) == Err(()) {
        syntax_error(args.file, args.line_number, LOG[25]);
    }
}
#[cfg(not(feature = "no-glisp"))]
fn method_import_gl(args: MethodArgs) {
    if let Some(head2) = args.line_splitted.next() {
        let env = &mut crate::glisp::core::default_env();
        match crate::glisp::core::parse_eval(
            read_to_string("config/".to_owned() + head2)
                .result_shldfatal(-1, || log!(Fatal, format!("{}{}", LOG[22], head2))),
            env,
        ) {
            Ok(res) => log!(Info, format!("[{}] {} {}", LOG[32], LOG[33], res)),
            Err(e) => match e {
                crate::glisp::core::GError::Reason(msg) => {
                    log!(Info, format!("[{}] {} {}", LOG[32], LOG[34], msg))
                }
            },
        }
    }
}
#[cfg(not(feature = "no-glisp"))]
fn method_import_pipe(args: MethodArgs) {
    if let Some(head2) = args.line_splitted.next() {
        args.config.router_config.pipe.push(
            read_to_string("config/".to_owned() + head2)
                .result_shldfatal(-1, || log!(Fatal, format!("{}{}", LOG[22], head2))),
        );
    }
}
fn method_log(args: MethodArgs) {
    log!(
        Info,
        args.line_splitted
            .map(|s| s.to_owned() + " ")
            .collect::<String>()
    );
}
fn method_inject_haserr(args: &mut MethodArgs) -> Result<(), ()> {
    let pathname = if let Some(a) = args.line_splitted.next() {
        a
    } else {
        return Err(());
    };
    let temp_pathname = &("/".to_owned() + pathname);
    let conf_serve_value = if let Some(a) = args
        .config
        .router_config
        .serve_files_info
        .get_mut(if pathname == "/" { "/" } else { temp_pathname })
    {
        a
    } else {
        return Err(());
    };
    let filename = &conf_serve_value.file_path;
    if !Path::new(&("temp/".to_string() + &filename)).is_file() {
        return Err(());
    }

    let lines = if let Ok(a) = read_lines("temp/".to_owned() + &filename) {
        a
    } else {
        return Err(());
    };
    let mut linenumbers: Vec<(u32, u32)> = vec![];
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
        if let Some(ori_tur) = &mut conf_serve_value.replace {
            if let Some(f) = linenumbers.get(linenumber) {
                ori_tur.push(ReplaceData {
                    content: if let Some(f) = args.line_splitted.next() {
                        match std::fs::read_to_string("export/".to_owned() + f) {
                            Ok(a) => a,
                            _ => return Err(()),
                        }
                    } else {
                        return Err(());
                    },
                    column: f.0,
                    line: f.1,
                });
            } else {
                return Err(());
            };
        } else if let Some(f) = linenumbers.get(linenumber) {
            conf_serve_value.replace = Some(vec![ReplaceData {
                content: if let Some(f) = args.line_splitted.next() {
                    match std::fs::read_to_string("export/".to_owned() + f) {
                        Ok(a) => a,
                        _ => return Err(()),
                    }
                } else {
                    return Err(());
                },
                column: f.0,
                line: f.1,
            }]);
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
