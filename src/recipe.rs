use leptos::server;
use std::fs::File;

use leptos::{logging, ServerFnError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    title: String,
    ingredients: Vec<String>,
    steps: Vec<String>,
}
impl Recipe {
    pub fn new(title: String, ingredients: Vec<String>, steps: Vec<String>) -> Self {
        Self {
            title,
            ingredients,
            steps,
        }
    }
}

fn name(title: &String) -> String {
    title.to_lowercase().replace(' ', "-")
}
#[server(AddRecipe)]
pub async fn save_recipe(recipe: Recipe) -> Result<(), ServerFnError> {
    let template = mustache::compile_str(include_str!("recipe.mustache")).unwrap();
    let filename = format!("{}.html", name(&recipe.title));
    let mut file = File::create(&filename).unwrap();
    template.render(&mut file, &recipe).unwrap();
    logging::log!("file {} saved", filename);

    Ok(())
}
