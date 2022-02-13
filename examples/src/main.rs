use gtk4_helper::{
    prelude::*,
    gtk,
    glib
};

use crate::counter::{CounterMsg, SimpleCounter};

mod counter;
mod list_view;
mod column_view;
mod models;
mod expressions;
// mod tree_view;

pub enum AppMsg {
    CounterMsg(CounterMsg),
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("GTK Test Program"));
    window.set_default_size(1024, 768);

    let notebook = gtk::Notebook::new();
    let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    // let tree_view = tree_view::tree();
    // notebook.append_page(&tree_view, Some(&gtk::Label::new(Some("Tree view"))));

    let mut counter = SimpleCounter::new_with_data(move |m| {
        tx.send(AppMsg::CounterMsg(m)).expect("Could not send msg");
    }, 2);
    notebook.append_page(counter.view(), Some(&gtk::Label::new(Some("Counter"))));

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
    notebook.append_page(&list_view, Some(&gtk::Label::new(Some("List view"))));

    let column_view = column_view::list();
    notebook.append_page(&column_view, Some(&gtk::Label::new(Some("Column view"))));

    window.set_child(Some(&notebook));
    window.show();
}

#[tokio::main]
async fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default());

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run();
}