
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
}
fn main() {
snlc_parse::snl!{
r#program hello
r#var r#char a;
    r#integer i;
r#begin
  r#read(a);
  r#while a < 10 r#do
    i := i + 1;
    r#write(a)
  r#endwh
r#end.

}
}