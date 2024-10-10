use yew::prelude::*;
use log::info;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use gloo::events::EventListener;
use stylist::yew::use_style;
use yew::{classes, html};

// Utilise Supabase comme backen

#[derive(Debug, Clone, Copy, PartialEq)]
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


#[derive(Properties, PartialEq)]
struct ColorSelectorProps {
    color: Color,
    on_click: Callback<Color>,
}

#[function_component(ColorSelectorComponent)]
fn color_selector(props: &ColorSelectorProps) -> Html {
    let on_click = props.on_click.clone();

    let selected_color = use_state(|| Color::White);

    let change_color = {
        let selected_color = selected_color.clone();
        move |color: Color| {
            selected_color.set(color);
        }
    };

    let colors = vec![
        (Color::Red, "Red", "background-color: red;"),
        (Color::Green, "Green", "background-color: green;"),
        (Color::Blue, "Blue", "background-color: blue;"),
        (Color::Yellow, "Yellow", "background-color: yellow;"),
        (Color::Black, "Black", "background-color: black; color: white;"),
        (Color::White, "White", "background-color: white;"),
    ];

    html! {
        <div style="display: flex; flex-direction: raw; padding: 10px;">

            <div style="margin-top: 20px;">
                { format!("Selected color: {:?}", *selected_color) }
            </div>

            { for colors.into_iter().map(|(color, color_name, style)| {
                let onclick = {
                    let color = color.clone();
                    let change_color = change_color.clone();
                    Callback::from(move |_| change_color(color.clone()))
                };
                let button_style: String = format!("{} width: 50px; height: 50px;  margin: 5px; border: 1px solid black; border-radius: 50%;", style);
                html! {
                    <button {onclick} style={button_style}>
                        // { color_name }
                    </button>
                }
            })}
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
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
            <canvas ref={canvas_ref} width="800" height="800" style="border: 1px solid black;" />
            <div style="position: absolute; bottom: 10px; right: 10px; border: 1px solid black; border-radius: 40px;">
                <ColorSelectorComponent color={Color::Red} on_click={|color| { info!("Color: {:?}", color); }} />
            </div>
        </div>
    }
}


fn main() {

    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}