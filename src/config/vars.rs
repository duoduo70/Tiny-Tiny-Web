/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */

use crate::{drop::log::LogLevel::*, macros::*};

use super::*;

pub fn method_set(args: MethodArgs) {
    macro_rules! float_read_to {
        ($var:ident, $field:ident) => {
            $var.store(
                if let Ok(a) = $field.parse::<f32>() {
                    (a * 1000.0) as u32
                } else {
                    syntax_error(
                        args.file,
                        args.line_number,
                        &format!("{}{}", LOG[17], $field),
                    );
                    $var.load(Ordering::Relaxed)
                },
                Ordering::Relaxed,
            )
        };
    }
    if let Some(head2) = args.line_splitted.next() {
        if let Some(head3) = args.line_splitted.next() {
            match head2 {
                "localtime" => pas_bool_option(
                    &mut args.config.use_localtime,
                    head3,
                    args.file,
                    args.line_number,
                ),
                "debug" => pas_bool_option(
                    &mut args.config.enable_debug,
                    head3,
                    args.file,
                    args.line_number,
                ),
                "+errpage" => {
                    if let Some(head4) = args.line_splitted.next() {
                        if head3 == "404" {
                            page_404_option(args, head4);
                        } else {
                            syntax_error(
                                args.file,
                                args.line_number,
                                &format!("{}{}", LOG[36], head3),
                            )
                        }
                    } else {
                        syntax_error(args.file, args.line_number, LOG[18]);
                    }
                }
                "+addr" => args.config.addr_bind.push(head3.to_owned()),
                "+mime" => {
                    if let Some(head4) = args.line_splitted.next() {
                        args.config
                            .mime_bind
                            .insert(head3.to_owned(), head4.to_owned());
                    } else {
                        syntax_error(args.file, args.line_number, LOG[18]);
                    }
                }
                "+code" => match head3 {
                    "400" => args.config.status_codes.push(400),
                    "404" => args.config.status_codes.push(404),
                    _ => syntax_error(
                        args.file,
                        args.line_number,
                        &format!("{}{}", LOG[36], head3),
                    ),
                },
                "threads" => THREADS_NUM.store(
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
                ),
                "ssl-certificate" => {
                    #[cfg(feature = "nightly")]
                    unsafe {
                        SSL_CERTIFICATE = Some(std::sync::Arc::new(
                            std::fs::read(head3.to_owned()).unwrap(),
                        )) // TODO: Error log
                    }
                }
                "ssl-private-key" => {
                    #[cfg(feature = "nightly")]
                    unsafe {
                        SSL_PRIVATE_KEY = Some(std::sync::Arc::new(
                            crate::drop::base64::decode_unchecked(head3)[4..36].to_vec(),
                        )); // TODO: Error log
                    }
                }
                "xrps-counter-cache-size" => float_read_to!(XRPS_COUNTER_CACHE_SIZE, head3),
                "box-num-per-thread-mag" => float_read_to!(BOX_NUM_PER_THREAD_MAG, head3),
                "box-num-per-thread-init-mag" => float_read_to!(BOX_NUM_PER_THREAD_INIT_MAG, head3),
                "xrps-predict-mag" => float_read_to!(XRPS_PREDICT_MAG, head3),
                "box-mode" => {
                    let mut value = false;
                    pas_bool_option(&mut value, head3, args.file, args.line_number);
                    BOX_MODE.store(value, Ordering::Relaxed);
                }
                "return-if-pipe-err" => {
                    let mut value = false;
                    pas_bool_option(&mut value, head3, args.file, args.line_number);
                    ENABLE_RETURN_IF_PIPE_ERR.store(value, Ordering::Relaxed);
                }
                "gl-debug" => {
                    let mut value = false;
                    pas_bool_option(&mut value, head3, args.file, args.line_number);
                    GLISP_DEBUG.store(value, Ordering::Relaxed);
                }
                "gl-stack" => {
                    let mut value = true;
                    pas_bool_option(&mut value, head3, args.file, args.line_number);
                    GLISP_ENABLE_STACK.store(value, Ordering::Relaxed);
                }
                _ => syntax_error(
                    args.file,
                    args.line_number,
                    &format!("{}{}", LOG[17], head3),
                ),
            }
        } else {
            syntax_error(args.file, args.line_number, LOG[18]);
        }
    } else {
        syntax_error(args.file, args.line_number, LOG[18]);
    }
}

fn pas_bool_option(option: &mut bool, opt_str: &str, file: &str, line_number: i32) {
    if opt_str == "yes" {
        *option = true;
    } else if opt_str == "no" {
        *option = false;
    } else {
        syntax_error(file, line_number, &format!("{}{}", LOG[17], opt_str));
    }
}

fn page_404_option(args: MethodArgs, head3: &str) {
    let mut res = HttpResponse::new();
    res.set_content(
        if let Ok(a) = std::fs::read_to_string("export/".to_owned() + head3) {
            a.into()
        } else {
            log!(Error, format!("{}{}", LOG[22], head3));
            return;
        },
    );
    args.config.router_config.response_404 = Some(res);
}
