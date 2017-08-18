use chrono;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use api::Api;
use duration;
use errors::*;
use output::{OutputData, OutputMeta, OutputSet};
use settings::Settings;
use types;

pub fn run(config_path: Option<&str>) -> Result<()> {
    let sets = Settings::new(config_path)?;

    let output_set = OutputSet::new(&sets.outputs)?;

    let now = chrono::Utc::now();

    let recent = duration::parse(&sets.recent)?;
    let recent_earliest = now - recent;

    let stale = duration::parse(&sets.stale)?;
    let stale_latest = now - stale;

    let api = Api::new(
        &sets.github.token,
        &sets.github.host
    )?;

    let repos: Vec<types::Repository> = sets.github.subjects.par_iter()
        .flat_map(|subject| api.list_repos(subject))
        .flat_map(|ve| ve)
        .collect();

    info!("Found {} repositories", repos.len());

    let prs: Vec<types::PullRequest> = repos.par_iter()
        .flat_map(|repo| api.list_pull_requests(&repo.full_name))
        .flat_map(|ve| ve)
        .collect();

    info!("Found {} pull requests", prs.len());

    let mut created_prs: Vec<&types::PullRequest> = prs.iter().filter(|pr| {
            pr.created_at.parse::<chrono::DateTime<chrono::Utc>>()
                .map(|dt| dt >= recent_earliest).unwrap_or(false)
        }).collect();
    created_prs.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    let mut updated_prs: Vec<&types::PullRequest> = prs.iter().filter(|pr| {
            pr.updated_at.parse::<chrono::DateTime<chrono::Utc>>()
                .map(|dt| dt >= recent_earliest).unwrap_or(false)
        }).collect();
    updated_prs.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

    let mut stale_prs: Vec<&types::PullRequest> = prs.iter().filter(|pr| {
            pr.updated_at.parse::<chrono::DateTime<chrono::Utc>>()
                .map(|dt| dt <= stale_latest).unwrap_or(false)
        }).collect();
    stale_prs.sort_by(|a, b| a.updated_at.cmp(&b.updated_at));;

    let meta = OutputMeta {
        now: now.with_timezone::<chrono::offset::Local>(&chrono::offset::Local),
        recent: recent,
        stale: stale
    };

    let data = OutputData {
        total: &prs,
        created: &created_prs,
        updated: &updated_prs,
        stale: &stale_prs
    };

    output_set.remind_all(&meta, &data);

    Ok(())
}
