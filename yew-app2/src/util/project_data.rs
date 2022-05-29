use octocrab::params::repos::forks::Sort;
use octocrab::Octocrab;
pub struct ProjectsType {
    pub project_title: String,
    pub project_description: String,
    pub src: String,
}

async fn get_repos() -> octocrab::Result<()> {
    let token = std::env::var("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") // This temp key that lives only for 7 days
        .expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let repo = octocrab::instance()
        .repos("owner", "repo")
        .list_forks()
        // Optional Parameters
        .sort(Sort::Oldest)
        .page(2u32)
        .per_page(30)
        .send()
        .await?;

    let repo_metrics = octocrab
        .repos("rust-lang", "rust")
        .get_community_profile_metrics()
        .await?;

    println!(
        "{} has {} stars and {}% health percentage",
        repo.items[0].clone_url.clone().unwrap(),
        repo.items[0].description.clone().unwrap(),
        repo_metrics.health_percentage
    );

    Ok(())
}

pub async fn project_data() -> Vec<ProjectsType> {
    let result = get_repos().await.unwrap();
    let projects: Vec<ProjectsType> = vec![
        ProjectsType {
            project_title: "This is the title".to_string(),
            project_description: "This is the description".to_string(),
            src: "https://picsum.photos/675/300".to_string(),
        },
        ProjectsType {
            project_title: "This is the title".to_string(),
            project_description: "This is the description".to_string(),
            src: "https://picsum.photos/675/300".to_string(),
        },
    ];

    return projects;
}
