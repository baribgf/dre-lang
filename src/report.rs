pub fn fatal(err: &str) -> ! {
    panic!("Fatal: {}", err)
}

pub fn error(msg: &str, line: &str, source_path: &str, line_num: usize, col_num: usize) -> ! {
    let extra_pad = " ".repeat(2 + 4 + line_num.to_string().chars().count());
    panic!(
        "{}:{}:{}\n\n{} |\t{}\n{}^\n{}\n",
        source_path,
        line_num,
        col_num,
        line_num,
        line,
        extra_pad.clone() + &"_".repeat(col_num - 1),
        msg
    );
}
