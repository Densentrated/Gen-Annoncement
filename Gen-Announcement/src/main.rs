use clap::Parser;
use rand::Rng;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[arg(long)]
    date: String,

    #[arg(long)]
    time: String,

    #[arg(long, default_value = "")]
    extra: String, 
}


fn main() {
    let args = Args::parse();
    let greeting = set_greeting();

    println!("{}", greeting);
    println!("we have a meeting from {} on {}", args.time, args.date);
    println!("{}", args.extra);
}

fn set_greeting() -> &'static str {
    let greeting_list = 
    [        
        "Bonjour, morning!",
        "YeeHaw, my friends!",
        "Morning, Govna's!",
        "Top of the morning to you all!",
        "What's up, docs and dudettes?",
        "Greetings and salutations, folks!",
        "Rise and shine, everyone!",
        "Howdy-doody, y'all!",
        "Hey there, bright and early!",
        "Good day, fine folks!",
        "Aloha, morning surfers!",
        "Hello, early birds!",
        "Salutations, sun-chasers!",
        "Mornin', partners in crime!",
        "Time to seize the day, amigos!",
        "Hey, world-wakers!",
        "Wakey-wakey, rise and shine!",
        "G'day, champions of the day!",
        "A bright morning to you all!",
        "How's it hanging, early risers?",
        "Good mornin', daybreak darlings!",
        "What's crackin', dawn patrol!",
        "Hello, sunshine enthusiasts!",
        "Greetings, morning magicians!",
        "Time to shine, daybreak divas!",
        "Wake up and conquer, warriors!",
        "Hey there, sunrise enthusiasts!",
        "Rise and thrive, morning champions!",
        "What's cooking, sun seekers?",
        "Hola, early risers!",
        "Morning glory awaits, rise and shine!",
        "Greetings, break of day enthusiasts!",
        "Roll out of bed and conquer, champs!",
        "Howdy, mornin' stars!",
        "Good morning, sun-kissed souls!",
        "Hello, dawn explorers!",
        "Rise like the morning sun, friends!",
        "How's the sunrise treating you?",
        "Greetings, early sunshine enthusiasts!",
        "Top o' the mornin', dream chasers!",
        "Hello, first light champions!",
        "Wishing you a day as bright as the morning sun!",
        "Rise and dazzle, early risers!",
        "Morning magic, brought to you by the sun!",
        "Salute the day, morning pioneers!",
        "How's the world lookin' from your side?",
        "Time to ignite your day, early birds!",
        "Hello, sunshine adventurers!",
        "Embrace the day, morning explorers!",];
    let mut rng = rand::thread_rng();
    let today_index = rng.gen_range(0..greeting_list.len());

    return greeting_list[today_index];
}