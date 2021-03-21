use gtk4_helper::{
    gtk4,
    glib,
    gio,
    model::prelude::*,
};

use gtk4_helper::gtk4::{Orientation};
use crate::models::{Person, get_persons};

pub fn list() -> gtk4::Box {
    let list_store = gio::ListStore::new(Person::static_type());
    let persons = get_persons(10);
    for person in persons {
        let obj: glib::Object = person.to_object();
        list_store.append(&obj);
    }

    let selection_model = gtk4::SingleSelection::new(Some(&list_store));
    let item_factory = gtk4::SignalListItemFactory::new();
    item_factory.connect_bind(|_, b| {
        if let Some(item) = b.get_item()
        {
            let e = gtk4::Entry::new();
            item.bind_property(Person::name, &e, "text")
                .flags(glib::BindingFlags::DEFAULT |glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL).build();
            b.set_child(Some(&e));
        }
    });

    let list_view = gtk4::ListViewBuilder::new()
        .factory(&item_factory)
        .model(&selection_model)
        .build();


    let container = gtk4::Box::new(Orientation::Vertical, 0);
    let btn = gtk4::Button::with_label("Check");
    btn.connect_clicked(move |_| {
        for i in 0..list_store.get_n_items() {
            if let Some(o) = list_store.get_object(i) {
                let item = Person::from_object(&o);
                println!("{:?}", item);
            }
        }
    });
    let sw = gtk4::ScrolledWindow::new();
    sw.set_vexpand(true);
    sw.set_child(Some(&list_view));
    container.append(&btn);
    container.append(&sw);
    container
}