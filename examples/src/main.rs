use gtk4_helper::{
    prelude::*,
    gtk4,
    glib
};

use std::env::args;
use crate::counter::{CounterMsg, SimpleCounter};

mod counter;
mod list_view;
mod column_view;
mod models;
mod expressions;

pub enum AppMsg {
    CounterMsg(CounterMsg),
}

fn build_ui(application: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(application);
    window.set_title(Some("GTK Test Program"));
    window.set_default_size(1024, 768);

    let notebook = gtk4::Notebook::new();

    let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let tx_ = tx.clone();
    let mut counter = SimpleCounter::new(move | msg|
        tx_.send(AppMsg::CounterMsg(msg)).expect("Could not send msg")
    , Some(2));
    notebook.append_page(counter.view(), Some(&gtk4::Label::new(Some("Counter"))));

    rx.attach(None, move |msg| {
        match msg {
            AppMsg::CounterMsg(msg) => {
                counter.update(msg);
            }
        }
        glib::Continue(true)
    });

    // expressions::test();
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