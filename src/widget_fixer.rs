#[derive(Debug)]
// can put the backed here
struct Fixer {}

impl Fixer {

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

    pub fn xratio(x: u8){
        // access screen width
        // get the length of 1%
        // multiply that by the ratio x
        if x > 100 {
            panic!("unable to provide x ratio bigger than 100\n
                provided {} but expected a number less than 100", x);
        }
    }

    pub fn yratio(y: u8){
        // access screen height
        // get the length of 1%
        // multiply that by the ratio y
        if y > 100 {
            panic!("unable to provide y ratio bigger than 100\n
                provided {} but expected a number less than 100", y);
        }
    }

}
