use std::{env, fs::{self, File}};

use serde::Serialize;

#[derive(Debug,Serialize)]
struct Data {
    title: String,
    ingredients: Vec<String>,
    steps: Vec<String>
}
impl Data {
    fn new() -> Self {
        Self {
            title: "".to_string(),
            ingredients: vec![],
            steps: vec![]
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("you must set the input file");
        return;
    }
    let template = mustache::compile_str(include_str!("recipe.mustache")).unwrap();
    let input_name = &args[1][..];
    let input = fs::read_to_string(input_name).expect("unable to read the file");
    let mut data = Data::new();    
    let mut lines = input.lines().into_iter();
    if let Some(title) = lines.next() {
        data.title = title.to_string();
    }
    while let Some(ingredient) = lines.next() {
        if ingredient.is_empty() {
            break;
        }
        data.ingredients.push(ingredient.to_string());
    }
    while let Some(step) = lines.next() {
        if step.is_empty() {
            break;
        }
        data.steps.push(step.to_string());
    }
    let filename = format!("{}.html", data.title.replace(" ", "-"));
    let mut file = File::create(&filename).unwrap();
    template.render(&mut file, &data).unwrap();
    println!("{}", filename);
}
