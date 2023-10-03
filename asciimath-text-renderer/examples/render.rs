extern crate asciimath_text_renderer;

fn main() {
    let arg = std::env::args().last().unwrap().to_string();
    let rendered = asciimath_text_renderer::render(&arg);
    println!("{}", rendered);
}
