use std::fmt::Debug;
use unicode_segmentation::UnicodeSegmentation;

use crate::text_canvas::TextCanvas;

//TODO: level for sqrt/root should match radicand level,
// for example: 1 + sqrt (-1^4/2)   "1" is rendered to high

#[derive(Clone, Debug, PartialEq)]
pub enum BracketType {
    None,
    LeftRound,   //"("
    RightRound,  //")"
    LeftSquare,  //"["
    RightSquare, //"]"
    LeftCurly,   //"{"
    RightCurly,  //"}"
    LeftAngled,  //"<"
    RightAngled, //">"
    Vertical,    //"￨"
}

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
    fn as_text(&self) -> String;
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

    fn as_text(&self) -> String {
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

    fn as_text(&self) -> String {
        self.to_canvas().as_text()
    }

    fn to_canvas(&self) -> TextCanvas {
        let mut result = TextCanvas::new(self.width(), self.height());

        let expr1_tc = self.expr1.to_canvas();
        let expr2_tc = self.expr2.to_canvas();

        result.draw(&expr1_tc, (self.width() - self.expr1.width() + 1) / 2, 0);
        result.draw(
            &expr2_tc,
            (self.width() - self.expr2.width() + 1) / 2,
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

    fn as_text(&self) -> String {
        self.to_canvas().as_text()
    }

    fn to_canvas(&self) -> TextCanvas {
        let mut result = TextCanvas::new(self.width(), self.height());

        let expr1_tc = self.expr1.to_canvas();
        let expr2_tc = self.expr2.to_canvas();

        result.draw(&expr1_tc, (self.width() - self.expr1.width() + 1) / 2, 0);
        result.draw(
            &expr2_tc,
            (self.width() - self.expr2.width() + 1) / 2,
            self.expr1.height(),
        );
        result
    }

    fn level(&self) -> usize {
        self.expr1.height() - 1
    }
}

pub fn bracket_width(bracket_type: &BracketType, expr_height: usize) -> usize {
    if expr_height == 0 {
        if *bracket_type == BracketType::None {
            return 0;
        } else {
            return 1;
        }
    }
    match bracket_type {
        BracketType::None => 0,
        BracketType::LeftRound | BracketType::RightRound => 1,
        BracketType::LeftSquare | BracketType::RightSquare => 1,
        BracketType::LeftCurly | BracketType::RightCurly => 1,
        BracketType::LeftAngled | BracketType::RightAngled => (expr_height + 1) / 2,
        BracketType::Vertical => 1,
    }
}

pub fn draw_simple_bracket(
    text_canvas: &mut TextCanvas,
    expr_height: usize,
    at_x: usize,
    at_y: usize,
    bracket: &str,
) {
    //expr_height >= 2
    for y_idx in 0..expr_height {
        text_canvas.set(at_x, at_y + y_idx, bracket);
    }
}

pub fn draw_long_bracket(
    text_canvas: &mut TextCanvas,
    expr_height: usize,
    at_x: usize,
    at_y: usize,
    top: &str,
    extension: &str,
    bottom: &str,
) {
    //expr_height >= 2
    let mut y = at_y;
    text_canvas.set(at_x, at_y, top);
    y += 1;
    for _ in 0..expr_height - 2 {
        text_canvas.set(at_x, y, extension);
        y += 1;
    }
    text_canvas.set(at_x, y, bottom);
}

pub fn draw_long_curly_bracket(
    text_canvas: &mut TextCanvas,
    expr_height: usize,
    at_x: usize,
    at_y: usize,
    top: &str,
    extension: &str,
    middle: &str,
    bottom: &str,
) {
    //expr_height >= 3
    let mut y = at_y;
    text_canvas.set(at_x, at_y, top);
    y += 1;
    for _ in 0..expr_height - 2 {
        if y == expr_height / 2 {
            text_canvas.set(at_x, y, middle);
        } else {
            text_canvas.set(at_x, y, extension);
        }
        y += 1;
    }
    text_canvas.set(at_x, y, bottom);
}

//Two different cases:
//expr_height % 2 == 0:
// ╲    upper
//  ╲   upper
//  ╱   lower
// ╱    lower
//
//expr_height % 2 == 1:
// ╲    upper
//  ╲   upper
//  🮥   middle
//  ╱   lower
// ╱    lower

pub fn draw_long_angled_bracket_left(
    text_canvas: &mut TextCanvas,
    expr_height: usize,
    at_x: usize,
    at_y: usize,
    upper: &str,
    middle: &str,
    lower: &str,
) {
    //expr_height >= 2
    let mut y = at_y;
    if expr_height % 2 == 0 {
        for idx in (0..(expr_height / 2)).rev() {
            text_canvas.set(at_x + idx, y, upper);
            y += 1;
        }
        for idx in 0..(expr_height / 2) {
            text_canvas.set(at_x + idx, y, lower);
            y += 1;
        }
    } else {
        for idx in (0..(expr_height / 2)).rev() {
            text_canvas.set(at_x + idx + 1, y, upper);
            y += 1;
        }

        text_canvas.set(at_x, y, middle);
        y += 1;
        for idx in 0..((expr_height) / 2) {
            text_canvas.set(at_x + idx + 1, y, lower);
            y += 1;
        }
    }
}

pub fn draw_long_angled_bracket_right(
    text_canvas: &mut TextCanvas,
    expr_height: usize,
    at_x: usize,
    at_y: usize,
    upper: &str,
    middle: &str,
    lower: &str,
) {
    //expr_height >= 2
    let mut y = at_y;
    if expr_height % 2 == 0 {
        for idx in 0..(expr_height / 2) {
            text_canvas.set(at_x + idx, y, upper);
            y += 1;
        }
        for idx in (0..(expr_height / 2)).rev() {
            text_canvas.set(at_x + idx, y, lower);
            y += 1;
        }
    } else {
        for idx in 0..(expr_height / 2) {
            text_canvas.set(at_x + idx, y, upper);
            y += 1;
        }

        text_canvas.set(at_x + y - 1, y, middle);
        y += 1;
        for idx in (0..((expr_height) / 2)).rev() {
            text_canvas.set(at_x + idx, y, lower);
            y += 1;
        }
    }
}

pub fn draw_bracket(
    text_canvas: &mut TextCanvas,
    bracket_type: &BracketType,
    expr_height: usize,
    at_x: usize,
    at_y: usize,
) {
    match bracket_type {
        BracketType::None => {}
        BracketType::LeftRound if expr_height <= 1 => text_canvas.set(at_x, at_y, "("),
        BracketType::LeftRound => {
            draw_long_bracket(text_canvas, expr_height, at_x, at_y, "⎛", "⎜", "⎝");
        }
        BracketType::RightRound if expr_height <= 1 => text_canvas.set(at_x, at_y, ")"),
        BracketType::RightRound => {
            draw_long_bracket(text_canvas, expr_height, at_x, at_y, "⎞", "⎜", "⎠");
        }
        BracketType::LeftSquare if expr_height <= 1 => text_canvas.set(at_x, at_y, "["),
        BracketType::LeftSquare => {
            draw_long_bracket(text_canvas, expr_height, at_x, at_y, "⎡", "⎥", "⎣");
        }
        BracketType::RightSquare if expr_height <= 1 => text_canvas.set(at_x, at_y, "]"),
        BracketType::RightSquare => {
            draw_long_bracket(text_canvas, expr_height, at_x, at_y, "⎤", "⎥", "⎦");
        }
        BracketType::LeftCurly if expr_height <= 1 => text_canvas.set(at_x, at_y, "{"),
        BracketType::LeftCurly if expr_height == 2 => {
            text_canvas.set(at_x, at_y, "⎰");
            text_canvas.set(at_x, at_y + 1, "⎱");
        }
        BracketType::LeftCurly => {
            draw_long_curly_bracket(text_canvas, expr_height, at_x, at_y, "⎧", "⎪", "⎨", "⎩");
        }
        BracketType::RightCurly if expr_height <= 1 => text_canvas.set(at_x, at_y, "}"),
        BracketType::RightCurly if expr_height == 2 => {
            text_canvas.set(at_x, at_y, "⎱");
            text_canvas.set(at_x, at_y + 1, "⎰");
        }
        BracketType::RightCurly => {
            draw_long_curly_bracket(text_canvas, expr_height, at_x, at_y, "⎫", "⎪", "⎬", "⎭");
        }
        BracketType::LeftAngled if expr_height <= 1 => text_canvas.set(at_x, at_y, "⟨"),

        BracketType::LeftAngled if expr_height == 2 => {
            text_canvas.set(at_x, at_y, "╱");
            text_canvas.set(at_x, at_y + 1, "╲");
        }
        BracketType::LeftAngled => {
            draw_long_angled_bracket_left(text_canvas, expr_height, at_x, at_y, "╱", "🮤", "╲");
        }
        BracketType::RightAngled if expr_height <= 1 => text_canvas.set(at_x, at_y, "⟩"),
        BracketType::RightAngled if expr_height == 2 => {
            text_canvas.set(at_x, at_y, "╲");
            text_canvas.set(at_x, at_y + 1, "╱");
        }
        BracketType::RightAngled => {
            draw_long_angled_bracket_right(text_canvas, expr_height, at_x, at_y, "╲", "🮥", "╱");
        }
        BracketType::Vertical => {
            draw_simple_bracket(text_canvas, expr_height, at_x, at_y, "￨");
        }
    }
}

//group -> render expression within brackets
//for example   (x)
#[derive(Debug)]
pub struct Group {
    left_bracket: BracketType,
    expr: Option<Box<dyn Drawable>>,
    right_bracket: BracketType,
}

impl Group {
    pub fn new(
        left_bracket: BracketType,
        expr: Option<Box<dyn Drawable>>,
        right_bracket: BracketType,
    ) -> Self {
        Group {
            left_bracket,
            expr,
            right_bracket,
        }
    }
}

impl Drawable for Group {
    fn width(&self) -> usize {
        bracket_width(
            &self.left_bracket,
            if let Some(expr) = &self.expr {
                expr.height()
            } else {
                0
            },
        ) + if let Some(expr) = &self.expr {
            expr.width()
        } else {
            0
        } + bracket_width(
            &self.right_bracket,
            if let Some(expr) = &self.expr {
                expr.height()
            } else {
                0
            },
        )
    }

    fn height(&self) -> usize {
        if let Some(expr) = &self.expr {
            expr.height()
        } else {
            1
        }
    }

    fn as_text(&self) -> String {
        self.to_canvas().as_text()
    }

    fn to_canvas(&self) -> TextCanvas {
        let mut result = TextCanvas::new(self.width(), self.height());
        let lbw = bracket_width(
            &self.left_bracket,
            if let Some(expr) = &self.expr {
                expr.height()
            } else {
                0
            },
        );
        let rbw = bracket_width(
            &self.right_bracket,
            if let Some(expr) = &self.expr {
                expr.height()
            } else {
                0
            },
        );
        let expr_width = if let Some(expr) = &self.expr {
            expr.width()
        } else {
            0
        };
        if lbw > 0 {
            draw_bracket(
                &mut result,
                &self.left_bracket,
                if let Some(expr) = &self.expr {
                    expr.height()
                } else {
                    0
                },
                0,
                0,
            );
        }
        if let Some(expr) = &self.expr {
            let expr_tc = expr.to_canvas();
            result.draw(&expr_tc, lbw, 0);
        }
        if rbw > 0 {
            draw_bracket(
                &mut result,
                &self.right_bracket,
                if let Some(expr) = &self.expr {
                    expr.height()
                } else {
                    0
                },
                lbw + expr_width,
                0,
            );
        }
        result
    }

    fn level(&self) -> usize {
        if let Some(expr) = &self.expr {
            //expr.level()
            expr.height() / 2
        } else {
            0
        }
    }
}

//matrix -> render matrix
//for example   ((a, b), (c, d))
//              [[a,b], [c,d]]
//TODO: support augmented matrices ((a,|),(b,|))
//TODO: align elements to level and middle
#[derive(Debug)]
pub struct Matrix {
    left_bracket: BracketType,
    exprs: Vec<Box<dyn Drawable>>,
    right_bracket: BracketType,
    num_colls: usize,
}

impl Matrix {
    pub fn new(
        left_bracket: BracketType,
        exprs: Vec<Box<dyn Drawable>>,
        right_bracket: BracketType,
        num_colls: usize,
    ) -> Self {
        Matrix {
            left_bracket,
            exprs,
            right_bracket,
            num_colls,
        }
    }

    ///Gather max width per column and max height per row
    ///for example:
    ///[a*2, c] gives [ (3,1), (1,1)]
    ///[b/2, d]       [ (3,3), (1,3)]
    pub fn max_sizes(&self) -> Vec<Vec<(usize, usize)>> {
        let num_rows = self.exprs.len() / self.num_colls;
        let mut data = vec![vec![(0, 0); self.num_colls]; num_rows];

        for row_idx in 0..num_rows {
            let max_row_height = (0..self.num_colls)
                .map(|coll_idx| self.exprs[row_idx * self.num_colls + coll_idx].height())
                .max()
                .unwrap();
            for coll_idx in 0..self.num_colls {
                data[row_idx][coll_idx].1 = max_row_height;
            }
        }
        for coll_idx in 0..self.num_colls {
            let max_col_width = (0..num_rows)
                .map(|row_idx| self.exprs[row_idx * self.num_colls + coll_idx].width())
                .max()
                .unwrap();
            for row_idx in 0..num_rows {
                data[row_idx][coll_idx].0 = max_col_width;
            }
        }
        data
    }
}

impl Drawable for Matrix {
    fn width(&self) -> usize {
        let _num_rows = self.exprs.len() / self.num_colls;
        let max_sizes = self.max_sizes();
        1 + (0..self.num_colls)
            .map(|coll_idx| max_sizes[0][coll_idx].0)
            .sum::<usize>()
            + (self.num_colls - 1)
            + 1
    }

    fn height(&self) -> usize {
        let num_rows = self.exprs.len() / self.num_colls;
        let max_sizes = self.max_sizes();
        (0..num_rows)
            .map(|row_idx| max_sizes[row_idx][0].1)
            .sum::<usize>()
            + (num_rows - 1)
    }

    fn as_text(&self) -> String {
        self.to_canvas().as_text()
    }

    fn to_canvas(&self) -> TextCanvas {
        let num_rows = self.exprs.len() / self.num_colls;
        let max_sizes = self.max_sizes();
        println!("sizes: {:?}", max_sizes);
        let mut result = TextCanvas::new(self.width(), self.height());
        draw_bracket(&mut result, &self.left_bracket, self.height(), 0, 0);
        let mut y = 0;
        for row_idx in 0..num_rows {
            let mut x = 1;
            let mut max_row_height = 0;
            for coll_idx in 0..self.num_colls {
                let tc = self.exprs[row_idx * self.num_colls + coll_idx].to_canvas();
                result.draw(&tc, x + (max_sizes[row_idx][coll_idx].0 - tc.width) / 2, y);
                x += max_sizes[row_idx][coll_idx].0 + 1;
                if tc.height > max_row_height {
                    max_row_height = tc.height
                }
            }
            y += max_row_height + 1;
        }
        draw_bracket(
            &mut result,
            &self.right_bracket,
            self.height(),
            self.width() - 1,
            0,
        );
        result
    }

    fn level(&self) -> usize {
        0
    }
}

//script -> render expression with sub or super script (or both)
//for example  3  x_0: x             2
//       2^3: 2         0   x_2^2:  x
//                                   1
#[derive(Debug)]
pub struct ScriptExpr {
    expr: Box<dyn Drawable>,
    sup_expr: Option<Box<dyn Drawable>>,
    sub_expr: Option<Box<dyn Drawable>>,
}

impl ScriptExpr {
    pub fn new(
        expr: Box<dyn Drawable>,
        sub_expr: Option<Box<dyn Drawable>>,
        sup_expr: Option<Box<dyn Drawable>>,
    ) -> Self {
        ScriptExpr {
            expr,
            sup_expr,
            sub_expr,
        }
    }
}

impl Drawable for ScriptExpr {
    fn width(&self) -> usize {
        self.expr.width()
            + match (&self.sup_expr, &self.sub_expr) {
                (Some(e), None) => e.width(),
                (None, Some(e)) => e.width(),
                (Some(e1), Some(e2)) => std::cmp::max(e1.width(), e2.width()),
                (None, None) => 0,
            }
    }

    fn height(&self) -> usize {
        self.expr.height()
            + match (&self.sup_expr, &self.sub_expr) {
                (Some(e), None) => e.height(),
                (None, Some(e)) => e.height(),
                (Some(e1), Some(e2)) => e1.height() + e2.height(),
                (None, None) => 0,
            }
    }

    fn as_text(&self) -> String {
        self.to_canvas().as_text()
    }

    fn to_canvas(&self) -> TextCanvas {
        let mut result = TextCanvas::new(self.width(), self.height());
        let expr_tc = self.expr.to_canvas();
        match (&self.sup_expr, &self.sub_expr) {
            (Some(e), None) => {
                let tc = e.to_canvas();
                result.draw(&tc, self.expr.width(), 0);
                result.draw(&expr_tc, 0, e.height());
                result
            }
            (None, Some(e)) => {
                let tc = e.to_canvas();
                result.draw(&expr_tc, 0, 0);
                result.draw(&tc, self.expr.width(), self.expr.height());
                result
            }
            (Some(e1), Some(e2)) => {
                let tc1 = e1.to_canvas();
                let tc2 = e2.to_canvas();
                result.draw(&tc1, self.expr.width(), 0);
                result.draw(&expr_tc, 0, e1.height());
                result.draw(&tc2, self.expr.width(), e1.height() + self.expr.height());
                result
            }
            (None, None) => {
                result.draw(&expr_tc, 0, 0);
                result
            }
        }
    }

    fn level(&self) -> usize {
        match (&self.sup_expr, &self.sub_expr) {
            (Some(e), Some(_)) | (Some(e), None) => self.expr.level() + e.height(),
            (None, Some(_)) | (None, None) => self.expr.level(),
        }
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

    fn as_text(&self) -> String {
        self.to_canvas().as_text()
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
/*

      ___
     /  1
   3/  ---
  \/    4

*/
#[derive(Debug)]
pub struct Root {
    index: Box<dyn Drawable>,
    radicand: Box<dyn Drawable>,
}

impl Root {
    pub fn new(index: Box<dyn Drawable>, radicand: Box<dyn Drawable>) -> Self {
        Root { index, radicand }
    }
}

impl Drawable for Root {
    fn width(&self) -> usize {
        let radical_symbol_height = (self.index.width() + 1) / 2;
        radical_symbol_height * 2 +    //((index width + 1)/2)*2
        if self.radicand.height() > radical_symbol_height { self.radicand.height() - radical_symbol_height } else { 0 }
        //self.index.height() + (self.index.height() + 1) / 2 - 1
        + self.radicand.width()
    }

    fn height(&self) -> usize {
        let radical_symbol_height = (self.index.width() + 1) / 2;
        std::cmp::max(
            self.index.height() + radical_symbol_height,
            self.radicand.height() + 1,
        )
    }

    fn as_text(&self) -> String {
        self.to_canvas().as_text()
    }

    fn to_canvas(&self) -> TextCanvas {
        let mut result = TextCanvas::new(self.width(), self.height());

        let index_tc = self.index.to_canvas();
        let radicand_tc = self.radicand.to_canvas();

        let radical_symbol_height = (self.index.width() + 1) / 2;
        result.draw(
            &index_tc,
            self.index.width() % 2,
            self.height() - self.index.height() - radical_symbol_height,
        );

        let mut x_idx = 0;

        for i in 0..radical_symbol_height {
            result.set(i, self.height() - radical_symbol_height + i, "╲");
            x_idx += 1;
        }
        for i in 0..radical_symbol_height {
            result.set(x_idx, self.height() - i - 1, "╱");
            x_idx += 1;
        }
        let mut top_line_level = self.height() - radical_symbol_height;
        if self.radicand.height() > radical_symbol_height {
            for i in 0..(self.radicand.height() - radical_symbol_height) {
                result.set(x_idx, self.height() - radical_symbol_height - i - 1, "╱");
                top_line_level = self.height() - radical_symbol_height - i - 1;
                x_idx += 1;
            }
        }
        for _ in 0..radicand_tc.width {
            result.set(x_idx, top_line_level - 1, "▁");
            x_idx += 1;
        }

        result.draw(
            &radicand_tc,
            self.width() - self.radicand.width(),
            (top_line_level) + (self.height() - top_line_level - self.radicand.height() + 1) / 2,
        );
        result
    }

    fn level(&self) -> usize {
        self.height() - (self.index.height() + 1) / 2
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
        if self.exprs.is_empty() {
            0
        } else {
            let level = self.level();
            self.exprs
                .iter()
                .map(|e| level + (e.height() - e.level() - 1))
                .max()
                .unwrap()
                + 1
        }
    }

    fn as_text(&self) -> String {
        self.to_canvas().as_text()
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
        assert_eq!(&l.as_text(), "abc");
    }

    #[test]
    fn test_div() {
        let l1 = Literal::new("1");
        let l2 = Literal::new("2");
        let div = Div::new(Box::new(l1), Box::new(l2));

        assert_eq!(div.width(), 3);
        assert_eq!(div.height(), 3);
        assert_eq!(&div.as_text(), " 1 \n───\n 2 ");
    }

    #[test]
    fn test_stack() {
        let l1 = Literal::new("1");
        let l2 = Literal::new("2");
        let stack = Stack::new(Box::new(l1), Box::new(l2));

        assert_eq!(stack.width(), 1);
        assert_eq!(stack.height(), 2);
        assert_eq!(&stack.as_text(), "1\n2");
    }

    #[test]
    fn test_sqrt() {
        let l1 = Literal::new("1");
        let sqrt = Sqrt::new(Box::new(l1.clone()));

        assert_eq!(sqrt.width(), 3);
        assert_eq!(sqrt.height(), 2);
        assert_eq!(&sqrt.as_text(), "  ▁\n╲╱1");

        let l2 = Literal::new("2");
        let div = Div::new(Box::new(l1.clone()), Box::new(l2.clone()));
        let sqrt = Sqrt::new(Box::new(div));
        assert_eq!(sqrt.width(), 8);
        assert_eq!(sqrt.height(), 4);
        assert_eq!(&sqrt.as_text(), "     ▁▁▁\n    ╱ 1 \n╲  ╱ ───\n ╲╱   2 ");
    }

    #[test]
    fn test_expression() {
        let expr = Expr::new(vec![]);
        assert_eq!(&expr.as_text(), "");
        let l1 = Literal::new("a");
        let expr = Expr::new(vec![Box::new(l1.clone())]);
        assert_eq!(&expr.as_text(), "a");
        let l2 = Literal::new("b");
        let expr = Expr::new(vec![Box::new(l1), Box::new(l2)]);
        assert_eq!(&expr.as_text(), "ab");
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
                    verify(example_name, example_asciimath, &example.join("\n"));
                    example_name = "";
                    example.clear();
                }
                example_name = line[2..].trim();
                mode = "example_asciimath";
            } else if line.starts_with('#') || line.is_empty() {
                if mode == "example" {
                    verify(example_name, example_asciimath, &example.join("\n"));
                    example_name = "";
                    example.clear();
                }
                mode = ""
            } else if mode == "example" {
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
