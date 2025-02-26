use std::collections::HashMap;
use std::time::{Duration, SystemTime};

const DEFAULT_CACHE_VALIDITY: Duration = Duration::from_secs(60);

#[derive(Clone)]
pub(crate) struct Cache<T: Clone> {
    inner: HashMap<String, CacheItem<T>>,
    cache_validity_duration: Duration,
}

impl<T: Clone> Cache<T> {
    pub(crate) fn new() -> Self {
        Cache {
            inner: HashMap::new(),
            cache_validity_duration: DEFAULT_CACHE_VALIDITY,
        }
    }

    // it felt like this might be an useful addition if we want to keep our caches with different
    // validity durations
    #[allow(unused)]
    pub(crate) fn with_validity_duration(mut self, new_cache_validity: Duration) -> Self {
        self.cache_validity_duration = new_cache_validity;
        self
    }

    pub(crate) fn len(&self) -> usize {
        self.inner.len()
    }

    pub(crate) fn get_all(&self) -> Vec<T> {
        self.inner
            .values()
            .map(|cache_item| cache_item.value.clone())
            .collect()
    }

    pub(crate) fn get(&self, key: &str) -> Option<T> {
        self.inner
            .get(key)
            .filter(|cache_item| cache_item.valid_until >= SystemTime::now())
            .map(|cache_item| cache_item.value.clone())
    }

    pub(crate) fn set(&mut self, key: &str, value: T) {
        self.inner.insert(
            key.to_string(),
            CacheItem {
                valid_until: SystemTime::now() + self.cache_validity_duration,
                value,
            },
        );
    }

    #[allow(unused)]
    pub(crate) fn remove(&mut self, key: &str) -> Option<T> {
        self.inner.remove(key).map(|item| item.value)
    }

    #[allow(unused)]
    pub(crate) fn remove_if_expired(&mut self, key: &str) -> Option<T> {
        if self.inner.get(key)?.has_expired() {
            self.remove(key)
        } else {
            None
        }
    }

    // it seems like something should be running on timer calling this method on all of our caches
    #[allow(unused)]
    pub(crate) fn remove_all_expired(&mut self) {
        self.inner.retain(|_, v| !v.has_expired())
    }
}

#[derive(Clone)]
pub(crate) struct CacheItem<T> {
    pub(crate) value: T,
    pub(crate) valid_until: std::time::SystemTime,
}

impl<T> CacheItem<T> {
    fn has_expired(&self) -> bool {
        let now = SystemTime::now();
        self.valid_until < now
    }
}
