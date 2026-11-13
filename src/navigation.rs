#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Url(String);

impl Url {
    pub fn parse(input: impl Into<String>) -> Self {
        let value = input.into();
        if value.contains("://") {
            Self(value)
        } else {
            Self(format!("https://{value}"))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Navigation {
    current: Url,
    history: Vec<Url>,
    history_index: usize,
}

impl Navigation {
    pub fn new(initial: Url) -> Self {
        Self {
            current: initial.clone(),
            history: vec![initial],
            history_index: 0,
        }
    }

    pub fn current(&self) -> &Url {
        &self.current
    }

    pub fn navigate(&mut self, url: Url) {
        self.history.truncate(self.history_index + 1);
        self.history.push(url.clone());
        self.history_index += 1;
        self.current = url;
    }

    pub fn can_go_back(&self) -> bool {
        self.history_index > 0
    }

    pub fn can_go_forward(&self) -> bool {
        self.history_index + 1 < self.history.len()
    }

    pub fn back(&mut self) -> Option<&Url> {
        if !self.can_go_back() {
            return None;
        }

        self.history_index -= 1;
        self.current = self.history[self.history_index].clone();
        Some(&self.current)
    }

    pub fn forward(&mut self) -> Option<&Url> {
        if !self.can_go_forward() {
            return None;
        }

        self.history_index += 1;
        self.current = self.history[self.history_index].clone();
        Some(&self.current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigation_preserves_history() {
        let mut navigation = Navigation::new(Url::parse("example.com"));
        navigation.navigate(Url::parse("rust-lang.org"));
        assert_eq!(navigation.current().as_str(), "https://rust-lang.org");
        assert!(navigation.can_go_back());

        navigation.back();
        assert_eq!(navigation.current().as_str(), "https://example.com");
        assert!(navigation.can_go_forward());
    }
}
