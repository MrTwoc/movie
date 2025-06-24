use clap::{arg, Parser, Subcommand};
use movie::handler::{handler_login, handler_logout};


#[derive(Parser)]
#[command(
    version,
    about = "Movie App",
    long_about = "A simple movie application to manage your movie collection.",
)]
struct Cli {
    #[command(subcommand)]
    commands: Option<Commands>
}
#[derive(Subcommand)]
enum Commands {
    /// User log into the system
    Login {
        /// The username of the user
        #[arg(short, long)]
        username: String,
    },
    /// 退出登录
    Logout,
}

fn main()-> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match &cli.commands {
        Some(Commands::Login { username }) => handler_login(username)?,
        Some(Commands::Logout) => handler_logout(),
        _ => {println!("Please provide a valid command.");}
    }

    Ok(())
}
