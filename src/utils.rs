pub fn gen_ir_file(snl_str: String) -> String {
    let prefix = r#"
fn read<T>() -> T 
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let res = match buf.trim().parse() {
        Ok(res) => res,
        Err(e) => {
            println!("{:?}: {:?}", "unmatch type", e);
            std::process::exit(1);
        }
    };
    res
}"#;
    let mut ir_file = String::from(prefix);
    let snl_str = format!("snlc_parse::snl!{{\n{}\n}}", snl_str);
    ir_file.push_str("\n");
    ir_file.push_str(format!("fn main() {{\n{}\n}}", snl_str).as_str());
    ir_file
}