use std::{collections::HashMap, hash::Hash};

use crate::{object_format::data_value::hashmap::DataValueHashmap, rof_compat::RofCompat};

impl<K: RofCompat + Hash + Eq, V: RofCompat> RofCompat for HashMap<K, V> {
    fn serialize(&self) -> Box<dyn crate::object_format::DataValue> {
        Box::new(DataValueHashmap::new(
            K::default().serialize().get_type(),
            V::default().serialize().get_type(),
            self.iter()
                .map(|(item_key, item_value)| (item_key.serialize(), item_value.serialize()))
                .collect(),
        ))
    }

    fn deserialize(rof_object: Box<dyn crate::object_format::DataValue>) -> Self {
        rof_object
            .as_hashmap()
            .into_iter()
            .map(|(item_key, item_value)| (K::deserialize(item_key), V::deserialize(item_value)))
            .collect()
    }
}
