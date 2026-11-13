use crate::navigation::{Navigation, Url};

#[derive(Debug)]
pub struct Tab {
    pub id: u64,
    pub title: String,
    pub navigation: Navigation,
}

impl Tab {
    pub fn new(id: u64, url: Url) -> Self {
        Self {
            id,
            title: "New Tab".to_string(),
            navigation: Navigation::new(url),
        }
    }
}

#[derive(Debug, Default)]
pub struct TabManager {
    tabs: Vec<Tab>,
    active_index: usize,
    next_id: u64,
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active_index: 0,
            next_id: 1,
        }
    }

    pub fn open(&mut self, url: Url) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.tabs.push(Tab::new(id, url));
        self.active_index = self.tabs.len() - 1;
        id
    }

    pub fn active(&self) -> Option<&Tab> {
        self.tabs.get(self.active_index)
    }

    pub fn active_mut(&mut self) -> Option<&mut Tab> {
        self.tabs.get_mut(self.active_index)
    }

    pub fn len(&self) -> usize {
        self.tabs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opening_tab_selects_it() {
        let mut tabs = TabManager::new();
        let id = tabs.open(Url::parse("example.com"));

        assert_eq!(tabs.len(), 1);
        assert_eq!(tabs.active().unwrap().id, id);
    }
}
