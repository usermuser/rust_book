use clap::Parser;

#[derive(Parser)]
struct Options {
    #[clap(default_value = "Meow!")]
    /// What does the cat say?
    message: String,

    #[clap(short = 'd', long = "dead")]
    /// Make the cat appear dead
    dead: bool,
}

fn main() {
    let options = Options::parse();
    let message = options.message;

    let eye = if options.dead { "x" } else { "o" };

    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("      /\\_/\\");
    println!("     ( {eye}  {eye} )");
    println!("      =( I )=");
}
