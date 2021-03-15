use gtk4_helper::{
    gtk4,
    glib,
    gio,
    list::prelude::*,
};
use gtk4_helper::gio::glib::Object;

#[model]
pub struct Person {
    #[param]
    pub name: String,
    #[param]
    pub sure_name: Option<String>,
    #[param(min = "0", max = "100")]
    pub age: i32,
    #[param(min = "0.0", max = "100000")]
    pub savings: f64,
    #[param]
    pub happy: bool,
}

pub fn list() -> gtk4::ScrolledWindow {
    let list_store = gio::ListStore::new(Person::static_type());

    for i in 30..100 {
        let h = Person {
            name: format!("Name {}", i),
            sure_name: Some(format!("Surname {}", i)),
            age: i,
            savings: i as f64 + 10.1,
            happy: i % 2 == 0
        };
        list_store.append(&h.to_object());
    }

    let selection_model = gtk4::SingleSelection::new(Some(&list_store));
    let item_factory = gtk4::SignalListItemFactory::new();
    item_factory.connect_bind(|_, b| {
        if let Some(item) = b.get_item() {
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

    let sw = gtk4::ScrolledWindow::new();
    sw.set_child(Some(&list_view));
    sw
}