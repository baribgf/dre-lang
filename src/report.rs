use crate::utils::Loc;

pub fn fatal(err: &str) -> ! {
    panic!("Fatal: {}", err)
}

pub fn error(msg: &str, line: &str, source_path: &str, start: Loc, end: Loc) -> ! {
    let line_num = start.0;
    let col_num = start.1;
    let extra_pad = " ".repeat(2 + 4 + line_num.to_string().chars().count());
    panic!(
        "{}:{}:{}\n\n{} |\t{}\n{}{}\n\n{}\n",
        source_path,
        line_num,
        col_num,
        line_num,
        line,
        extra_pad + &"_".repeat(col_num - 1),
        "^".repeat((end.1 - start.1).max(1)),
        msg
    );
}
