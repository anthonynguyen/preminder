use chrono;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use api::Api;
use duration;
use errors::*;
use output;
use settings::Settings;
use types;

pub fn run(config_path: Option<&str>) -> Result<()> {
    let sets = Settings::new(config_path)?;

    let outputs = output::init(&sets.outputs)?;

    let now = chrono::Utc::now();

    let recent = duration::parse(&sets.recent)?;
    let recent_earliest = now - recent;

    let stale = duration::parse(&sets.stale)?;
    let stale_latest = now - stale;

    let api = Api::new(
        &sets.github.token,
        &sets.github.host
    )?;

    let repos = api.list_repos(&sets.github.subject)?;
    println!("Found {} repositories", repos.len());

    let prs: Vec<types::PullRequest> = repos.par_iter()
        .flat_map(|repo| api.list_pull_requests(&repo.full_name))
        .flat_map(|ve| ve)
        .collect();

    let total_prs = prs.len();
    println!("Found {} pull requests", total_prs);

    let created_prs: Vec<&types::PullRequest> = prs.iter().filter(|pr| {
        pr.created_at.parse::<chrono::DateTime<chrono::Utc>>()
            .map(|dt| dt >= recent_earliest).unwrap_or(false)
    }).collect();

    let updated_prs: Vec<&types::PullRequest> = prs.iter().filter(|pr| {
        pr.updated_at.parse::<chrono::DateTime<chrono::Utc>>()
            .map(|dt| dt >= recent_earliest).unwrap_or(false)
    }).collect();

    let stale_prs: Vec<&types::PullRequest> = prs.iter().filter(|pr| {
        pr.updated_at.parse::<chrono::DateTime<chrono::Utc>>()
            .map(|dt| dt <= stale_latest).unwrap_or(false)
    }).collect();

    let meta = output::OutputMeta {
        now: now.with_timezone::<chrono::offset::Local>(&chrono::offset::Local),
        recent: recent.clone(),
        stale: stale.clone()
    };

    for output in &outputs {
        output.remind(&meta, &prs, &created_prs, &updated_prs, &stale_prs)?;
    }

    Ok(())
}
