use tui::{
    backend::{Backend},
    widgets::{Block, Borders, BorderType},
    layout::Rect,
    Frame, style::{Style, Color, Modifier}, buffer::Buffer,
};
use crate::{stateful_button::{StatefullButton, ButtonState}, button::Button};

/// widget
fn get_block<'a>(title: String) -> Block<'a>{
    return Block::default()
        .style(
            Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::BOLD | Modifier::ITALIC)
        )
        .title(title.clone())
        .borders(Borders::ALL);
}

pub fn ui<B: Backend>(f: &mut Frame<B>) {

    // let mut button: Button = Button::new(
    //     f.size(),
    //     Rect {
    //         x: 12,
    //         y: 2,
    //         width: 10,
    //         height: 5,
    //     },
    //     get_block(String::from("some random text"))
    // );
    // button.widget(
    //     Color::Red, Color::Cyan,Modifier::BOLD,
    //     String::from(""), vec![Borders::RIGHT]
    // );

    let mut button2 = Button::default()
        .widget(
            Color::LightRed, Color::LightMagenta, Modifier::BOLD,
            String::from("stuff"), Borders::ALL
        ).layout(1, 1, 10, 10);

    // println!("{:?}", button2.get_layout());
    f.render_widget(button2.get_widget(), button2.get_layout());
    // f.render_stateful_widget(button2.get_widget(), button2.get_layout(), state);

    // let buffer:Buffer = Buffer {
    //     area: Rect { x: 5, y: 10, width: 5, height: 10 },
    //     content: vec![] 
    // };

    // let mut state = ListState::default();
    // state.select(Some(1));
    // let items = vec![
    //     ListItem::new("Item 1"),
    //     ListItem::new("Item 2"),
    // ];
    // let list = List::new(items);
    // let area = Rect::new(0, 0, 5, 5);

    // let items = [ListItem::new("Item 1"), ListItem::new("Item 2"), ListItem::new("Item 3")];
    // List::new(items)
    //     .block(Block::default().title("List").borders(Borders::ALL))
    //     .style(Style::default().fg(Color::White))
    //     .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    //     .highlight_symbol(">>");
    // 
    // println!("{:?}", list);

    let mut onhover = |rect: Rect, buf:&mut Buffer, st:&mut ButtonState|{
        //unimplemented!()
    };
    let mut onclick= |rect: Rect, buf:&mut Buffer, st:&mut ButtonState|{
        // unimplemented!()
    };

    let mut button: StatefullButton = StatefullButton::default()
        .layout(1, 1, 5, 5)
        .widget(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::Red),
            Borders::ALL,
            BorderType::Rounded
        )
        .onhover(&mut onhover)
        .onclick(&mut onclick);
        // .text("some".to_string());

    let layout = button.get_layout().clone();
    // desired behavior
    // button.onclick(some closure here that takes the StatefullButton and the ButtonState instances)
    // button.onhover(some closure here that takes the StatefullButton and the ButtonState instances)

    let mut state:ButtonState = ButtonState::new(true, true);

    f.render_stateful_widget(
        button,
        layout,
        &mut state
    );

    // f.render_widget(get_block(String::from("hello")), frame3);

    // f.render_widget(Wall, Wall_area);
    // f.render_widget(Timer, Timer_area);
    // f.render_widget(Button, Button_area);
}
