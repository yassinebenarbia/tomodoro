use std::{fmt::Debug, cmp::Ordering, rc::Rc, ops::Deref};
use tui::widgets;

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
#[derive(Debug, Clone)]
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
pub struct Screen<'w, T: Displayable + Debug + Clone> {
    widgets: Vec<WidgetWrapper<'w, T>>, 
    selected: u8,
}

impl<'w, T> Screen<'w ,T>  where
    T: Displayable + Clone + Debug + 'w,
{

    // NOTE: can we dismiss the call of clone?
    pub fn new(widgets:&'w Vec<Box<T>>)->Screen<'w, T> {

        // here is the logic respobsible for seting up the widgets
        let mut x_widgets: Vec<&'w T> = vec![];
        let mut y_widgets: Vec<&'w T> = vec![];

        for i in 0..widgets.len() {

            x_widgets.push(& widgets[i]);
            y_widgets.push(& widgets[i]);

        }

        // let mut x_widgets = widgets.clone();
        // let mut y_widgets = widgets.clone();

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


        // wrappers vector
        let mut wrv: Vec<WidgetWrapper<'w ,T>> = vec![];
        // needs to determine the left, right, up and down widgets of each 
        // widget and then put them inside the widget wrapper vector wrv
        if widgets.len() == 1{
            wrv = self::Screen::orderw_one(x_widgets, y_widgets);
        }else {
            wrv = self::Screen::orderw(x_widgets, y_widgets);
        }
        
        // TODO: change the clone behavior
        Screen {
            widgets: wrv.clone(),
            selected: 0,
        }

    }

    /// this is used to make a matrix of WidgetWrapper where as
    /// each widget will be sorted with their x and y indecies
    /// NOTE: this will take tow vectors of length 1
    fn orderw_one<'a>(x_widgets: Vec<&T>, y_widgets: Vec<&T>) -> Vec<WidgetWrapper<'a, T>>{
        let mut toreturn: Vec<WidgetWrapper<T>> = vec![];

        toreturn.push(
            WidgetWrapper{
                wrapped: x_widgets[0].clone(),
                right: None,
                left: None,
                down: None,
                up: None,
            }
        );

        toreturn
    }

    /// this is used to make a matrix of WidgetWrapper where as
    /// each widget will be sorted with their x and y indecies
    /// NOTE: this will take two vectors of a length greater than 1
    /// those vectors will contain a reference to the initial widgets list
    fn orderw<'a: 'w>(x_widgets: Vec<&'w T>, y_widgets: Vec<&'w T>) -> Vec<WidgetWrapper<'w, T>>{

        let mut toreturn: Vec<WidgetWrapper<'_, T>> = vec![];
        let mut temp = vec![];
        let mut ypos = 0;

        for i in 0..x_widgets.len(){

            // the position of x_widgets[i] in y_widgets, with no repeat
            // meaning if there are two identical widgets, we will get first than the later
            for j in  0..y_widgets.len(){

                // comparing widgets with respect to x value
                let cx = x_widgets[i].x().cmp(&x_widgets[j].x());
                // comparing widgets with respect to y value
                let cy = x_widgets[i].y().cmp(&x_widgets[j].y());

                let cond = match temp.iter().position(|&v|{return v==j}) {
                    Some(x) => true,
                    None => false,
                };

                // cond == false, meaning that the current y_widget is not found
                // in the temp vector
                if cx.is_eq() && cy.is_eq() && cond == false{
                    ypos = j;
                    break;
                }
                // if j == y_widget.len() {
                //      flag = true;
                // }

            }

            // if flag == true {
            //      panic!("widget in the x_widgets is not found in the y_widget");
            // }

            // temp will hold the position of the detected y_widget
            temp.push(ypos);

            // NOTE: x_widgets and y_widget are sorted with the x and y vlaue respectively
            if i == 0 {

                // we are guaranteed that the next widget in the y_widgets exist
                // since the length of it is greater than 1
                if ypos == 0 {

                    toreturn.push(
                        WidgetWrapper{
                            wrapped: x_widgets[i].clone(),
                            right: Some(x_widgets[i+1]),
                            left: None,
                            down: Some(y_widgets[ypos+1]),
                            up: None,
                        }
                    );
                    
                // we are guaranteed that the previous widget in the y_widgets exist
                // since the length of it is greater than 1
                }else if ypos == y_widgets.len()-1 {

                    toreturn.push(
                        WidgetWrapper{
                            wrapped: x_widgets[i].clone(),
                            right: Some(x_widgets[i+1]),
                            left: None,
                            down: None,
                            up: Some(y_widgets[ypos-1]),
                        }
                    );

                }else {

                    toreturn.push(
                        WidgetWrapper{
                            wrapped: x_widgets[i].clone(),
                            right: Some(x_widgets[i+1]),
                            left: None,
                            down: Some(y_widgets[ypos+1]),
                            up: Some(y_widgets[ypos-1]),
                        }
                    );
                    
                }
                
            }else if i == x_widgets.len() - 1 {
                                
                if ypos == 0 {

                    toreturn.push(
                        WidgetWrapper{
                            wrapped: x_widgets[i].clone(),
                            right: None,
                            left: Some(x_widgets[i-1]),
                            down: Some(y_widgets[ypos+1]),
                            up: None,
                        }
                    );
                    
                }else if ypos == y_widgets.len() {

                    toreturn.push(
                        WidgetWrapper{
                            wrapped: x_widgets[i].clone(),
                            right: None,
                            left: Some(x_widgets[i-1]),
                            down: None,
                            up: Some(y_widgets[ypos-1]),
                        }
                    );

                }else {

                    toreturn.push(
                        WidgetWrapper{
                            wrapped: x_widgets[i].clone(),
                            right: None,
                            left: Some(x_widgets[i-1]),
                            down: Some(y_widgets[ypos+1]),
                            up: Some(y_widgets[ypos-1]),
                        }
                    );
                    
                }

            } else {

                if ypos == 0 {

                    toreturn.push(
                        WidgetWrapper{
                            wrapped: x_widgets[i].clone(),
                            right: Some(x_widgets[i+1]),
                            left: Some(x_widgets[i-1]),
                            down: Some(y_widgets[ypos+1]),
                            up: None,
                        }
                    );
                    
                }else if ypos == y_widgets.len() {

                    toreturn.push(
                        WidgetWrapper{
                            wrapped: x_widgets[i].clone(),
                            right: Some(x_widgets[i+1]),
                            left: Some(x_widgets[i-1]),
                            down: None,
                            up: Some(y_widgets[ypos-1]),
                        }
                    );

                }else {

                    toreturn.push(
                        WidgetWrapper{
                            wrapped: x_widgets[i].clone(),
                            right: Some(x_widgets[i+1]),
                            left: Some(x_widgets[i-1]),
                            down: Some(y_widgets[ypos+1]),
                            up: Some(y_widgets[ypos-1]),
                        }
                    );
                    
                }

            }

        }

        toreturn

    }

    pub fn load_conf(){}

    pub fn selected(&self) -> u8{
        self.selected
    }

}

mod Test{

    use crate::app::Dumy;

    use super::Screen;


    #[test]
    fn should_sort() {

        // desired behavior
        // let app = App::create(config_path);
        // loop {
        //  app.draw();
        // }
        let dumy = Dumy::new(1, 1);
        let dumy1 = Dumy::new(4, 9);
        let dumy2 = Dumy::new(9, 9);
        // let screen = Screen::new(& vec![&dumy, &dumy1, &dumy2]);
        
    }

}
