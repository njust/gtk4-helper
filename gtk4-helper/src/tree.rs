use crate::{
    gtk,
    gtk::{
        TreeListRow,
        TreeExpanderBuilder,
        Widget
    },
    gio::{self, ListStore},
    model::prelude::*,
};

pub trait TreeViewDataProvider {
    fn load(&self, parent: Option<&Object>) -> Option<ListStore>;
}

pub trait TreeView {
    type TreeItem : IsA<Widget>;
    type DataProvider : TreeViewDataProvider;
    fn create_tree_item(obj: Object) -> Self::TreeItem;

    fn view<T: TreeViewDataProvider + 'static>(data_provider: T) -> gtk::ListView {
        let item_factory = gtk::SignalListItemFactory::new();
        item_factory.connect_bind(move |_, list_item| {
            if let Some((row, child_obj)) = list_item.item()
                .and_then(|obj| obj.downcast::<TreeListRow>().ok())
                .and_then(|row| row.item().map(|child_obj| (row, child_obj)))
            {
                let tree_item = Self::create_tree_item(child_obj);
                let expander = TreeExpanderBuilder::new()
                    .child(&tree_item)
                    .list_row(&row)
                    .build();

                list_item.set_child(Some(&expander));
            }
        });

        let list_store = data_provider.load(None).expect("No root items");
        let model = list_store.upcast_ref::<gio::ListModel>();

        let tree_list_model = gtk::TreeListModel::new(model, false, false, move |obj| {
            let list_store = data_provider.load(Some(obj));
            list_store.map(|ls| ls.upcast::<gio::ListModel>())
        });

        let selection_model = gtk::SingleSelection::new(Some(&tree_list_model));
        gtk::ListViewBuilder::new()
            .factory(&item_factory)
            .model(&selection_model)
            .build()
    }
}