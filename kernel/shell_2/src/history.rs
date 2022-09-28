use alloc::{borrow::ToOwned, string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub(crate) struct History {
    commands: Vec<String>,
    /// The currently selected command.
    ///
    /// ```text
    ///           +-------+-------+-------+-------+-------+
    /// commands: |   0   |   1   |   2   |   3   |   4   |
    ///           +-------+-------+-------+-------+-------+   
    ///               ^       ^       ^       ^       ^       ^
    ///               |       |       |       |       |       |
    ///    index:   last   past(3) past(2) past(1) past(0) current
    /// ```
    index: Index,
    current: String,
}

impl History {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn reset(&mut self) {
        self.commands.dedup();
        self.index = Index::Current;
        // We don't need to clear self.current as it will be overwritten on the
        // next call of self.previous.
    }

    pub(crate) fn previous(&mut self, current: &str) -> Option<&str> {
        match self.index {
            Index::Current => {
                self.current = current.to_owned();
                if self.commands.is_empty() {
                    self.index = Index::Last;
                    None
                } else if self.commands.len() == 1 {
                    self.index = Index::Last;
                    Some(&self.commands[0])
                } else {
                    self.index = Index::Past(0);
                    let vec_idx = self.commands.len() - 1;
                    Some(&self.commands[vec_idx])
                }
            }
            Index::Past(mut idx) => {
                if idx + 2 == self.commands.len() {
                    self.index = Index::Last;
                    Some(&self.commands[0])
                } else {
                    idx += 1;
                    self.index = Index::Past(idx);
                    let vec_idx = self.commands.len() - (idx + 1);
                    Some(&self.commands[vec_idx])
                }
            }
            Index::Last => None,
        }
    }

    pub(crate) fn next(&mut self) -> Option<&str> {
        match self.index {
            Index::Current => None,
            Index::Past(mut idx) => {
                if idx == 0 {
                    self.index = Index::Current;
                    Some(&self.current)
                } else {
                    idx -= 1;
                    self.index = Index::Past(idx);
                    let vec_idx = self.commands.len() - (idx + 1);
                    Some(&self.commands[vec_idx])
                }
            }
            Index::Last => {
                if self.commands.is_empty() {
                    None
                } else if self.commands.len() == 1 {
                    self.index = Index::Current;
                    None
                } else {
                    self.index = Index::Past(self.commands.len() - 2);
                    Some(&self.commands[1])
                }
            }
        }
    }

    pub(crate) fn push(&mut self, command: String) {
        self.reset();
        self.commands.push(command);
    }
}

/// A command index.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Index {
    Current,
    Past(usize),
    Last,
}

impl Default for Index {
    fn default() -> Self {
        Self::Current
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::borrow::ToOwned;

    #[test]
    fn test_history() {
        let mut history = History::new();

        assert_eq!(history.index, Index::Current);
        assert_eq!(history.previous(), None);
        assert_eq!(history.index, Index::Last);
        assert_eq!(history.previous(), None);
        assert_eq!(history.index, Index::Last);
        assert_eq!(history.next(), None);

        history.push("you".to_owned());
        assert_eq!(history.index, Index::Current);

        assert_eq!(history.previous(), Some("you"));
        assert_eq!(history.index, Index::Last);

        assert_eq!(history.next(), None);
        assert_eq!(history.index, Index::Current);

        history.push("walk".to_owned());
        assert_eq!(history.index, Index::Current);

        history.push("around".to_owned());
        assert_eq!(history.index, Index::Current);

        assert_eq!(history.previous(), Some("around"));
        assert_eq!(history.index, Index::Past(0));

        assert_eq!(history.previous(), Some("walk"));
        assert_eq!(history.index, Index::Past(1));

        assert_eq!(history.next(), Some("around"));
        assert_eq!(history.index, Index::Past(0));

        assert_eq!(history.next(), None);
        assert_eq!(history.index, Index::Current);

        assert_eq!(history.previous(), Some("around"));
        assert_eq!(history.index, Index::Past(0));

        assert_eq!(history.previous(), Some("walk"));
        assert_eq!(history.index, Index::Past(1));

        assert_eq!(history.previous(), Some("you"));
        assert_eq!(history.index, Index::Last);

        assert_eq!(history.previous(), None);
        assert_eq!(history.index, Index::Last);

        assert_eq!(history.next(), Some("walk"));
        assert_eq!(history.index, Index::Past(1));

        assert_eq!(history.previous(), Some("you"));
        assert_eq!(history.index, Index::Last);
    }
}
