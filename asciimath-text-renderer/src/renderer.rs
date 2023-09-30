use std::fmt::Debug;
use unicode_segmentation::UnicodeSegmentation;

use crate::text_canvas::TextCanvas;

pub trait Drawable: Debug {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    /*where the horizontal middle of expression is
     so that we know where to draw it
     provided in relation to top of the expression
     1     _
     - + \/2   <- level
     2
    */
    fn level(&self) -> usize;
    fn to_string(&self) -> String;
    fn to_canvas(&self) -> TextCanvas;
}

#[derive(Clone, Debug)]
//hold vec of graphemes, which will be simply displayed as literals
//in other words, it's a line of text
pub struct Literal {
    value: Vec<String>,
}

impl Literal {
    pub fn new(from_str: &str) -> Self {
        Literal {
            value: from_str.graphemes(true).map(|s| s.to_string()).collect(),
        }
    }
}

impl Drawable for Literal {
    fn width(&self) -> usize {
        self.value.len()
    }
    fn height(&self) -> usize {
        1
    }
    fn to_string(&self) -> String {
        self.value.join("")
    }
    fn to_canvas(&self) -> TextCanvas {
        let mut tc = TextCanvas::new(self.value.len(), 1);
        for (idx, s) in self.value.iter().enumerate() {
            tc.set(idx, 0, s);
        }
        tc
    }

    fn level(&self) -> usize {
        0
    }
}

#[derive(Debug)]
pub struct Div {
    expr1: Box<dyn Drawable>,
    expr2: Box<dyn Drawable>,
}

impl Div {
    pub fn new(expr1: Box<dyn Drawable>, expr2: Box<dyn Drawable>) -> Self {
        Div { expr1, expr2 }
    }
}

impl Drawable for Div {
    fn width(&self) -> usize {
        std::cmp::max(self.expr1.width(), self.expr2.width()) + 2
    }

    fn height(&self) -> usize {
        self.expr1.height() + self.expr2.height() + 1
    }

    fn to_string(&self) -> String {
        self.to_canvas().to_string()
    }

    fn to_canvas(&self) -> TextCanvas {
        let mut result = TextCanvas::new(self.width(), self.height());

        let expr1_tc = self.expr1.to_canvas();
        let expr2_tc = self.expr2.to_canvas();

        result.draw(
            &expr1_tc,
            ((self.width() - self.expr1.width()) as f64 / 2.0f64) as usize,
            0,
        );
        result.draw(
            &expr2_tc,
            ((self.width() - self.expr1.width()) as f64 / 2.0f64) as usize,
            self.expr1.height() + 1,
        );
        for idx in 0..self.width() {
            result.set(idx, self.expr1.height(), "─")
        }

        result
    }

    fn level(&self) -> usize {
        self.expr1.height()
    }
}

//stack (stackrel) -> stack expr1 over expr2
//stackrel a b =>  a
//                 b
#[derive(Debug)]
pub struct Stack {
    expr1: Box<dyn Drawable>,
    expr2: Box<dyn Drawable>,
}

impl Stack {
    pub fn new(expr1: Box<dyn Drawable>, expr2: Box<dyn Drawable>) -> Self {
        Stack { expr1, expr2 }
    }
}

impl Drawable for Stack {
    fn width(&self) -> usize {
        std::cmp::max(self.expr1.width(), self.expr2.width())
    }

    fn height(&self) -> usize {
        self.expr1.height() + self.expr2.height()
    }

    fn to_string(&self) -> String {
        self.to_canvas().to_string()
    }

    fn to_canvas(&self) -> TextCanvas {
        let mut result = TextCanvas::new(self.width(), self.height());

        let expr1_tc = self.expr1.to_canvas();
        let expr2_tc = self.expr2.to_canvas();

        result.draw(
            &expr1_tc,
            ((self.width() - self.expr1.width()) as f64 / 2.0f64) as usize,
            0,
        );
        result.draw(
            &expr2_tc,
            ((self.width() - self.expr1.width()) as f64 / 2.0f64) as usize,
            self.expr1.height(),
        );
        result
    }

    fn level(&self) -> usize {
        self.expr1.height() - 1
    }
}

//square root
#[derive(Debug)]
pub struct Sqrt {
    expr: Box<dyn Drawable>,
}

impl Sqrt {
    pub fn new(expr: Box<dyn Drawable>) -> Self {
        Sqrt { expr }
    }
}

impl Drawable for Sqrt {
    fn width(&self) -> usize {
        self.expr.width() + self.expr.height() + ((self.expr.height() as f64 * 0.5 + 0.5) as usize)
    }

    fn height(&self) -> usize {
        self.expr.height() + 1
    }

    fn to_string(&self) -> String {
        self.to_canvas().to_string()
    }

    fn to_canvas(&self) -> TextCanvas {
        let mut result = TextCanvas::new(self.width(), self.height());
        let expr_tc = self.expr.to_canvas();
        result.draw(&expr_tc, self.width() - self.expr.width(), 1);

        let mut idx = 0;
        let m = (self.expr.height() as f64 * 0.5 + 0.5) as usize;
        for (pos, i) in (0..m).rev().enumerate() {
            result.set(pos, self.height() - i - 1, "╲");
            idx += 1;
        }
        for (pos, i) in (idx..(idx + self.expr.height())).enumerate() {
            result.set(i, self.height() - pos - 1, "╱");
            idx += 1
        }
        for i in idx..(idx + self.expr.width()) {
            result.set(i, 0, "▁")
        }

        result
    }

    fn level(&self) -> usize {
        (self.expr.height() + 1) / 2
    }
}

//root
#[derive(Debug)]
pub struct Root {
    arg1: Box<dyn Drawable>,
    arg2: Box<dyn Drawable>,
}

impl Root {
    pub fn new(arg1: Box<dyn Drawable>, arg2: Box<dyn Drawable>) -> Self {
        Root { arg1, arg2 }
    }
}

//size of character of V part of root
//to wrap half of first argument
fn v_size(w: usize, h: usize) -> (usize, usize) {
    ((h + 1) / 2 + (h + 1) / 2, (w + 1) / 2 + (h + 1) / 2)
}

impl Drawable for Root {
    fn width(&self) -> usize {
        //self.arg1.width() + self.expr.height() + ((self.expr.height() as f64 * 0.5 + 0.5) as usize)
        ((self.arg1.width() + 1) / 2) * 2 + self.arg1.height() + (self.arg1.height() + 1) / 2 - 1
            + self.arg2.width()
            - 1
        //TODO: account for arg2 wuth height larger than arg1
        //self.arg1.width() + self.arg2.width()
    }

    fn height(&self) -> usize {
        self.arg1.height() + (self.arg1.width() + 1) / 2
    }

    fn to_string(&self) -> String {
        self.to_canvas().to_string()
    }

    fn to_canvas(&self) -> TextCanvas {
        let mut result = TextCanvas::new(self.width(), self.height());

        let arg1_tc = self.arg1.to_canvas();
        let arg2_tc = self.arg2.to_canvas();

        result.draw(&arg1_tc, arg1_tc.width % 2, 0);

        let mut x_idx = 0;
        let box_h_2 = (self.arg1.height() + 1) / 2;
        let box_w_2 = (self.arg1.width() + 1) / 2;
        for i in (0..box_w_2) {
            result.set(i, arg1_tc.height + i, "╲");
            x_idx += 1;
        }
        for i in 0..box_w_2 {
            result.set(x_idx, self.height() - i - 1, "╱");
            x_idx += 1;
        }
        let arg2_pos = x_idx;

        for _ in 0..arg2_tc.width {
            result.set(x_idx, box_h_2 - 1, "▁");
            x_idx += 1;
        }

        result.draw(&arg2_tc, arg2_pos, box_h_2);

        result
    }

    fn level(&self) -> usize {
        (self.arg1.height() + 1) / 2
    }
}

//Expression holding a row of items
#[derive(Debug)]
pub struct Expr {
    pub exprs: Vec<Box<dyn Drawable>>,
}

impl Expr {
    pub fn new(exprs: Vec<Box<dyn Drawable>>) -> Self {
        Expr { exprs }
    }
}

impl Drawable for Expr {
    fn width(&self) -> usize {
        self.exprs.iter().map(|e| e.width()).sum()
    }

    fn height(&self) -> usize {
        if self.exprs.len() == 0 {
            0
        } else {
            let level = self.level();
            self.exprs
                .iter()
                .map(|e| level + e.height() - level)
                .max()
                .unwrap()
        }
    }

    fn to_string(&self) -> String {
        self.to_canvas().to_string()
    }

    fn to_canvas(&self) -> TextCanvas {
        let mut result = TextCanvas::new(self.width(), self.height());
        let mut idx = 0;
        let level = self.level();
        for expr in &self.exprs {
            result.draw(&expr.to_canvas(), idx, level - expr.level());
            idx += expr.width();
        }

        result
    }

    fn level(&self) -> usize {
        self.exprs.iter().map(|e| e.level()).max().unwrap_or(0)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::asciimath;
    use std::fs::read_to_string;

    #[test]
    fn test_literal() {
        let l = Literal::new("abc");
        assert_eq!(l.width(), 3);
        assert_eq!(l.height(), 1);
        assert_eq!(&l.to_string(), "abc");
    }

    #[test]
    fn test_div() {
        let l1 = Literal::new("1");
        let l2 = Literal::new("2");
        let div = Div::new(Box::new(l1), Box::new(l2));

        assert_eq!(div.width(), 3);
        assert_eq!(div.height(), 3);
        assert_eq!(&div.to_string(), " 1 \n───\n 2 ");
    }

    #[test]
    fn test_stack() {
        let l1 = Literal::new("1");
        let l2 = Literal::new("2");
        let stack = Stack::new(Box::new(l1), Box::new(l2));

        assert_eq!(stack.width(), 1);
        assert_eq!(stack.height(), 2);
        assert_eq!(&stack.to_string(), "1\n2");
    }

    #[test]
    fn test_sqrt() {
        let l1 = Literal::new("1");
        let sqrt = Sqrt::new(Box::new(l1.clone()));

        assert_eq!(sqrt.width(), 3);
        assert_eq!(sqrt.height(), 2);
        assert_eq!(&sqrt.to_string(), "  ▁\n╲╱1");

        let l2 = Literal::new("2");
        let div = Div::new(Box::new(l1.clone()), Box::new(l2.clone()));
        let sqrt = Sqrt::new(Box::new(div));
        assert_eq!(sqrt.width(), 8);
        assert_eq!(sqrt.height(), 4);
        assert_eq!(&sqrt.to_string(), "     ▁▁▁\n    ╱ 1 \n╲  ╱ ───\n ╲╱   2 ");
    }

    #[test]
    fn test_expression() {
        let expr = Expr::new(vec![]);
        assert_eq!(&expr.to_string(), "");
        let l1 = Literal::new("a");
        let expr = Expr::new(vec![Box::new(l1.clone())]);
        assert_eq!(&expr.to_string(), "a");
        let l2 = Literal::new("b");
        let expr = Expr::new(vec![Box::new(l1), Box::new(l2)]);
        assert_eq!(&expr.to_string(), "ab");
    }

    #[test]
    fn test_examples_from_test_file() {
        fn verify(example_name: &str, asciimath_str: &str, expected: &str) {
            let rendered = asciimath::render(asciimath_str);
            assert_eq!(
                rendered, expected,
                "testing example {}: {}\nexpected output:\n{}\n\nrendered output:\n{}\n\n",
                example_name, asciimath_str, expected, rendered
            );
        }
        let mut mode = "";
        let mut example_name = "";
        let mut example_asciimath = "";
        let mut example: Vec<String> = vec![];
        for line in read_to_string("tests.txt").unwrap().lines() {
            if line.starts_with("##") {
                if mode == "example" {
                    verify(&example_name, &example_asciimath, &example.join("\n"));
                    example_name = "";
                    example.clear();
                }
                example_name = line[2..].trim();
                mode = "example_asciimath";
            } else if line.starts_with("#") || line == "" {
                if mode == "example" {
                    verify(&example_name, &example_asciimath, &example.join("\n"));
                    example_name = "";
                    example.clear();
                }
                mode = ""
            } else {
                if mode == "example" {
                    example.push(line.to_string());
                } else if mode == "example_asciimath" {
                    example_asciimath = line;
                    mode = "example"
                } else {
                    panic!("invalid test file");
                }
            }
        }
    }
}
