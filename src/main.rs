use yew::prelude::*;
use log::info;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use gloo::events::EventListener;

// Utilise Supabase comme backend

enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Black,
    White,
}

struct Author {
    name: String,
}

struct Pixel {
    x: i32,
    y: i32,
    author: Option<Author>,
    color: Color,
}

struct Board {
    size: i32,
    pixels: Vec<Vec<Pixel>>,
}

impl Pixel {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y, author: None, color: Color::White }
    }

    fn set_author(&mut self, author: Author) {
        self.author = Some(author);
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl Board {
    fn new(size: i32) -> Self {
        let mut pixels = Vec::new();
        for x in 0..size {
            let mut row = Vec::new();
            for y in 0..size {
                row.push(Pixel::new(x, y));
            }
            pixels.push(row);
        }
        Self { size, pixels }
    }

    fn get_size(&self) -> i32 {
        self.size
    }

    fn get_pixel(&self, x: i32, y: i32) -> Option<&Pixel> {
        self.pixels.get(x as usize).and_then(|row| row.get(y as usize))
    }

    fn set_pixel_author(&mut self, x: i32, y: i32, author: Author) {
        if let Some(pixel) = self.pixels.get_mut(x as usize).and_then(|row| row.get_mut(y as usize)) {
            pixel.set_author(author);
        }
    }

    fn set_pixel_color(&mut self, x: i32, y: i32, color: Color) {
        if let Some(pixel) = self.pixels.get_mut(x as usize).and_then(|row| row.get_mut(y as usize)) {
            pixel.set_color(color);
        }
    }

    fn increase_size(&mut self, size: i32) {
        self.size += 1;
        self.pixels.push((0..self.size).map(|x| Pixel::new(x, self.size - 1)).collect());
        for row in self.pixels.iter_mut() {
            row.push(Pixel::new(self.size - 1, row.len() as i32));
        }
    }
}


#[function_component(App2)]
fn app2() -> Html {
    let canvas_ref = use_node_ref();
    let zoom_factor = use_state(|| 10.0);  // Facteur de zoom initial (10 pixels par "pixel")

    let colors = vec![
        vec!["#ff0000", "#00ff00", "#0000ff", "#ffff00", "#ff00ff", "#00ffff", "#000000", "#ffffff", "#888888", "#444444"],
        vec!["#ff0000", "#00ff00", "#0000ff", "#ffff00", "#ff00ff", "#00ffff", "#000000", "#ffffff", "#888888", "#444444"],
        vec!["#ff0000", "#00ff00", "#0000ff", "#ffff00", "#ff00ff", "#00ffff", "#000000", "#ffffff", "#888888", "#444444"],
        vec!["#ff0000", "#00ff00", "#0000ff", "#ffff00", "#ff00ff", "#00ffff", "#000000", "#ffffff", "#888888", "#444444"],
        vec!["#ff0000", "#00ff00", "#0000ff", "#ffff00", "#ff00ff", "#00ffff", "#000000", "#ffffff", "#888888", "#444444"],
        vec!["#ff0000", "#00ff00", "#0000ff", "#ffff00", "#ff00ff", "#00ffff", "#000000", "#ffffff", "#888888", "#444444"],
        vec!["#ff0000", "#00ff00", "#0000ff", "#ffff00", "#ff00ff", "#00ffff", "#000000", "#ffffff", "#888888", "#444444"],
        vec!["#ff0000", "#00ff00", "#0000ff", "#ffff00", "#ff00ff", "#00ffff", "#000000", "#ffffff", "#888888", "#444444"],
        vec!["#ff0000", "#00ff00", "#0000ff", "#ffff00", "#ff00ff", "#00ffff", "#000000", "#ffffff", "#888888", "#444444"],
        vec!["#ff0000", "#00ff00", "#0000ff", "#ffff00", "#ff00ff", "#00ffff", "#000000", "#ffffff", "#888888", "#444444"],
    ];
    
    {
        let canvas_ref = canvas_ref.clone();
        let zoom_factor = zoom_factor.clone();

        use_effect(move || {
            let canvas = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            // Fonction pour redessiner le canvas
            let draw = {
                let canvas = canvas.clone();
                let zoom_factor = zoom_factor.clone();  // Cloner l'état ici
                move || {
                    context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

                    let scale = *zoom_factor;
                    context.set_fill_style(&"#ffffff".into());
                    context.fill_rect(0.0, 0.0, 100.0 * scale, 100.0 * scale);

                    for (i, row) in colors.iter().enumerate() {
                        for (j, &color) in row.iter().enumerate() {
                            context.set_fill_style(&color.into());
                            context.fill_rect(
                                (j as f64) * scale,
                                (i as f64) * scale,
                                scale, scale,
                            );
                        }
                    }
                }
            };

            // Dessiner au démarrage
            draw();

            // Ajouter un écouteur d'événement pour la molette (zoom in/out)
            let listener = {
                let zoom_factor = zoom_factor.clone();  // Cloner zoom_factor pour l'utiliser dans le listener
                EventListener::new(&canvas, "wheel", move |event| {
                    let event = event.dyn_ref::<web_sys::WheelEvent>().unwrap();
                    let delta = event.delta_y();  // Récupérer le mouvement de la molette

                    // Ajuster le facteur de zoom
                    if delta > 0.0 {
                        zoom_factor.set(*zoom_factor * 0.9);  // Dézoomer
                    } else if delta < 0.0 {
                        zoom_factor.set(*zoom_factor * 1.1);  // Zoomer
                    }

                    draw();  // Redessiner le canvas après avoir ajusté le zoom
                })
            };

            // Nettoyer l'écouteur d'événements
            || drop(listener)
        });
    }

    html! {
        <div>
            <canvas ref={canvas_ref} width="1000" height="1000" style="border: 1px solid black;" />
        </div>
    }
}


fn main() {

    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App2>::new().render();
}