use clap::Parser;

#[derive(Debug, Parser)]
/// A little CLI that outputs a message.
struct Arguments {
    ///The thing to say.
    message: String,
    
    #[clap(short, long)]
    mad: bool,

    #[clap(short, long)]
    sad: bool,
}

fn main(){
    let args = Arguments::parse();
    //println!("{}",args.message);

    let message = &args.message;
    let dashes = "-".repeat(message.len() + 2);
    println!("         +{dashes}+");
    println!("         | {message} |");
    println!("         +{dashes}+");
    println!("        /");

    let mad: bool = args.mad;
    if mad == true {
        println!("(╯°□°)╯︵ ┻━┻");
    } else if args.sad == true{
       println!("≽( ಡ_ಡ )≼");
    } else {
        println!("≽(◕ ᴗ ◕)≼");
    }
}