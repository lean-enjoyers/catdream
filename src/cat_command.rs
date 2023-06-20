use pad::PadStr;
use string_builder::Builder;
use textwrap::wrap;

const ASCII_CAT: &str = "
\to
\t o  |\\__/,|   (`\\
\t  o |_ _  |.--.) )
\t    ( T   )     /
\t   (((^_(((/(((_/
";

const MAX_LINE_LEN: usize = 40;

pub fn print_msg_box(msg: &str) {
    let lines: Vec<String> = wrap(msg, MAX_LINE_LEN)
        .iter()
        .map(|cow_str| cow_str.to_string())
        .collect();
    let max_line_len: usize = lines.iter().map(|line| line.len()).max().unwrap();

    let mut builder = Builder::default();
    builder.append(format!("{}{}{}", "╭", "─".repeat(max_line_len + 2), "╮\n"));
    lines.iter().for_each(|line| {
        builder.append("│ ");
        builder.append(line.as_str().pad_to_width(max_line_len));
        builder.append(" │\n");
    });
    builder.append(format!("{}{}{}", "╰", "━".repeat(max_line_len + 2), "╯"));

    print!("{}", builder.string().unwrap());
}

pub fn print_cat() {
    print!("{}", ASCII_CAT);
}
