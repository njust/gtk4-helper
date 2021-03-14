use gtk4_helper::{
    prelude::*,
    gtk4,
};

use std::env::args;
mod counter;
mod list;

use list::list;

fn build_ui(application: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(application);
    window.set_title(Some("GTK Test Program"));
    window.set_default_size(350, 70);

    // let counter = CounterWidget::new();
    let s = list();
    window.set_child(Some(&s));
    window.show();
}

#[tokio::main]
async fn main() {
    let application =
        gtk4::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}