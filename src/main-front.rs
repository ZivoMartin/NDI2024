use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

//fn on_validate_click<F>(ev: MouseEvent, f: F)
//where
//    F: FnOnce(usize),
//{
//}

fn node(id: usize) -> (usize, String) {
    (id, String::from(""))
}

#[component]
fn App() -> impl IntoView {
    let (title, set_title) = create_signal("".to_string());
    let mut next_ingredient_id = 2;
    let (ingredients, set_ingredients) =
        create_signal::<Vec<(usize, String)>>(vec![node(0), node(1), node(2)]);
    let mut next_step_id = 2;
    let (steps, set_steps) = create_signal::<Vec<(usize, String)>>(vec![node(0), node(1), node(2)]);
    let input_title: NodeRef<html::Input> = create_node_ref();
    let on_validate_click = move |ev: leptos::ev::MouseEvent| {
        //set_title(input_title().expect("<input> should be mounted").value());
        logging::log!("{:#?}", ingredients());
        logging::log!("{:#?}", steps());
        logging::log!("{:#?}", title());
        let r = fs::write("test.txt", "salut");
        logging::log!("{:#?}", r);
    };
    let add_ingredient = move |_| {
        next_ingredient_id += 1;
        set_ingredients.update(move |ingredients| ingredients.push(node(next_ingredient_id)));
    };
    let add_steps = move |_| {
        next_step_id += 1;
        set_steps.update(move |steps| steps.push(node(next_step_id)));
    };
    let change_ingredient = move|id: usize, s: String| {
        let i = ingredients().iter().position(|v|v.0==id).unwrap();
        set_ingredients.update(|ingredients|ingredients[i].1 = s);
    };
    let change_step = move|id: usize, s: String| {
        let i = steps().iter().position(|v|v.0==id).unwrap();
        set_steps.update(|steps|steps[i].1 = s);
    };

    view! {
        //<div class="flex">
        //<p>"Titre de la recette: "</p>
        <input type="text" node_ref=input_title placeholder="Titre de la recette"
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
    }
}
