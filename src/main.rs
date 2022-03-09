use say::encode;
//use crate powershell_script;
fn main() {
    loop {
        let mut buf = String::new();

        println!("Insert an integer: ");
        std::io::stdin().read_line(&mut buf).unwrap();

        if buf == "exit" {
            break;
        }
        let num: u64 = buf.trim().parse().expect("Incorrect number");

        let word = encode(num);

        println!("{}", word);
    }
}
