use chrono::Utc;
use ratatui::widgets::ListState;

const SPINNER: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub struct Spinner {
    elements: Vec<String>,
}

impl Default for Spinner {
    fn default() -> Self {
        Self {
            elements: SPINNER.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
        }
    }
}

impl ToString for Spinner {
    fn to_string(&self) -> String {
        let cycle = 1500; //millisec
        self.elements[((Utc::now().timestamp_millis() % cycle)
            / (cycle / self.elements.len() as i64)) as usize]
            .to_owned()
    }
}

#[derive(Clone)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
    pub header_size: usize,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
            header_size: 2,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 + self.header_size {
                    0 + self.header_size
                } else {
                    i + 1
                }
            }
            None => 0 + self.header_size,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i <= 0 + self.header_size {
                    self.items.len() - 1 + self.header_size
                } else {
                    i - 1
                }
            }
            None => 0 + self.header_size,
        };
        self.state.select(Some(i));
    }

    pub fn get_selected_item_index(&self) -> Option<usize> {
        self.state.selected().map(|state| state - self.header_size)
    }

    /*
    fn unselect(&mut self) {
        self.state.select(None);
    }
    */
}
