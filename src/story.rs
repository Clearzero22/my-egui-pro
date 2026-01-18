use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Debug, Clone, Deserialize)]
pub struct Story {
    pub id: u64,
    pub title: String,
    #[serde(default)]
    pub url: Option<String>,
    pub by: String,
    pub score: i32,
    pub time: u64,
    #[serde(default)]
    pub descendants: Option<u64>,
}

#[derive(Clone)]
pub struct StoryDisplay {
    pub story: Story,
    pub domain: Option<String>,
}

impl StoryDisplay {
    pub fn from_story(story: Story) -> Self {
        let domain = story.url.as_ref().and_then(|url| {
            url.parse::<reqwest::Url>()
                .ok()
                .and_then(|parsed| parsed.host_str().map(|s| s.to_string()))
        });

        Self { story, domain }
    }

    pub fn time_ago(&self) -> String {
        let Ok(datetime) = OffsetDateTime::from_unix_timestamp(self.story.time as i64) else {
            return "unknown time".to_string();
        };
        let now = OffsetDateTime::now_utc();
        let duration = now - datetime;

        if duration.whole_hours() > 24 {
            format!("{} days ago", duration.whole_days())
        } else if duration.whole_hours() > 0 {
            format!("{} hours ago", duration.whole_hours())
        } else if duration.whole_minutes() > 0 {
            format!("{} minutes ago", duration.whole_minutes())
        } else {
            "just now".to_string()
        }
    }

    pub fn hn_url(&self) -> String {
        format!("https://news.ycombinator.com/item?id={}", self.story.id)
    }
}
