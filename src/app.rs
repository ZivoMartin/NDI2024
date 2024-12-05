use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptonic::components::prelude::*;

use crate::recipe::{save_recipe, Recipe};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-server-tutorial.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Root default_theme=LeptonicTheme::default()>
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
        </Root>
    }
}

fn node(id: usize) -> (usize, String) {
    (id, String::from(""))
}
/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (title, set_title) = create_signal("".to_string());
    let mut next_ingredient_id = 2;
    let (ingredients, set_ingredients) =
        create_signal::<Vec<(usize, String)>>(vec![node(0), node(1), node(2)]);
    let mut next_step_id = 2;
    let (steps, set_steps) = create_signal::<Vec<(usize, String)>>(vec![node(0), node(1), node(2)]);
    let (alert, set_alert) = create_signal("".to_string());
    let on_validate_click = move |_| {
        if title().is_empty() {
            logging::log!("title is empty");
            set_alert("title is empty".to_string());
        } else {
            let title = title().clone();
            let ingredients = ingredients().iter().map(|(_, s)| s.clone()).collect();
            let steps = steps().iter().map(|(_, s)| s.clone()).collect();
            spawn_local(async {
                let _ = save_recipe(Recipe::new(title, ingredients, steps)).await;
            });
        }
    };
    let add_ingredient = move |_| {
        next_ingredient_id += 1;
        set_ingredients.update(move |ingredients| ingredients.push(node(next_ingredient_id)));
    };
    let add_steps = move |_| {
        next_step_id += 1;
        set_steps.update(move |steps| steps.push(node(next_step_id)));
    };
    let change_ingredient = move |id: usize, s: String| {
        let i = ingredients().iter().position(|v| v.0 == id).unwrap();
        set_ingredients.update(|ingredients| ingredients[i].1 = s);
    };
    let change_step = move |id: usize, s: String| {
        let i = steps().iter().position(|v| v.0 == id).unwrap();
        set_steps.update(|steps| steps[i].1 = s);
    };

    view! {
        <input type="text" placeholder="Titre de la recette"
            on:input=move |e| set_title(event_target_value(&e))
        />
        <br/>
        <br/>
        <h2>Ingrédients</h2>
        <ul>
        <For
            each=ingredients
            key=|ingredient| ingredient.0
            let:child
        >
            <li>
                <input type="text" value={child.1} placeholder="ingrédient"
                    on:input=move |e| change_ingredient(child.0, event_target_value(&e))
                ></input>
                <button on:click=move |_| set_ingredients.update(|ingredients| ingredients.retain(|ingredient|ingredient.0 != child.0))>delete</button>
            </li>
        </For>
        </ul>
        <button on:click=add_ingredient>+</button>
        <br/>
        <h2>Étapes</h2>
        <ul>
        <For
            each=steps
            key=|step| step.0
            let:child
        >
            <li>
                <input type="text" value={child.1} placeholder="étape"
                    on:input=move |e| change_step(child.0, event_target_value(&e))
                ></input>
                <button on:click=move |_| set_steps.update(|steps| steps.retain(|step|step.0 != child.0))>delete</button>
            </li>
        </For>
        </ul>
        <button on:click=add_steps>+</button>
        //</div>
        <br/>
        <button on:click=on_validate_click>"Valider"</button>
        { if !alert().is_empty() { "erreur" } else { "no erreur" } }
        <Show
            when=move || { !alert().is_empty() }
            fallback=|| "no errors"
        >
            <Alert variant=AlertVariant::Danger>
                <AlertTitle slot>"Erreur"</AlertTitle>
                <AlertContent slot>{alert}</AlertContent>
            </Alert>
            //<Alert variant=AlertVariant::Warning title="Erreur">{alert()}</Alert>
        </Show>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
