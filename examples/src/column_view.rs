use gtk4_helper::{
    gtk4,
    glib,
    gio,
    model::prelude::*,
};

use gtk4_helper::gtk4::{Orientation, NONE_EXPRESSION, NONE_SORTER, ColumnView};

use crate::models::{Person, get_persons};

fn create_item(_factory: &gtk4::SignalListItemFactory, item: &gtk4::ListItem, property: &str) {
    if let Some(obj) = item.get_item() {
        let lbl = gtk4::Label::new(None);
        obj.bind_property(property, &lbl, "label")
            .flags(glib::BindingFlags::DEFAULT |glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL).build();
        item.set_child(Some(&lbl));
    }
}

fn create_column(column_view: &ColumnView, ty: glib::Type, property: &'static str, title: &str) {
    let column_factory = gtk4::SignalListItemFactory::new();
    column_factory.connect_bind(move |a, item| {
        create_item(a, item, property)
    });

    let prop_exp = gtk4::PropertyExpression::new(ty, NONE_EXPRESSION, property);
    column_view.append_column(&gtk4::ColumnViewColumnBuilder::new()
        .title(title)
        .factory(&column_factory)
        .sorter(&gtk4::StringSorter::new(Some(&prop_exp)))
        .build()
    );
}


pub fn list() -> gtk4::Box {
    let list_store = gio::ListStore::new(Person::static_type());
    let persons = get_persons(10);
    for person in persons {
        let obj: glib::Object = person.to_object();
        list_store.append(&obj);
    }

    let sort_view = gtk4::SortListModel::new(Some(&list_store), NONE_SORTER);
    let column_view = gtk4::ColumnViewBuilder::new()
        .model(&gtk4::SingleSelection::new(Some(&sort_view)))
        .build();

    if let Some(so) = column_view.get_sorter() {
        sort_view.set_sorter(Some(&so));
    }

    create_column(&column_view, Person::static_type(),Person::name,"Name");
    create_column(&column_view, Person::static_type(), Person::surname, "Surname");


    let container = gtk4::Box::new(Orientation::Vertical, 0);
    let sw = gtk4::ScrolledWindow::new();
    sw.set_vexpand(true);
    sw.set_child(Some(&column_view));
    container.append(&sw);
    container
}