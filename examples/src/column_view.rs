use gtk4_helper::{
    gtk,
    glib,
    gio,
    model::prelude::*,
};

use gtk4_helper::gtk::{Orientation, NONE_EXPRESSION, NONE_SORTER, ColumnView};

use crate::models::{Person, get_persons};

fn create_item(_factory: &gtk::SignalListItemFactory, item: &gtk::ListItem, property: &str) {
    if let Some(obj) = item.item() {
        let lbl = gtk::Label::new(None);
        obj.bind_property(property, &lbl, "label")
            .flags(glib::BindingFlags::DEFAULT |glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL).build();
        item.set_child(Some(&lbl));
    }
}

fn create_column(column_view: &ColumnView, ty: glib::Type, property: &'static str, title: &str, num: bool) {
    let column_factory = gtk::SignalListItemFactory::new();
    column_factory.connect_bind(move |a, item| {
        create_item(a, item, property)
    });

    let prop_exp = gtk::PropertyExpression::new(ty, NONE_EXPRESSION, property);
    let mut col_builder = gtk::ColumnViewColumnBuilder::new()
        .title(title)
        .factory(&column_factory);
    col_builder = if !num {
        col_builder.sorter(&gtk::StringSorter::new(Some(&prop_exp)))
    }else {
        col_builder.sorter(&gtk::NumericSorter::new(Some(&prop_exp)))
    };
    column_view.append_column(&col_builder.build());
}


pub fn list() -> gtk::Box {
    let list_store = gio::ListStore::new(Person::static_type());
    let persons = get_persons(10);
    for person in persons {
        let obj: glib::Object = person.to_object();
        list_store.append(&obj);
    }

    let sort_view = gtk::SortListModel::new(Some(&list_store), NONE_SORTER);
    let column_view = gtk::ColumnViewBuilder::new()
        .model(&gtk::SingleSelection::new(Some(&sort_view)))
        .build();

    if let Some(so) = column_view.sorter() {
        sort_view.set_sorter(Some(&so));
    }

    create_column(&column_view, Person::static_type(),Person::name,"Name", false);
    create_column(&column_view, Person::static_type(), Person::surname, "Surname", false);
    create_column(&column_view, Person::static_type(), Person::age, "Age", true);


    let container = gtk::Box::new(Orientation::Vertical, 0);
    let sw = gtk::ScrolledWindow::new();
    sw.set_vexpand(true);
    sw.set_child(Some(&column_view));
    container.append(&sw);
    container
}