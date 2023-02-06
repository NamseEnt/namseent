use clap::*;

#[derive(Parser)]
#[command(about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    AddUser(AddUser),
}

#[derive(Args)]
struct AddUser {
    #[arg(short, long)]
    username: String,
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    server_core::init().await;
    let cli = Cli::parse();

    match cli.command {
        Command::AddUser(AddUser { username }) => {
            let result = server_core::services::auth::get_or_create_user(
                server_core::services::auth::UserIdentity::Github {
                    github_user_id: "ONLY_FOR_LOCAL_TEST".to_string(),
                    username,
                },
            )
            .await;
            match result {
                Ok(user) => {
                    println!("Success! user id: {}, username: {}", user.id, user.name);
                }
                Err(error) => {
                    println!("Error: {}", error);
                }
            }
        }
    }
}
