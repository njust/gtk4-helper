use gtk4_helper::{
    gtk,
    gtk::{
        TreeListRow,
        TreeExpanderBuilder,
    },
    glib,
    gio,
    model::prelude::*,
};

use gtk4_helper::gtk::{Orientation};

use crate::models::{Person, get_persons};

pub fn tree() -> gtk::Box {
    let item_factory = gtk::SignalListItemFactory::new();
    item_factory.connect_bind(move |_, list_item| {
        if let Some((row, child_obj)) = list_item.item()
            .and_then(|obj| obj.downcast::<TreeListRow>().ok())
            .and_then(|row| row.item().map(|child_obj| (row, child_obj)))
        {
            let lbl = gtk::Label::new(None);
            child_obj.bind_property(Person::name, &lbl, "label")
                .flags(glib::BindingFlags::DEFAULT |glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL).build();

            let expander = TreeExpanderBuilder::new()
                .child(&lbl)
                .list_row(&row)
                .build();

            list_item.set_child(Some(&expander));
        }
    });

    let list_store = gio::ListStore::new(Person::static_type());
    let persons = get_persons(10);
    for person in persons {
        let obj: glib::Object = person.to_object();
        list_store.append(&obj);
    }

    let model = list_store.upcast_ref::<gio::ListModel>();
    let tree_list_model = gtk::TreeListModel::new(model, false, false, |_obj| {
        let list_store = gio::ListStore::new(Person::static_type());
        let persons = get_persons(5);
        for person in persons {
            let obj: glib::Object = person.to_object();
            list_store.append(&obj);
        }
        Some(list_store.upcast::<gio::ListModel>())
    });

    let selection_model = gtk::SingleSelection::new(Some(&tree_list_model));
    let list_view = gtk::ListViewBuilder::new()
        .factory(&item_factory)
        .model(&selection_model)
        .build();

    let container = gtk::Box::new(Orientation::Vertical, 0);
    let sw = gtk::ScrolledWindow::new();

    sw.set_vexpand(true);
    sw.set_child(Some(&list_view));
    container.append(&sw);
    container
}