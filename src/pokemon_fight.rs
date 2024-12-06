use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[derive(Debug)]
struct Attack {
    name: &'static str,
    damages: u32,
}

impl Attack {
    fn new(name: &'static str, damages: u32) -> Self {
        Self { name, damages }
    }

    fn thunder() -> Self {
        Self::new("thunder.png", 30)
    }

    fn iron_queue() -> Self {
        Self::new("ironQueue.png", 20)
    }
    fn electro_orb() -> Self {
        Self::new("electroOrb.png", 50)
    }
    fn vive_attack() -> Self {
        Self::new("vivattack.png", 30)
    }
}

#[derive(Debug)]
struct PokemonDatas {
    path: &'static str,
    attacks: [Attack; 4],
    pv_max: u64,
    speed: u64,
}

impl PokemonDatas {
    fn new(path: &'static str, attacks: [Attack; 4], pv_max: u64, speed: u64) -> Self {
        PokemonDatas {
            path,
            attacks,
            pv_max,
            speed,
        }
    }

    fn pikachu() -> Self {
        PokemonDatas::new(
            "pikachu.png",
            [
                Attack::thunder(),
                Attack::iron_queue(),
                Attack::electro_orb(),
                Attack::vive_attack(),
            ],
            100,
            5,
        )
    }
}

impl From<String> for PokemonDatas {
    fn from(pokemon: String) -> PokemonDatas {
        match &pokemon as &str {
            "pikachu" => PokemonDatas::pikachu(),
            _ => panic!("This pokemon doesn't exists: {pokemon}"),
        }
    }
}

use wasm_bindgen::{JsCast, JsValue};
use web_sys::CanvasRenderingContext2d; // For `dyn_into`

/// Renders the home page with a Pokémon fight canvas.
#[component]
pub fn PokemonFight() -> impl IntoView {
    // Create a reference to the canvas element
    let canvas_ref = create_node_ref::<html::Canvas>();

    // Run code when the component is first mounted
    create_effect(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let mut ctx = canvas
                .get_context("2d")
                .expect("Failed to get canvas context")
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>() // Corrected context type
                .unwrap();

            CanvasRenderingContext2d::set_line_width(&mut ctx, 10.0);

            // Wall
            CanvasRenderingContext2d::stroke_rect(&mut ctx, 75.0, 140.0, 150.0, 110.0);

            // Door
            CanvasRenderingContext2d::fill_rect(&mut ctx, 130.0, 190.0, 40.0, 60.0);

            // Roof
            CanvasRenderingContext2d::begin_path(&mut ctx);
            CanvasRenderingContext2d::move_to(&mut ctx, 50.0, 140.0);
            CanvasRenderingContext2d::line_to(&mut ctx, 150.0, 60.0);
            CanvasRenderingContext2d::line_to(&mut ctx, 250.0, 140.0);
            CanvasRenderingContext2d::close_path(&mut ctx);
            CanvasRenderingContext2d::stroke(&mut ctx);
        }
    });

    view! {
        <div>
            <h1>"Pokémon Fight!"</h1>
            <canvas node_ref=canvas_ref width="1000" height="800" style="border:1px solid black;"></canvas>
        </div>
    }
}
