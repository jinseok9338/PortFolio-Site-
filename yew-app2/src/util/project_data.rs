pub struct ProjectsType {
    pub project_title: String,
    pub project_description: String,
    pub src: String,
}

pub fn project_data() -> Vec<ProjectsType> {
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
