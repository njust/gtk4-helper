use gtk4_helper::{
    gtk4,
    glib,
    gio,
    list::prelude::*,
};

#[model]
pub struct Hudel  {
    #[param()]
    name: String,
    #[param()]
    gerda: Option<String>,
    #[param(min = "0", max = "100")]
    count: i32,
    #[param(min = "0.0", max = "1000")]
    double: f64,
    #[param()]
    even: bool,
}

pub fn list() -> gtk4::ScrolledWindow {
    let m = gio::ListStore::new(Hudel::static_type());
    for i in 0..100 {
        let h = Hudel {
            name: format!("gerda {}", i),
            gerda: if i % 2 == 0 { None } else { Some(format!("hudel {}", i)) },
            count: i,
            double: 0.1,
            even: i % 2 == 0
        };
        m.append(&h.to_object());
    }

    let s = gtk4::SingleSelection::new(Some(&m));
    let f = gtk4::SignalListItemFactory::new();
    f.connect_bind(|_,b|{
        if let Some(item) = b.get_item() {
            let e = gtk4::Entry::new();
            item.bind_property(Hudel::even, &e, "text")
                .flags(glib::BindingFlags::DEFAULT |glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL).build();
            b.set_child(Some(&e));
        }
    });
    let list = gtk4::ListViewBuilder::new()
        .factory(&f)
        .model(&s)
        .build();

    let sw = gtk4::ScrolledWindow::new();
    sw.set_child(Some(&list));
    sw
}