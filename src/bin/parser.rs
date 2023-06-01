use std::cell::RefCell;

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

    snlc_parse::snl!(
        r#program bubble
        r#var r#integer i, j, num;
            r#procedure q(r#integer num, r#integer awa)
            r#var r#integer k;
            r#begin
                k:=1;
                i:=num;
                r#write(i);
                r#while k <= 10 r#do
                    k:=k+1;
                    r#write(k)
                r#endwh
            r#end
        r#begin
            r#read(num);
            q(num, j)
        r#end.
    );
}
