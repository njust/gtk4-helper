use gtk4_helper::{
    gtk::{self, Orientation},
    glib,
    gio::ListStore,
    model::prelude::*,
    tree::{TreeViewDataProvider, TreeView}
};

use crate::models::{Person, get_persons};

struct DataProvider {}
impl TreeViewDataProvider for DataProvider {
    fn load(&self, parent: Option<&Object>) -> Option<ListStore> {
        let list_store = ListStore::new(Person::static_type());
        let persons = if parent.is_none() {
            get_persons(10)
        }else {
            get_persons(5)
        };

        for person in persons {
            let obj: glib::Object = person.to_object();
            list_store.append(&obj);
        }

        Some(list_store)
    }
}

struct Tree {}
impl TreeView for Tree {
    type TreeItem = gtk::Label;
    type DataProvider = DataProvider;

    fn create_tree_item(obj: Object) -> Self::TreeItem {
        let lbl = gtk::Label::new(None);
        obj.bind_property(Person::name, &lbl, "label")
            .flags(glib::BindingFlags::DEFAULT |glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL).build();
        lbl
    }
}

pub fn tree() -> gtk::Box {
    let dp = DataProvider {};
    let tree_view = Tree::view(dp);
    let container = gtk::Box::new(Orientation::Vertical, 0);
    let sw = gtk::ScrolledWindow::new();

    sw.set_vexpand(true);
    sw.set_child(Some(&tree_view));
    container.append(&sw);
    container
}