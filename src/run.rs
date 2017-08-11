use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use api::Api;
use errors::*;
use settings::Settings;
use types;

pub fn run(config_path: Option<&str>) -> Result<()> {
    let sets = Settings::new(config_path)?;

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

    println!("Found {} pull requests", prs.len());

    for pr in &prs {
        println!("[{}] {} ({})\n{}\n", pr.base.repo.full_name, pr.title,
            pr.state, pr.user.login);
    }

    Ok(())
}
