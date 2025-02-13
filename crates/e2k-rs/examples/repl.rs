use std::io::Write;

fn gets() -> String {
    print!("> ");
    std::io::stdout().flush().unwrap();
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    buf
}

fn main() {
    let c2k = e2k::C2k::new(32);
    loop {
        let line = gets();
        if line.is_empty() {
            break;
        }
        let dst = c2k.infer(&line);
        println!("{} -> {}", line, dst);
    }
}
