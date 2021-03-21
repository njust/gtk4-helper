use gtk4_helper::{
    prelude::*,
    gtk4,
};

use std::env::args;
mod counter;
mod list_view;
mod column_view;
mod manual_column_view;
mod models;
mod expressions;
mod manual_model;

use crate::counter::CounterWidget;

fn build_ui(application: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(application);
    window.set_title(Some("GTK Test Program"));
    window.set_default_size(1024, 768);


    // expressions::test();
    let notebook = gtk4::Notebook::new();

    let column_view = manual_column_view::Test::new();
    notebook.append_page(&column_view.list(), Some(&gtk4::Label::new(Some("Manual column view"))));

    let counter = CounterWidget::new();
    notebook.append_page(counter.view(), Some(&gtk4::Label::new(Some("Counter"))));

    let list_view = list_view::list();
    notebook.append_page(&list_view, Some(&gtk4::Label::new(Some("List view"))));

    let column_view = column_view::list();
    notebook.append_page(&column_view, Some(&gtk4::Label::new(Some("Column view"))));

    window.set_child(Some(&notebook));
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