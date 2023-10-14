use std::{fmt::Debug, cmp::Ordering};
use crate::displayable::Displayable;

// question:
// does dealing with the widget inside the screen depend on the type of the widget, 
// meaning being a stateless and statefull widget respectively
// design choice: 
// 1) having a single vector of widgets that will encapsulate all the stateless and statefull
// widgets
// pros: 
//  - having all widgets encapsulated in a one variable
//  - a single variable is reposable for deaing with all widgets
// cons:
//  - having to check each time we deal with a widget for it's type
// 2) having a separate vecctor for stateless and statefull widgets respetively
// Answer:
// the screen will be only concerned about the statefull widgets, since we can
// have a statefull widget that behave like stateless one, and not the other
// way around
#[derive(Debug)]
struct WidgetWrapper<'a, T: Displayable>{
    wrapped: T,
    up: Option<&'a T>,
    down: Option<&'a T>,
    right: Option<&'a T>,
    left: Option<&'a T>,
}

/// This screen will act as a wrapper for all widgets, that triggers
/// the highlight method for the selected widget and also will: 
///  - change the selected widget using a defined api of methods as dow(),
/// up(), right() and left()
/// - retrive the selected widget using the selected() method
pub struct Screen<T: Displayable + Debug + Clone> {
    widgets: Vec<T>
}

impl<T> Screen<T>  where
    T: Displayable + Clone + Debug 
{

    // NOTE: can we dismiss the call of clone?
    pub fn new(widgets: Vec<T>){
        // here is the logic respobsible for seting up the widgets
        let mut x_widgets: Vec<T> = vec![];
        let mut y_widgets: Vec<T> = vec![];

        // let mut xw:Vec<_> = vec![];
        // let mut yw:Vec<_> = vec![];

        for i in 0..widgets.len() {
            x_widgets.push(widgets[i].clone());
            y_widgets.push(widgets[i].clone());
        }

        x_widgets.sort_by(|w1, w2|{
            match w1.x() > w2.x() {
                true => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Less,
            }
        });

        y_widgets.sort_by(|w1, w2|{
            match w1.y() > w2.y() {
                true => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Less,
            }
        });


        let mut wrv: Vec<WidgetWrapper<T>> = vec![];
        // needs to determine the left, right, up and down widgets of each 
        // widget and then put them inside the widget wrapper vector wrv
        if widgets.len() == 1{
            wrv = self::Screen::orderw_one(&x_widgets, &y_widgets);
        }else {
            wrv = self::Screen::orderw(&x_widgets, &y_widgets)
        }

    }

    /// this is used to make a matrix of WidgetWrapper where as
    /// each widget will be sorted with their x and y indecies
    /// NOTE: this will take tow vectors of length 1
    fn orderw_one<'a>(x_widgets:&'a Vec<T>,y_widgets:&'a Vec<T>) -> Vec<WidgetWrapper<'a, T>>{
        let toreturn: Vec<WidgetWrapper<T>> = vec![];
        println!("unimplemented!()");
        toreturn
    }

    /// this is used to make a matrix of WidgetWrapper where as
    /// each widget will be sorted with their x and y indecies
    /// NOTE: this will take tow vectors of length greater than 1
    fn orderw<'a>(x_widgets:&'a Vec<T>, y_widgets:&'a Vec<T>) -> Vec<WidgetWrapper<'a, T>>{

        let mut toreturn: Vec<WidgetWrapper<'a, T>> = vec![];
        let mut temp = vec![];
        let mut ypos = 0;
        for i in 0..x_widgets.len()-1{

            // the position of x_widgets[i] in y_widgets
            for j in  0..y_widgets.len(){

                let cx = x_widgets[i].x().cmp(&x_widgets[j].x());
                let cy = x_widgets[i].y().cmp(&x_widgets[j].y());
                let cond = match temp.iter().position(|&v|{return v==j}) {
                    Some(x) => true,
                    None => false,
                };

                if cx.is_eq() && cy.is_eq() && cond  & cond{
                    ypos = j;
                    break;
                }
            }
            temp.push(ypos);
            println!("{}", ypos);

            if i == 0 {

                // we are guaranteed that the next widget in the y_widgets exist
                // since the length of it is greater than 1
                if ypos == 0 {

                    toreturn.push(
                        WidgetWrapper{
                            wrapped: x_widgets[i].clone(),
                            right: x_widgets.get(i+1),
                            left: None,
                            down: y_widgets.get(ypos+1),
                            up: None,
                        }
                    );
                    
                // we are guaranteed that the previous widget in the y_widgets exist
                // since the length of it is greater than 1
                }else if ypos == y_widgets.len() {

                    toreturn.push(
                        WidgetWrapper{
                            wrapped: x_widgets[i].clone(),
                            right: x_widgets.get(i+1),
                            left: None,
                            down: None,
                            up: y_widgets.get(ypos-1),
                        }
                    );

                }else {

                    toreturn.push(
                        WidgetWrapper{
                            wrapped: x_widgets[i].clone(),
                            right: x_widgets.get(i+1),
                            left: None,
                            down: y_widgets.get(ypos+1),
                            up: y_widgets.get(ypos-1),
                        }
                    );
                    
                }
                
            } if i == x_widgets.len() -1 {
                
                if ypos == 0 {

                    toreturn.push(
                        WidgetWrapper{
                            wrapped: x_widgets[i].clone(),
                            right: None,
                            left: x_widgets.get(i-1),
                            down: y_widgets.get(ypos+1),
                            up: None,
                        }
                    );
                    
                }else if ypos == y_widgets.len() {

                    toreturn.push(
                        WidgetWrapper{
                            wrapped: x_widgets[i].clone(),
                            right: None,
                            left: x_widgets.get(i-1),
                            down: None,
                            up: y_widgets.get(ypos-1),
                        }
                    );

                }else {

                    toreturn.push(
                        WidgetWrapper{
                            wrapped: x_widgets[i].clone(),
                            right: None,
                            left: x_widgets.get(i-1),
                            down: y_widgets.get(ypos+1),
                            up: y_widgets.get(ypos-1),
                        }
                    );
                    
                }

            } else {

                if ypos == 0 {

                    toreturn.push(
                        WidgetWrapper{
                            wrapped: x_widgets[i].clone(),
                            right: x_widgets.get(i+1),
                            left: x_widgets.get(i-1),
                            down: y_widgets.get(ypos+1),
                            up: None,
                        }
                    );
                    
                }else if ypos == y_widgets.len() {

                    toreturn.push(
                        WidgetWrapper{
                            wrapped: x_widgets[i].clone(),
                            right: x_widgets.get(i+1),
                            left: x_widgets.get(i-1),
                            down: None,
                            up: y_widgets.get(ypos-1),
                        }
                    );

                }else {

                    toreturn.push(
                        WidgetWrapper{
                            wrapped: x_widgets[i].clone(),
                            right: x_widgets.get(i+1),
                            left: x_widgets.get(i-1),
                            down: y_widgets.get(ypos+1),
                            up: y_widgets.get(ypos-1),
                        }
                    );
                    
                }

            }

        }

        toreturn
    }

    pub fn load_conf(){}

    pub fn selected() -> T{
       unimplemented!() 
    }

    pub fn up(){}
    pub fn down(){}
    pub fn right(){}
    pub fn left(){}
    
}
