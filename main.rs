use rand::Rng;
use std::io;
// imports ^

// 8 ball function
fn magic_ball() {
  loop {
    // responses v
    let responses = ["It is certain.", "It is decidedly so.", "Without a doubt.", "Yes, definitely.", "You may rely on it.", "As I see it, yes.", "Most likely.", "Outlook good.", "Yes.", "Signs point to yes.", "Reply hazy. Try again.", "Ask again later.", "Better not tell you now.", "Cannot predict now.", "Concentrate and ask again.","Don't count on it.", "My reply is no.", "My sources say no.", "Outlook not so good.", "Very doubtful."];
    let mut question = String::new();
    // input ^ v
    io::stdin().read_line(&mut question).expect("Failed to read line");
    println!("Revealing your fate...");
    let mut rng = rand::thread_rng();
    let response = responses[rng.gen_range(0..responses.len())];
    println!("{}", response);
    // printing the response
  }
}
// main func
fn main() {
    println!("You just got a magic 8-Ball for your birthday! Ask a question to reveal the secrets of the universe.");
    magic_ball();
    // calling
}
