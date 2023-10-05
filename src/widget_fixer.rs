use tui::{Frame, backend::Backend};
// desired behavior
// let fixer = Fixer::new(f);
// fixer.rrect(10, 20, 5, 10); // returns a Rect with width 5% and height 10%
// of the screen, and on a position 10% and 20% respectivelf
// some_widget.layout(fixer.xratio(20), fixer.yratio(10), 10, 20);
// other_widget.layout(fixer.rrect(10, 20, 5, 10));

// #[derive(Debug)]
// can put the backed here
struct Fixer<'a, B: Backend> {
    frame: &'a Frame<'a, B>
}

impl<'a, B: Backend> Fixer<'a, B> {

    pub fn ratio(x: u8, y: u8){
        if x > 100 {
            panic!("unable to provide x ratio bigger than 100\n
                provided {} but expected a number less than 100", x);
        }
        if y > 100 {
            panic!("unable to provide y ratio bigger than 100\n
                provided {} but expected a number less than 100", y);
        }
        // use xratio and yratio respectively to extract the correct ratios
    }

    pub fn xratio(&mut self, x: u16) -> u16{

        // access screen width
        // get the length of 1%
        // multiply that by the ratio x
        if x > 100 {
            panic!("unable to provide x ratio bigger than 100\n
                provided {} but expected a number less than 100", x);
        }
        let size = self.frame.size();
        // 20%
        // size.x / 100 * 20%
        size.x / 100 * x
    }

    pub fn yratio(&mut self, y: u16) -> u16{

        // access screen height
        // get the length of 1%
        // multiply that by the ratio y
        if y > 100 {
            panic!("unable to provide y ratio bigger than 100\n
                provided {} but expected a number less than 100", y);
        }

        let size = self.frame.size();
        // 20%
        // size.y / 100 * 20%
        size.y / 100 * y

    }

}
