use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub(crate) struct PageWire<T> {
    current_page: u32,
    per_page: u32,
    data: Vec<T>,
    next_page_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct Page<T> {
    pub items: Vec<T>,
    pub page: u32,
    pub per_page: u32,
    pub has_next_page: bool,
}

impl<T> From<PageWire<T>> for Page<T> {
    fn from(w: PageWire<T>) -> Self {
        Self {
            items: w.data,
            page: w.current_page,
            per_page: w.per_page,
            has_next_page: w.next_page_url.is_some(),
        }
    }
}
