use crate::schema::projects;

#[derive(Queryable)]
pub struct Project {
    pub project_id: u32,
    pub name: String
}

// ↓insertする場合に必要
#[derive(Insertable)]
// ↓これがないと use of undeclared crate or module `new_projects`エラー出る
#[table_name = "projects"]
pub struct NewProject {
    pub name: String
}

