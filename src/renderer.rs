use unicode_segmentation::UnicodeSegmentation;

use crate::text_canvas::TextCanvas;

pub trait Drawable {

    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn to_string(&self) -> String;
    fn to_canvas(&self) -> TextCanvas;

}


#[derive(Clone, Debug)]
//hold vec of graphemes, which will be simply displayed as literals
//in other words, it's a line of text
pub struct Literal {
    value: Vec<String>
}
//hold single grapheme
impl Literal {
    pub fn new(from_str: &str) -> Self {
        Literal { value: from_str.graphemes(true).map(|s| s.to_string()).collect() }
    }
}

impl Drawable for Literal {
    fn width(&self) -> usize { self.value.len() }
    fn height(&self) -> usize { 1 }
    fn to_string(&self) -> String { self.value.join("") }
    fn to_canvas(&self) -> TextCanvas {
        let mut tc = TextCanvas::new(self.value.len(), 1);
        for (idx, s) in self.value.iter().enumerate() {
            tc.set(idx, 0, s);
        }
        tc
    }
}


pub struct Div<'a> {
    expr1: &'a dyn Drawable,
    expr2: &'a dyn Drawable,
}

impl <'a>Div<'a> {

    pub fn new(expr1: &'a dyn Drawable, expr2: &'a dyn Drawable) -> Self {
        Div {
            expr1, expr2
        }
    }
}

impl <'a>Drawable for Div<'a> {

    fn width(&self) -> usize {
        std::cmp::max(self.expr1.width(), self.expr2.width()) + 2

    }

    fn height(&self) -> usize {
        self.expr1.height() + self.expr2.height() + 1

    }
  
    fn to_string(&self) -> String { self.to_canvas().to_string() }

    fn to_canvas(&self) -> TextCanvas {

        let mut result = TextCanvas::new(self.width(), self.height());

        let expr1_tc = self.expr1.to_canvas();
        let expr2_tc = self.expr2.to_canvas();

        result.draw(&expr1_tc, ((self.width() - self.expr1.width()) as f64 /2.0f64) as usize, 0);
        result.draw(&expr2_tc, ((self.width() - self.expr1.width()) as f64 /2.0f64) as usize, self.expr1.height() + 1);
        for idx in 0..self.width() {
            result.set(idx, self.expr1.height(), "─")
        }

        result
    }

}

//square root
pub struct Sqrt<'a> {
    expr: &'a dyn Drawable,
}

impl <'a>Sqrt<'a> {

    pub fn new(expr: &'a dyn Drawable) -> Self {
        Sqrt {
            expr
        }
    }
}

impl <'a>Drawable for Sqrt<'a> {

    fn width(&self) -> usize {
        self.expr.width() + self.expr.height() + ((self.expr.height() as f64 * 0.5 + 0.5) as usize)
    }

    fn height(&self) -> usize {
        self.expr.height() + 1
    }
  
    fn to_string(&self) -> String { self.to_canvas().to_string() }

    fn to_canvas(&self) -> TextCanvas {

        let mut result = TextCanvas::new(self.width(), self.height());
        let expr_tc = self.expr.to_canvas();
        result.draw(&expr_tc, self.width() - self.expr.width(), 1);

        let mut idx = 0;
        let m = (self.expr.height() as f64 * 0.5 + 0.5) as usize;
        println!("m is {}", m);
        for (pos, i) in (0..m).rev().enumerate() {
            result.set(pos, self.height()-i-1, "╲");
            idx += 1;
        }
        for (pos,i) in (idx..(idx+self.expr.height())).enumerate() {
            result.set(i, self.height()-pos-1, "╱");
            idx += 1
        }
        for i in idx..(idx + self.expr.width()) {
            result.set(i, 0, "▁")
        }

        result
    }

}



#[cfg(test)]
mod test {

    use std::fs::read_to_string;
    use super::*;

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
        let div = Div::new(&l1, &l2);

        assert_eq!(div.width(), 3);
        assert_eq!(div.height(), 3);
        assert_eq!(&div.to_string(), " 1 \n───\n 2 ");
    }

    #[test]
    fn test_sqrt() {
        let l1 = Literal::new("1");
        let sqrt = Sqrt::new(&l1);

        assert_eq!(sqrt.width(), 3);
        assert_eq!(sqrt.height(), 2);
        assert_eq!(&sqrt.to_string(), "  ▁\n╲╱1");


        let l2 = Literal::new("2");
        let div = Div::new(&l1, &l2);
        let sqrt = Sqrt::new(&div);
        assert_eq!(sqrt.width(), 8);
        assert_eq!(sqrt.height(), 4);
        assert_eq!(&sqrt.to_string(), "     ▁▁▁\n    ╱ 1 \n╲  ╱ ───\n ╲╱   2 ");

    }

    #[test]
    fn test_examples_from_test_file() {
        fn verify(example_name: &str, expected: &Vec<String>) {
            //TODO: parse and compare
            //println!("V:: {} for {:?}", example_name, expected);
        }
        let mut mode = "";
        let mut example_name = "";
        let mut example: Vec<String> = vec![];
        for line in read_to_string("tests.txt").unwrap().lines() 
        {
            if line.starts_with("##") {
                if mode == "example" {
                    verify(&example_name, &example);
                    example_name = "";
                    example.clear();
                }
                example_name = line[2..].trim();
                mode = "example";
            } else if line.starts_with("#") || line == "" {
                if mode == "example" {
                    verify(&example_name, &example);
                    example_name = "";
                    example.clear();
                }
                mode = ""
            } else {
                if mode == "example" {
                    example.push(line.to_string());
                } else {
                    panic!("invalid test file");
                }
            }
        }




        
    }
}
