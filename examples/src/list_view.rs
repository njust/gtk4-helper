use gtk4_helper::{
    gtk,
    glib,
    gio,
    model::prelude::*,
};

use gtk4_helper::gtk::{Orientation};
use crate::models::{Person, get_persons};

pub fn list() -> gtk::Box {
    let list_store = gio::ListStore::new(Person::static_type());
    let persons = get_persons(10);
    for person in persons {
        let obj: glib::Object = person.to_object();
        list_store.append(&obj);
    }

    let selection_model = gtk::SingleSelection::new(Some(&list_store));
    let item_factory = gtk::SignalListItemFactory::new();

    item_factory.connect_bind(move |_, b| {
        if let Some(item) = b.item()
        {
            let e = gtk::Entry::new();
            item.bind_property(Person::name, &e, "text")
                .flags(glib::BindingFlags::DEFAULT |glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL).build();
            b.set_child(Some(&e));
        }
    });

    let list_view = gtk::ListViewBuilder::new()
        .factory(&item_factory)
        .model(&selection_model)
        .build();


    let container = gtk::Box::new(Orientation::Vertical, 0);
    let btn = gtk::Button::with_label("Check");
    btn.connect_clicked(move |_| {
        for i in 0..list_store.n_items() {
            if let Some(o) = list_store.item(i) {
                let item = Person::from_object(&o);
                println!("{:?}", item);
            }
        }
    });
    let sw = gtk::ScrolledWindow::new();
    sw.set_vexpand(true);
    sw.set_child(Some(&list_view));
    container.append(&btn);
    container.append(&sw);
    container
}