use chronicle_client::Client;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = &args[1];
    let res = Client::get(input.to_string());
    println!("{}", res.print());
}
