//TextCanvas is two-dimenstional array
//For example to represent 1/2 * 3/4, we would think of it as:
//1 3
//-*-
//2 4
//creating array 3x3. However as memory is linear, we will store it
//as a vec: [1,' ', 3, -, *, -, 2, ' ', 4]
//because we use unicode, every grapheme (displayed element)
//will be stored as a String

//canvas we will be drawing on
#[derive(Clone, Debug)]
pub struct TextCanvas {
    //each item in vec represents a cell, holding single grapheme
    //(which is stored as a String to keep it simple)
    //TODO: this could be replaced with reference to cached set of strings
    data: Vec<String>,
    pub width: usize,
    pub height: usize,
}

impl TextCanvas {
    pub fn new(width: usize, height: usize) -> Self {
        let mut data = Vec::with_capacity(width * height);
        for _ in 0..(width * height) {
            data.push(" ".to_string())
        }
        TextCanvas {
            data,
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &str {
        &self.data[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: &str) {
        self.data[y * self.width + x] = value.to_string()
    }

    //draw another canvas content at given coordinates
    pub fn draw(&mut self, other_text_canvas: &TextCanvas, at_x: usize, at_y: usize) {
        for x in 0..other_text_canvas.width {
            for y in 0..other_text_canvas.height {
                self.set(at_x + x, at_y + y, other_text_canvas.get(x, y))
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut result: String = String::with_capacity(
            self.data.iter().map(|s| s.len()).sum::<usize>()
                + (if self.height > 0 { self.height - 1 } else { 0 }),
        );

        for y in 0..self.height {
            for x in 0..self.width {
                result.push_str(self.get(x, y));
            }
            if y < self.height - 1 {
                result.push('\n');
            }
        }
        result
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_text_canvas() {
        let mut tc = TextCanvas::new(1, 1);
        assert_eq!(tc.to_string(), " ");
        tc.set(0, 0, "a");
        assert_eq!(tc.to_string(), "a");
    }

    #[test]
    fn test_text_canvas_to_string_with_newlines() {
        let tc = TextCanvas::new(1, 2);
        assert_eq!(tc.to_string(), " \n ");
    }

    #[test]
    fn test_text_canvas_draw() {
        let mut tc = TextCanvas::new(2, 2);
        tc.set(0, 0, "a");
        tc.set(1, 0, "b");
        tc.set(0, 1, "c");
        tc.set(1, 1, "d");
        assert_eq!(tc.to_string(), "ab\ncd");
        let mut bigger_tc = TextCanvas::new(4, 4);
        bigger_tc.draw(&tc, 0, 0);
        const expected: &str = concat!("ab  \n", "cd  \n", "    \n", "    ");
        assert_eq!(bigger_tc.to_string(), expected);
    }
}
