use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, date, time, extra)]
struct Args {
    #[arg(short, long)]
    date: String,

    #[arg(short, long)]
    time: String,

    #[arg(short, long)]
    extra: String, 
}


fn main() {
    let args = Args::parse();
    let greeting = "yo!";

    println!("{}, we have a meeting from {} on {}", greeting, args.time, args.date)
}

fn setGreeting() {

}