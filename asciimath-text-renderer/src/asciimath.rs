use phf::phf_map;

use crate::renderer::{Div, Drawable, Expr, Literal, Root, Sqrt, Stack};
use asciimath_parser;

static SYMBOLS: phf::Map<&'static str, &'static str> = phf_map! {
    //symbols taken from https://github.com/asciimath/asciimathml/blob/master/ASCIIMathML.js

    //greek letters
    "alpha" => "α",
    "beta" => "β",
    "chi" => "χ",
    "delta" => "δ",
    "Delta" => "Δ",
    "epsi" => "ε",
    "varepsilon" => "ɛ",
    "eta" => "η",
    "gamma" => "γ",
    "Gamma" => "Γ",
    "iota" => "ι",
    "kappa" => "κ",
    "lambda" => "λ",
    "Lambda" => "Λ",
    "lamda" => "λ",
    "Lamda" => "Λ",
    "mu" => "μ",
    "nu" => "ν",
    "omega" => "ω",
    "Omega" => "Ω",
    "phi" => "ϕ",
    "varphi" => "φ",
    "Phi" => "Φ",
    "pi" => "π",
    "Pi" => "Π",
    "psi" => "ψ",
    "Psi" => "Ψ",
    "rho" => "ρ",
    "sigma" => "σ",
    "Sigma" => "Σ",
    "tau" => "τ",
    "theta" => "θ",
    "vartheta" => "ϑ",
    "Theta" => "Θ",
    "upsilon" => "υ",
    "xi" => "ξ",
    "Xi" => "Ξ",
    "zeta" => "ζ",

    //binary operation symbols
    "*" => "⋅",
    "**" => "∗",
    "***" => "⋆",
    "//" => "/",
    "\\\\" => "\\",
    "setminus" => "\\",
    "xx" => "×",
    "|><" => "⋉",
    "><|" => "⋊",
    "|><|" => "⋈",
    "-:" => "÷",
    "divide" => "-:",
    "@" => "∘",
    "o+" => "⊕",
    "ox" => "⊗",
    "o." => "⊙",
    "sum" => "∑",
    "prod" => "∏",
    "^^" => "∧",
    "^^^" => "⋀",
    "vv" => "∨",
    "vvv" => "⋁",
    "nn" => "∩",
    "nnn" => "⋂",
    "uu" => "∪",
    "uuu" => "⋃",

    //binary relation symbols
    "!=" => "≠",
    ":=" => ":=",
    "lt" => "<",
    "<=" => "≤",
    "lt=" => "≤",
    "gt" => ">",
    "mlt" => "≪",
    ">=" => "≥",
    "gt=" => "≥",
    "mgt" => "≫",
    "-<" => "≺",
    "-lt" => "≺",
    ">-" => "≻",
    "-<=" => "⪯",
    ">-=" => "⪰",
    "in" => "∈",
    "!in" => "∉",
    "sub" => "⊂",
    "sup" => "⊃",
    "sube" => "⊆",
    "supe" => "⊇",
    "-=" => "≡",
    "~=" => "≅",
    "~~" => "≈",
    "~" => "∼",
    "prop" => "∝",

    //logical symbols
    "and" => "and",
    "or" => "or",
    "not" => "¬",
    "=>" => "⇒",
    "if" => "if",
    "<=>" => "⇔",
    "AA" => "∀",
    "EE" => "∃",
    "_|_" => "⊥",
    "TT" => "⊤",
    "|--" => "⊢",
    "|==" => "⊨",

    //grouping brackets
    "(" => "(",
    ")" => ")",
    "[" => "[",
    "]" => "]",
    "{" => "{",
    "}" => "}",
    "|" => "|",
    ":|:" => "|",
    "|:" => "|",
    ":|" => "|",
    "(:" => "〈",
    ":)" => "〉",
    "<<" => "〈",
    ">>" => "〉",
    "{:" => "{:",
    ":}" => ":}",

    //arrows
    "uarr" => "↑",
    "darr" => "↓",
    "rarr" => "→",
    "->" => "→",
    ">->" => "↣",
    "->>" => "↠",
    ">->>" => "⤖",
    "|->" => "↦",
    "larr" => "←",
    "harr" => "↔",
    "rArr" => "⇒",
    "lArr" => "⇐",
    "hArr" => "⇔",

    //miscellaneous symbols
    "int" => "∫",
    "dx" => "{:d x:}",
    "dy" => "{:d y:}",
    "dz" => "{:d z:}",
    "dt" => "{:d t:}",
    "oint" => "∮",
    "del" => "∂",
    "grad" => "∇",
    "+-" => "±",
    "-+" => "∓",
    "O/" => "∅",
    "oo" => "∞",
    "aleph" => "ℵ",
    "..." => "...",
    ":." => "∴",
    ":'" => "∵",
    "/_" => "∠",
    "/_\\" => "△",
    "'" => "′",
    "tilde" => "~",
    "\\ " => " ",
    "frown" => "⌢",
    "quad" => "  ",
    "qquad" => "    ",
    "cdots" => "⋯",
    "vdots" => "⋮",
    "ddots" => "⋱",
    "diamond" => "⋄",
    "square" => "□",
    "|__" => "⌊",
    "__|" => "⌋",
    "|~" => "⌈",
    "~|" => "⌉",
    "CC" => "ℂ",
    "NN" => "ℕ",
    "QQ" => "ℚ",
    "RR" => "ℝ",
    "ZZ" => "ℤ",
    "f" => "f",
    "g" => "g",

};

//bunch of visitors to map axiimath_parser hierarchy into tree of renderer structs
pub fn visit_simple(simple: &asciimath_parser::tree::Simple) -> Option<Box<dyn Drawable>> {
    match simple {
        asciimath_parser::tree::Simple::Missing => None,
        asciimath_parser::tree::Simple::Number(number) => Some(Box::new(Literal::new(number))),
        asciimath_parser::tree::Simple::Text(text) => Some(Box::new(Literal::new(text))),
        asciimath_parser::tree::Simple::Ident(ident) => Some(Box::new(Literal::new(ident))),
        asciimath_parser::tree::Simple::Symbol(symbol) => Some(Box::new(Literal::new(
            if let Some(s) = SYMBOLS.get(symbol) {
                s
            } else {
                symbol
            },
        ))),
        asciimath_parser::tree::Simple::Unary(unary) => match unary.op {
            "sqrt" => Some(Box::new(Sqrt::new(visit_simple(unary.arg()).unwrap()))),
            _ => unimplemented!(), //TODO: implementation and test for all unary functions
        },
        asciimath_parser::tree::Simple::Func(func) => None,
        asciimath_parser::tree::Simple::Binary(binary) => {
            match binary.op {
                "frac" => Some(Box::new(Div::new(
                    visit_simple(&binary.first()).unwrap(),
                    visit_simple(&binary.second()).unwrap(),
                ))),
                "stackrel" => Some(Box::new(Stack::new(
                    visit_simple(&binary.first()).unwrap(),
                    visit_simple(&binary.second()).unwrap(),
                ))),
                "root" => Some(Box::new(Root::new(
                    visit_simple(&binary.first()).unwrap(),
                    visit_simple(&binary.second()).unwrap(),
                ))),

                _ => unimplemented!(), //TODO: implementation and test for all binary functions
            }
        }
        asciimath_parser::tree::Simple::Group(group) => None,
        asciimath_parser::tree::Simple::Matrix(matrix) => None,
    }
}

pub fn visit_simple_script(
    simple_script: &asciimath_parser::tree::SimpleScript,
) -> Option<Box<dyn Drawable>> {
    //TODO: handle script
    if let Some(expr) = visit_simple(&simple_script.simple) {
        Some(expr)
    } else {
        //visit_script(&simple_script.script, drawables);
        None
    }
}

pub fn visit_func(script_func: &asciimath_parser::tree::Func) -> Option<Box<dyn Drawable>> {
    //func, script, arg
    None
}

pub fn visit_script_func(
    script_func: &asciimath_parser::tree::ScriptFunc,
) -> Option<Box<dyn Drawable>> {
    match script_func {
        asciimath_parser::tree::ScriptFunc::Simple(simple_script) => {
            visit_simple_script(&simple_script)
        }
        asciimath_parser::tree::ScriptFunc::Func(func) => visit_func(&func),
    }
}

pub fn visit_fraction(fraction: &asciimath_parser::tree::Frac) -> Option<Box<dyn Drawable>> {
    Some(Box::new(Div::new(
        visit_script_func(&fraction.numer).unwrap(),
        visit_script_func(&fraction.denom).unwrap(),
    )))
}

pub fn visit_expr(expr: &asciimath_parser::tree::Expression) -> Option<Box<dyn Drawable>> {
    let mut r_expr = Expr::new(vec![]);
    for e in expr.iter() {
        match e {
            asciimath_parser::tree::Intermediate::ScriptFunc(script_func) => {
                if let Some(_e) = visit_script_func(&script_func) {
                    r_expr.exprs.push(_e)
                }
            }
            asciimath_parser::tree::Intermediate::Frac(fraction) => {
                if let Some(_e) = visit_fraction(&fraction) {
                    r_expr.exprs.push(_e)
                }
            }
        }
    }
    Some(Box::new(r_expr))
}

pub fn render(expr: &str) -> String {
    //oddly, it doesn't return result, always parsing as something
    let parsed = asciimath_parser::parse(&expr);
    println!("{:#?}", parsed);
    let mut drawables: Vec<Box<dyn Drawable>> = vec![];
    let expr_opt = visit_expr(&parsed);

    if let Some(expr) = expr_opt {
        expr.to_string()
    } else {
        "".to_string()
    }
}
