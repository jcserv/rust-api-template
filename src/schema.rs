use serde::{Deserialize, Serialize};

// #[derive(Deserialize, Debug)]
// pub struct FilterOptions {
//     pub page: Option<usize>,
//     pub limit: Option<usize>,
// }

// #[derive(Deserialize, Debug)]
// pub struct ParamOptions {
//     pub id: String,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAuthorSchema {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateAuthorSchema {
    pub name: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct CreateBookSchema {
    pub title: String,
    pub author_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateBookSchema {
    pub title: Option<String>,
}
