use phf::phf_map;

use crate::renderer::{
    BracketType, Div, Drawable, Expr, Group, Literal, Matrix, Root, ScriptExpr, Sqrt, Stack,
};


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

//ident substitutions - mostly to add some spacing
static IDENTS: phf::Map<&'static str, &'static str> = phf_map! {
    //"+" => " + ",
    //"-" => " - ",
    //"=" => " = ",
};

pub fn bracket_type(bracket: &str) -> BracketType {
    match bracket {
        "(" => BracketType::LeftRound,
        ")" => BracketType::RightRound,
        "[" => BracketType::LeftSquare,
        "]" => BracketType::RightSquare,
        "{" => BracketType::LeftCurly,
        "}" => BracketType::RightCurly,
        "<<" | "(:" => BracketType::LeftAngled,
        ">>" | ":)" => BracketType::RightAngled,
        "{:" => BracketType::None,
        ":}" => BracketType::None,
        "|" => BracketType::Vertical,
        _ => BracketType::None, //TODO: panic?
    }
}

//bunch of visitors to map axiimath_parser hierarchy into tree of renderer structs
pub fn visit_simple(
    simple: &asciimath_parser::tree::Simple,
    omit_braces: bool,
) -> Option<Box<dyn Drawable>> {
    match simple {
        asciimath_parser::tree::Simple::Missing => None,
        asciimath_parser::tree::Simple::Number(number) => Some(Box::new(Literal::new(number))),
        asciimath_parser::tree::Simple::Text(text) => Some(Box::new(Literal::new(text))),
        asciimath_parser::tree::Simple::Ident(ident) => {
            Some(Box::new(Literal::new(if let Some(s) = IDENTS.get(ident) {
                s
            } else {
                ident
            })))
        }
        asciimath_parser::tree::Simple::Symbol(symbol) => Some(Box::new(Literal::new(
            if let Some(s) = SYMBOLS.get(symbol) {
                s
            } else {
                symbol
            },
        ))),
        asciimath_parser::tree::Simple::Unary(unary) => match unary.op {
            "sqrt" => Some(Box::new(Sqrt::new(
                visit_simple(unary.arg(), true).unwrap(),
            ))),
            "bar" => unimplemented!(),
            "hat" => unimplemented!(),
            "ul" => unimplemented!(),
            "vec" => unimplemented!(),
            "tilde" => unimplemented!(),
            "dot" => unimplemented!(),
            "ddot" => unimplemented!(),
            _ => unimplemented!(), //TODO: implementation and test for all unary functions
        },
        asciimath_parser::tree::Simple::Func(_func) => None, //TODO: handle func
        asciimath_parser::tree::Simple::Binary(binary) => {
            match binary.op {
                "frac" => Some(Box::new(Div::new(
                    visit_simple(binary.first(), true).unwrap(),
                    visit_simple(binary.second(), true).unwrap(),
                ))),
                "stackrel" => Some(Box::new(Stack::new(
                    visit_simple(binary.first(), true).unwrap(),
                    visit_simple(binary.second(), true).unwrap(),
                ))),
                "root" => Some(Box::new(Root::new(
                    visit_simple(binary.first(), true).unwrap(),
                    visit_simple(binary.second(), true).unwrap(),
                ))),

                _ => unimplemented!(), //TODO: implementation and test for all binary functions
            }
        }
        asciimath_parser::tree::Simple::Group(group) => {
            let rendered_expr = visit_expr(&group.expr); //can be empoty, ie. "f()"
            Some(Box::new(Group::new(
                if omit_braces {
                    BracketType::None
                } else {
                    bracket_type(group.left_bracket)
                },
                rendered_expr,
                if omit_braces {
                    BracketType::None
                } else {
                    bracket_type(group.right_bracket)
                },
            )))
        }
        asciimath_parser::tree::Simple::Matrix(matrix) => {
            let mut exprs: Vec<Box<dyn Drawable>> = vec![];
            matrix.rows().for_each(|row| {
                for e in row {
                    exprs.push(visit_expr(e).unwrap());
                }
            });
            Some(Box::new(Matrix::new(
                bracket_type(matrix.left_bracket),
                exprs,
                bracket_type(matrix.right_bracket),
                matrix.num_cols(),
            )))
        } //TODO: handle matrix
    }
}

pub fn visit_simple_script(
    simple_script: &asciimath_parser::tree::SimpleScript,
    omit_braces: bool,
) -> Option<Box<dyn Drawable>> {
    if let Some(expr) = visit_simple(&simple_script.simple, omit_braces) {
        match &simple_script.script {
            asciimath_parser::tree::Script::None => Some(expr),
            asciimath_parser::tree::Script::Sub(simple) => {
                let sub_expr = visit_simple(simple, true).unwrap();
                Some(Box::new(ScriptExpr::new(expr, Some(sub_expr), None)))
            }
            asciimath_parser::tree::Script::Super(simple) => {
                let sup_expr = visit_simple(simple, true).unwrap();
                Some(Box::new(ScriptExpr::new(expr, None, Some(sup_expr))))
            }
            asciimath_parser::tree::Script::Subsuper(simple1, simple2) => {
                let sub_expr = visit_simple(simple1, true).unwrap();
                let sup_expr = visit_simple(simple2, true).unwrap();
                Some(Box::new(ScriptExpr::new(
                    expr,
                    Some(sub_expr),
                    Some(sup_expr),
                )))
            }
        }
    } else {
        None
    }
}

pub fn visit_func(func: &asciimath_parser::tree::Func) -> Option<Box<dyn Drawable>> {
    let arg = visit_script_func(func.arg(), false);
    let func_lit = Box::new(Literal::new(func.func));

    let func_expr: Option<Box<dyn Drawable>> = match &func.script {
        asciimath_parser::tree::Script::None => Some(func_lit),
        asciimath_parser::tree::Script::Sub(simple) => {
            let sub_expr = visit_simple(simple, true).unwrap();
            Some(Box::new(ScriptExpr::new(func_lit, Some(sub_expr), None)))
        }
        asciimath_parser::tree::Script::Super(simple) => {
            let sup_expr = visit_simple(simple, true).unwrap();
            Some(Box::new(ScriptExpr::new(func_lit, None, Some(sup_expr))))
        }
        asciimath_parser::tree::Script::Subsuper(simple1, simple2) => {
            let sub_expr = visit_simple(simple1, false).unwrap();
            let sup_expr = visit_simple(simple2, false).unwrap();
            Some(Box::new(ScriptExpr::new(
                func_lit,
                Some(sub_expr),
                Some(sup_expr),
            )))
        }
    };
    Some(Box::new(Expr::new(vec![func_expr.unwrap(), arg.unwrap()])))
}

pub fn visit_script_func(
    script_func: &asciimath_parser::tree::ScriptFunc,
    omit_braces: bool,
) -> Option<Box<dyn Drawable>> {
    match script_func {
        asciimath_parser::tree::ScriptFunc::Simple(simple_script) => {
            visit_simple_script(simple_script, omit_braces)
        }
        asciimath_parser::tree::ScriptFunc::Func(func) => visit_func(func),
    }
}

pub fn visit_fraction(fraction: &asciimath_parser::tree::Frac) -> Option<Box<dyn Drawable>> {
    Some(Box::new(Div::new(
        visit_script_func(&fraction.numer, true).unwrap(),
        visit_script_func(&fraction.denom, true).unwrap(),
    )))
}

pub fn visit_expr(expr: &asciimath_parser::tree::Expression) -> Option<Box<dyn Drawable>> {
    let mut r_expr = Expr::new(vec![]);
    for e in expr.iter() {
        match e {
            asciimath_parser::tree::Intermediate::ScriptFunc(script_func) => {
                if let Some(_e) = visit_script_func(script_func, false) {
                    r_expr.exprs.push(_e)
                }
            }
            asciimath_parser::tree::Intermediate::Frac(fraction) => {
                if let Some(_e) = visit_fraction(fraction) {
                    r_expr.exprs.push(_e)
                }
            }
        }
    }
    Some(Box::new(r_expr))
}

pub fn render(expr: &str) -> String {
    //oddly, it doesn't return result, always parsing as something
    let parsed = asciimath_parser::parse(expr);
    //println!("{:#?}", parsed);
    let expr_opt = visit_expr(&parsed);

    //println!("{:#?}", expr_opt);
    if let Some(expr) = expr_opt {
        expr.as_text()
    } else {
        "".to_string()
    }
}
