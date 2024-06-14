mod cli;
use dialoguer::{theme::ColorfulTheme, MultiSelect};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let token = match std::env::var("GITHUB_TOKEN") {
        Ok(token) => token,
        Err(_) => panic!("Missing github token"),
    };

    let username = match std::env::var("GITHUB_USER") {
        Ok(username) => username,
        Err(_) => panic!("Missing github user"),
    };

    let mut app = cli::App::new(username, token);

    app.init_repositories().await?;

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select repositories to be deleted")
        .items(&app.get_repositories())
        .interact()
        .unwrap();

    if selections.is_empty() {
        println!("No repositories selected");
        return Ok(());
    }

    let mut handles = Vec::new(); // Vector to hold JoinHandles

    for selection in selections {
        let app_clone = app.clone();
        let handle = tokio::spawn(async move {
            match app_clone.delete_repository(selection).await {
                Ok(_) => {
                    println!(
                        "Deleted repo github.com/{}/{}",
                        app_clone.get_username(),
                        app_clone.get_repository(selection).unwrap()
                    );
                    ()
                }
                Err(error) => println!("{}", error),
            }
        });

        handles.push(handle)
    }

    tokio::join!(async {
        for handle in handles {
            handle.await.unwrap();
        }
    });

    Ok(())
}
