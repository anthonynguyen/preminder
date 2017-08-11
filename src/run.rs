use chrono;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use api::Api;
use duration;
use errors::*;
use settings::Settings;
use types;

pub fn run(config_path: Option<&str>) -> Result<()> {
    let sets = Settings::new(config_path)?;

    let period = duration::parse(sets.period)?;
    let earliest = chrono::Utc::now() - period;

    let api = Api::new(
        sets.github.token,
        sets.github.host
    )?;

    let repos = api.list_repos(&sets.github.subject)?;
    println!("Found {} repositories", repos.len());

    let prs: Vec<types::PullRequest> = repos.par_iter()
        .flat_map(|repo| api.list_pull_requests(&repo.full_name))
        .flat_map(|ve| ve)
        .collect();

    let before_prs = prs.len();
    println!("Found {} pull requests", before_prs);

    let prs: Vec<&types::PullRequest> = prs.iter().filter(|pr| {
        pr.created_at.parse::<chrono::DateTime<chrono::Utc>>()
            .map(|dt| dt >= earliest)
            .unwrap_or(false)
    }).collect();

    let after_prs = prs.len();
    println!("Filtered out {} older pull requests ({} left)",
        before_prs - after_prs, after_prs);

    for pr in &prs {
        println!("[{}] {} ({})\n{}\n", pr.base.repo.full_name, pr.title,
            pr.state, pr.user.login);
    }

    Ok(())
}
