pub mod prelude {
    pub use gtk::prelude::*;
    pub use gtk::gio::glib::subclass::prelude::{ObjectSubclass, ObjectImpl};
    pub use crate::glib::{Value, Type, value::FromValue, value::GenericValueTypeOrNoneChecker, Object, self};
    pub use crate::glib::ParamSpec;
    pub use once_cell;
    pub use gtk4_helper_macros::{DataModel, model};
    pub use std::{
        cell::RefCell,
        collections::HashMap
    };
}