use serde::Deserialize;
use std::env;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Repo {
    name: String,
    html_url: String,
    description: Option<String>,
    language: Option<String>,
    stargazers_count: u32,
}

fn fetch_repo_info(repo_url: &str) -> Result<Repo, Box<dyn Error>> {
    let url = format!("https://api.github.com/repos/{}", repo_url);
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "RustApp")
        .send()?
        .json::<Repo>()?;
    Ok(response)
}

fn compare_repos(repo_urls: Vec<&str>) -> Result<Vec<Repo>, Box<dyn Error>> {
    let mut repos = Vec::new();
    for url in repo_urls {
        match fetch_repo_info(url) {
            Ok(repo) => repos.push(repo),
            Err(e) => eprintln!("Error fetching repo {}: {}", url, e),
        }
    }
    Ok(repos)
}

fn main() -> Result<(), Box<dyn Error>> {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();

    // 引数がなければエラーメッセージを表示
    if args.len() < 2 {
        eprintln!("Usage: {} <repo1> <repo2> ...", args[0]);
        return Ok(());
    }

    let repo_urls: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();

    let repos = compare_repos(repo_urls)?;

    println!(
        "{:<30} {:<50} {:<30} {:<15} {}",
        "Name", "URL", "Description", "Language", "Stars"
    );
    println!("{}", "-".repeat(130));

    for repo in repos {
        println!(
            "{:<30} {:<50} {:<30} {:<15} {}",
            repo.name,
            repo.html_url,
            repo.description
                .unwrap_or_else(|| "No description".to_string()),
            repo.language.unwrap_or_else(|| "No language".to_string()),
            repo.stargazers_count
        );
    }

    Ok(())
}