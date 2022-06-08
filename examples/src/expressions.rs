use gtk4_helper::{
    gtk,
    model::prelude::*
};
use crate::gtk::Expression;

use crate::models::{Person, get_persons, Address};

#[allow(dead_code)]
pub fn test() {
    if let Some(person) = get_persons(1).iter().next() {
        let obj: glib::Object = person.to_object();
        if let Some(address) = obj.property_value(Person::address).get::<String>().ok() {
            println!("Ok: {}", address);
        }

        let address_exp =
            gtk::PropertyExpression::new(Person::static_type(), Option::<&Expression>::None, Person::address);
        let street_exp =
            gtk::PropertyExpression::new(Address::static_type(), Some(&address_exp), Address::street);

        if let Some(val)  = street_exp.evaluate(Some(&obj))
            .and_then(|val| val.get::<String>().ok())
        {
            println!("Ok: {}", val);
        }
    }
}