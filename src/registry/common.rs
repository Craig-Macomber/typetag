use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use serde::de::{self, Expected};

use crate::private::{DeserializeFn, DeserializerRegistry};

#[doc(hidden)]
pub struct Registry<T: ?Sized> {
    #[doc(hidden)]
    pub map: BTreeMap<&'static str, Option<DeserializeFn<T>>>,
    #[doc(hidden)]
    pub names: Vec<&'static str>,
}

#[cfg(feature = "runtime")]
impl<T: ?Sized> Default for Registry<T> {
    #[must_use]
    fn default() -> Self {
        let map = std::collections::BTreeMap::new();
        let names = std::vec::Vec::new();

        Self { map, names }
    }
}

impl<T: ?Sized> DeserializerRegistry<T> for Registry<T> {
    fn get_deserializer<E>(
        &'static self,
        key: &str,
        expected: &dyn Expected,
    ) -> Result<DeserializeFn<T>, E>
    where
        E: serde::de::Error,
    {
        match self.map.get(key) {
            Some(Some(value)) => Ok(*value),
            Some(None) => Err(de::Error::custom(format_args!(
                "non-unique tag of {}: {:?}",
                expected, key
            ))),
            None => Err(de::Error::unknown_variant(key, &self.names)),
        }
    }
}
