pub mod asciimath;
pub mod renderer;
pub mod text_canvas;


fn main() {
    let arg =std::env::args().last().unwrap().to_string();
    let rendered = asciimath::render(&arg);
    println!("{}", rendered);

}
