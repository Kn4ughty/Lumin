// use gtk::traits::ButtonExt;
// use relm4::{gtk, SimpleComponent};
// use gtk::glib::clone;
// use relm4::{ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};


use gtk::glib::clone;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt};
use relm4::{ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

use gtk::{Window};

struct AppModel {
    counter: u8,
}

#[derive(Debug)]
enum AppMsg {
    Increment,
    Decrement,
}

struct AppWidgets {
    label: gtk::Label,
}

impl SimpleComponent for AppModel {
    type Input = AppMsg;

    type Output = ();

    type Init = u8;

    type Root = gtk::Window;
    
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        gtk::Window::builder()
            .title("Simple app")
            .default_width(300)
            .default_height(100)
            .build()
    }

    fn init(
            init: Self::Init,
            root: Self::Root,
            sender: relm4::ComponentSender<Self>,
        ) -> relm4::ComponentParts<Self> {

        let model = AppModel {counter: 0};

        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .build();

        let inc_button = gtk::Button::with_label("Increment");
        let dec_button = gtk::Button::with_label("Decrement");

        let label = gtk::Label::new(Some(&format!("Num: {}", model.counter)));
        label.set_margin_all(5);

        root.set_child(Some(&vbox));
        vbox.set_child(&inc_button);
        vbox.set_child(&dec_button);
        vbox.set_child(&label);

        inc_button.connect_clicked(clone!(

    ))

    }
}

fn main() {
    println!("Hello, world!");
}
