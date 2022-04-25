//! Simple in-memory key-value store.
#![deny(missing_docs)]
use std::collections::HashMap;

/// An in-memory key-value store for
/// [Strings](https://doc.rust-lang.org/std/string/struct.String.html) using a
/// [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html).
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    /// Create a blank `KvStore`. Equivalent to
    /// [HashMap::new()](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.new)
    pub fn new() -> KvStore {
        KvStore {
            store: HashMap::new()
        }
    }

    /// Store a key-value pair.
    /// ``` rust
    /// # use std::error::Error;
    /// # use kvs::KvStore;
    /// #
    /// # pub fn main() -> Result<(), Box<dyn Error>> {
    ///     let mut store = KvStore::new();
    ///     store.set(String::from("Alice"), String::from("Curiouser"));
    /// #     Ok(())
    /// # }
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    /// Retrieve a cloned copy of the value for a given key, or `None` if the key is not present in
    /// the store.
    /// ```rust
    /// # use std::error::Error;
    /// # use std::fmt::format;
    /// # use kvs::KvStore;
    /// #
    /// # pub fn main() -> Result<(), Box<dyn Error>> {
    ///     let mut store = KvStore::new();
    ///     let name = "White Rabbit";
    ///     let state = "Late, for a very important date";
    ///     store.set(String::from(name), String::from(state));
    ///     let val = store.get(String::from(name));
    /// #   Ok(())
    // ///     match val {
    // ///         Some(val) => {
    // ///             if val == state {
    // ///                 Ok(())
    // ///             } else {
    // ///                 Err(format("Didn't retrieve expected state `{}`. Instead got `{}`", state, val))
    // ///             }
    // ///         }
    // ///         None => Err(format("Received no value for key `{}`", name))
    // ///     }
    /// # }
    /// ```
    pub fn get(&self, key: String) -> Option<String> {
        let val = self.store.get(&key)?;
        Some((*val).clone())
    }

    /// Ensure that there is no value for a given key. Does nothing if the key is not present in
    /// the store.
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
