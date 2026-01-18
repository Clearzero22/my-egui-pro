use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Category {
    #[default]
    Top,
    New,
    Best,
    Ask,
    Show,
    Jobs,
}

impl Category {
    pub const ALL: [Category; 6] = [
        Category::Top,
        Category::New,
        Category::Best,
        Category::Ask,
        Category::Show,
        Category::Jobs,
    ];

    pub fn api_endpoint(&self) -> &'static str {
        match self {
            Category::Top => "topstories",
            Category::New => "newstories",
            Category::Best => "beststories",
            Category::Ask => "askstories",
            Category::Show => "showstories",
            Category::Jobs => "jobstories",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Category::Top => "Top",
            Category::New => "New",
            Category::Best => "Best",
            Category::Ask => "Ask",
            Category::Show => "Show",
            Category::Jobs => "Jobs",
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
