use std::{fs::File, io::Write, rc::Rc};

use tui::{Frame, backend::Backend};
// desired behavior
// let fixer = Fixer::new(f);
// fixer.rrect(10, 20, 5, 10); // returns a Rect with width 5% and height 10%
// of the screen, and on a position 10% and 20% respectivelf
// some_widget.layout(fixer.xratio(20), fixer.yratio(10), 10, 20);
// other_widget.layout(fixer.rrect(10, 20, 5, 10));

// #[derive(Debug)]
// can put the backed here
pub struct Fixer<'a, B: Backend> {
    frame: &'a Frame<'a, B>
}

impl<'a, B: Backend> Fixer<'a, B> {

    pub fn new(f: &'a Frame<'a, B>) -> Fixer<'a, B>{

        Fixer {
            frame: f 
        }

    }
    // pub fn new(f: &'a Frame<'a, B> )->Fixer<'a, B>{

    //     Fixer {
    //         frame: f 
    //     }

    // }

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

        if x == 0 {
            
            return size.x as u16

        }

        let fwidth:f32 = size.width as f32 / 100 as f32 * x as f32;

        let uwidth:u16 = fwidth as u16;

        // 20%
        // size.x / 100 * 20%
        uwidth + size.x

    }

    pub fn yratio(&mut self, y: u16) -> u16{

        if y > 100 {
            panic!("unable to provide y ratio bigger than 100\n
                provided {} but expected a number less than 100", y);
        }

        let size = self.frame.size();

        if y == 0 {

            return size.y as u16

        }

        // access screen height
        // get the length of 1%
        // multiply that by the ratio y

        let fheight:f32 = size.height as f32 / 100 as f32 * y as f32;

        let uheight:u16 = fheight as u16;
        
        // 20%
        // size.y / 100 * 20%
        uheight + size.y

    }

    pub fn wratio(&mut self, width: u16) -> u16{

        if width == 0{return 0}

        let size = self.frame.size();

        let width_rect = size.width;
        let x_rect = size.x;
        let width_bound = width_rect + x_rect;

        if width > width_bound {
            panic!("width out of bound\n
                provided {} but expected a number less than {}", width, width_rect);
        }
        let fwidth:f32 = width_rect as f32 / 100 as f32 * width as f32;

        let uwidth:u16 = fwidth as u16;

        uwidth

    }

    pub fn hratio(&mut self, height: u16) -> u16{

        if height == 0{return 0}

        let size = self.frame.size();

        let height_rect = size.height;
        let y_rect = size.y;
        let height_bound = height_rect + y_rect;

        if height > height_bound {
            panic!("width out of bound\n
                provided {} but expected a number less than {}", height, height_bound);
        }

        let fheight:f32 = height_bound as f32 / 100 as f32 * height as f32;

        let uheight:u16 = fheight as u16;

       uheight 

    }

}
mod Test{
    #[test]
    fn eq_ratios() {
        unimplemented!();
    }
}
