use gtk4_helper::{
    gtk4::{self, prelude::*},
    glib::{self, clone},
    gio
};

use gtk4_helper::gtk4::{Orientation, NONE_EXPRESSION, NONE_SORTER, PropertyExpression};

use crate::manual_model::{
    ManualPersonObject, ManualAddressObject
};

#[allow(dead_code)]
pub struct Test {
    address_exp: PropertyExpression,
    street_exp: PropertyExpression,
    name_exp: PropertyExpression,
}

impl Test {
    pub fn new() -> Self {
        let address_exp = gtk4::PropertyExpression::new(ManualPersonObject::static_type(), NONE_EXPRESSION, "address");
        let name_exp = gtk4::PropertyExpression::new(ManualPersonObject::static_type(), NONE_EXPRESSION, "name");
        let street_exp = gtk4::PropertyExpression::new(ManualAddressObject::static_type(), Some(&address_exp), "street");

        Self {
            address_exp,
            street_exp,
            name_exp
        }
    }

    fn get_exp(&self) -> &PropertyExpression {
        &self.street_exp
    }

    pub fn list(&self) -> gtk4::Box {
        let list_store = gio::ListStore::new(ManualPersonObject::static_type());
        for i in 0..10 {
            let address = ManualAddressObject::new(format!("test str {}", i));
            let person: glib::Object = ManualPersonObject::new(format!("tester {}", i), &address);
            list_store.append(&person);
        }

        let sort_view = gtk4::SortListModel::new(Some(&list_store), NONE_SORTER);
        let column_view = gtk4::ColumnViewBuilder::new()
            .model(&gtk4::SingleSelection::new(Some(&sort_view)))
            .build();


        if let Some(so) = column_view.get_sorter() {
            sort_view.set_sorter(Some(&so));
        }

        let exp = self.get_exp();
        let column_factory = gtk4::SignalListItemFactory::new();
        column_factory.connect_bind(clone!(@strong exp => move |_, item| {
            if let Some(obj) = item.get_item() {
                let lbl = gtk4::Label::new(None);
                exp.bind(lbl.upcast_ref(), "label", Some(&obj));
                item.set_child(Some(&lbl));
            }
        }));

        column_view.append_column(&gtk4::ColumnViewColumnBuilder::new()
            .title("Name")
            .factory(&column_factory)
            .sorter(&gtk4::StringSorter::new(Some(self.get_exp())))
            .build()
        );

        let container = gtk4::Box::new(Orientation::Vertical, 0);
        let sw = gtk4::ScrolledWindow::new();
        sw.set_vexpand(true);
        sw.set_child(Some(&column_view));
        container.append(&sw);
        container
    }
}