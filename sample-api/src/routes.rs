use rocket_contrib::json::Json;
use crate::models::Recipe;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/recipes")]
pub fn recipes() -> Json<Vec<Recipe>> {
    Json(vec![Recipe {
        id: 1,
        title: "Read Rocket tutorial".into(),
        description: "Read https://rocket.rs/guide/quickstart/".into(),
        done: false,
    }])
}

#[post("/recipes", data = "<recipe>")]
pub fn crate_recipe(recipe: Json<Recipe>) -> String {
    format!("Accepted post request! {:?}", recipe.0)
}

#[get("/recipes/<recipe_id>")]
pub fn get_recipe_by_id(recipe_id: u32) -> String {
    let recipe = Recipe {
        id: 1,
        title: "Read Rocket tutorial".into(),
        description: "Read https://rocket.rs/guide/quickstart/".into(),
        done: false,
    };
    format!("{:?}", recipe)
}
