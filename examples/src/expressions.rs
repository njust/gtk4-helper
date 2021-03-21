use gtk4_helper::{
    gtk4,
    model::prelude::*
};

use crate::models::{Person, get_persons, Address};
use gtk4_helper::gtk4::NONE_EXPRESSION;

#[allow(dead_code)]
pub fn test() {
    if let Some(person) = get_persons(1).iter().next() {
        let obj: glib::Object = person.to_object();
        if let Some(address) = obj.get_property(Person::address).ok()
            .and_then(|val| val.get::<glib::Object>().ok())
            .and_then(|obj| obj)
            .and_then(|obj| obj.get_property(Address::street).ok())
            .and_then(|val| val.get::<String>().ok())
            .and_then(|s|s)
        {
            println!("Ok: {}", address);
        }

        let address_exp =
            gtk4::PropertyExpression::new(Person::static_type(), NONE_EXPRESSION, Person::address);
        let street_exp =
            gtk4::PropertyExpression::new(Address::static_type(), Some(&address_exp), Address::street);

        if let Some(val)  = street_exp.evaluate(Some(&obj))
            .and_then(|val| val.get::<String>().ok())
            .and_then(|s|s)
        {
            println!("Ok: {}", val);

        }
    }
}