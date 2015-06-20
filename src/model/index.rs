use ::glob;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SortField {
    Date, Title
}

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum SortDirection {
    Ascending, Descending
}

#[derive(Default, Clone, Debug, Copy, Eq, PartialEq)]
pub struct Sort {
    pub field: SortField,
    pub direction: SortDirection,
}

impl Default for SortField {
    fn default() -> SortField {
        SortField::Title
    }
}

impl Default for SortDirection {
    fn default() -> SortDirection {
        SortDirection::Descending
    }
}

#[derive(Debug, Clone)]
pub struct Index {
    pub sort: Sort,
    pub directories: Vec<glob::Pattern>,
    pub paginate: Option<u32>,
    pub max: Option<u32>,
}