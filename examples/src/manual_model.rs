use gtk4_helper::{
    once_cell,
    glib::subclass::prelude::{ObjectSubclass, ObjectImpl},
    glib::{
        self,
        prelude::*,
        subclass::prelude::*
    }
};
use std::cell::{RefCell};

mod imp {
    use super::*;

    pub struct ManualPersonObject {
        name: RefCell<Option<String>>,
        address: RefCell<Option<super::wrp::ManualAddressObject>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ManualPersonObject {
        const NAME: &'static str = "ManualPersonObject";
        type Type = super::wrp::ManualPersonObject;
        type ParentType = glib::Object;
        type Interfaces = ();

        fn new() -> Self {
            Self {
                name: RefCell::new(None),
                address: RefCell::new(None),
            }
        }
    }

    impl ObjectImpl for ManualPersonObject {
        fn properties() -> &'static [glib::ParamSpec] {
            use once_cell::sync::Lazy;
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                vec![
                    glib::ParamSpec::string(
                        "name",
                        "Name",
                        "Name of this object",
                        None,
                        glib::ParamFlags::READWRITE,
                    ),
                    glib::ParamSpec::object(
                        "address",
                        "address",
                        "address",
                        super::wrp::ManualAddressObject::static_type(),
                        glib::ParamFlags::READWRITE,
                    )
                ]
            });

            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            match pspec.get_name() {
                "name" => {
                    let name = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.name.replace(name);
                }
                "address" => {
                    let address = value
                        .get::<super::wrp::ManualAddressObject>()
                        .expect("type conformity checked by `Object::set_property`");
                    self.address.replace(address);
                }
                _ => unimplemented!(),
            }
        }

        fn get_property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.get_name() {
                "name" => self.name.borrow().to_value(),
                "address" => self.address.borrow().to_value(),
                _ => unimplemented!(),
            }
        }

        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }
    }

    pub struct ManualAddressObject {
        name: RefCell<Option<String>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ManualAddressObject {
        const NAME: &'static str = "ManualAddressObject";
        type Type = super::wrp::ManualAddressObject;
        type ParentType = glib::Object;
        type Interfaces = ();

        fn new() -> Self {
            Self {
                name: RefCell::new(None),
            }
        }
    }

    impl ObjectImpl for ManualAddressObject {
        fn properties() -> &'static [glib::ParamSpec] {
            use once_cell::sync::Lazy;
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                vec![
                    glib::ParamSpec::string(
                        "street",
                        "street",
                        "street",
                        None,
                        glib::ParamFlags::READWRITE,
                    )
                ]
            });

            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            match pspec.get_name() {
                "street" => {
                    let name = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.name.replace(name);
                },
                _ => unimplemented!(),
            }
        }

        fn get_property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.get_name() {
                "street" => self.name.borrow().to_value(),
                _ => unimplemented!(),
            }
        }

        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }
    }
}

mod wrp {
    use super::*;
    glib::wrapper! {
        pub struct ManualPersonObject(ObjectSubclass<imp::ManualPersonObject>);
    }

    glib::wrapper! {
        pub struct ManualAddressObject(ObjectSubclass<imp::ManualAddressObject>);
    }

    impl ManualPersonObject {
        pub fn new(name: String, address: &wrp::ManualAddressObject) -> Self {
            glib::Object::new(&[("name", &name), ("address", &address)]).unwrap()
        }
    }

    impl ManualAddressObject {
        // Create an object instance of the new type.
        pub fn new(street: String) -> Self {
            glib::Object::new(&[("street", &street)]).unwrap()
        }
    }
}

pub struct ManualPersonObject {}
impl ManualPersonObject {
    pub fn new(name: String, address: &wrp::ManualAddressObject) -> glib::Object {
        wrp::ManualPersonObject::new(name, address).upcast::<glib::Object>()
    }

    pub fn static_type() -> glib::Type {
        wrp::ManualPersonObject::static_type()
    }
}

pub struct ManualAddressObject {}
impl ManualAddressObject {
    pub fn new(street: String) -> wrp::ManualAddressObject {
        wrp::ManualAddressObject::new(street)
    }

    pub fn static_type() -> glib::Type {
        wrp::ManualAddressObject::static_type()
    }
}