# Hacker News egui App Design

**Date:** 2025-01-18
**Author:** Claude Code
**Status:** Approved

## Overview

A cross-platform desktop application built with Rust and egui that fetches and displays stories from Hacker News across multiple categories.

## Requirements

### Functional Requirements
- Display Hacker News stories from 6 categories: Top, New, Best, Ask, Show, Jobs
- Show story details: title, metadata (points, author, time), URL, comment count
- Allow users to open original articles and HN comment threads
- Manual refresh of story data
- Error handling with inline error messages

### Non-Functional Requirements
- Fast startup and responsive UI
- Handle API failures gracefully
- Limit to 30 stories per category for performance

## Architecture

### Application Structure

```
my_egui_pro/
├── Cargo.toml
├── src/
│   ├── main.rs           # Entry point, eframe setup
│   ├── app.rs            # HackerNewsApp struct
│   ├── story.rs          # Story model & display logic
│   ├── category.rs       # Category enum & API endpoints
│   ├── hn_api.rs         # Hacker News API client
│   └── ui.rs             # UI rendering code
├── docs/
│   └── plans/
│       └── 2025-01-18-hacker-news-egui-app-design.md
└── .gitignore
```

### Core Components

**HackerNewsApp** (main state container):
- `categories: [Category; 6]` - All available categories
- `current_category: Category` - Currently selected
- `stories: Vec<StoryDisplay>` - Fetched stories
- `is_loading: bool` - Loading state
- `error_message: Option<String>` - Error display
- `runtime: Option<tokio::runtime::Runtime>` - Async runtime

## Data Structures

```rust
#[derive(Debug, Clone, Deserialize)]
struct Story {
    id: u64,
    title: String,
    url: Option<String>,
    by: String,
    score: i32,
    time: u64,
    descendants: Option<u64>,
}

#[derive(Clone)]
struct StoryDisplay {
    story: Story,
    domain: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    Top,    // /topstories.json
    New,    // /newstories.json
    Best,   // /beststories.json
    Ask,    // /askstories.json
    Show,   // /showstories.json
    Jobs,   // /jobstories.json
}
```

## UI Design

### Layout
- **Sidebar** (left, ~200px, resizable): Category buttons
- **Central Panel** (right): Scrollable story list

### Sidebar
- Vertical list of 6 category buttons
- Highlight selected category
- Click triggers category fetch

### Story List
Each story card displays:
```
[Bold Title]                     (domain.com)

⬤ 42 points by author | 2 hours ago

💬 15 comments    [🔗 Story] [💬 Discuss]
```

### States
- **Loading**: Spinner at top of list
- **Error**: Red inline message with retry button
- **Empty**: "No stories available"

## API Integration

### Hacker News Firebase API
- Base URL: `https://hacker-news.firebaseio.com/v0/`
- Category endpoints return array of story IDs
- Item endpoint: `/item/{id}.json`

### Fetch Flow
1. GET `{category}.json` → Vec<u64> (story IDs)
2. Take first 30 IDs
3. Parallel fetch: GET `/item/{id}.json` for each
4. Parse and return Vec<Story>

### Async Strategy
- Use `tokio::runtime::Runtime` stored in app state
- Spawn blocking tasks for API calls
- Update state on completion

## Error Handling

| Error Type          | Display                         | Action          |
|---------------------|---------------------------------|-----------------|
| Network failure     | "Failed to fetch: {reason}"     | Retry button    |
| Parse error         | "Invalid data from API"         | Retry button    |
| Timeout             | "Request timed out"             | Retry button    |
| No URL on story     | Open HN item page instead       | Automatic       |

## Dependencies

```toml
[dependencies]
eframe = "0.27"
egui = "0.27"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1.0", features = ["full"] }
opener = "0.7"
time = "0.3"
```

## Build & Run

```bash
cargo run
```

- Window size: 1000x700
- Window title: "Hacker News Reader"

## Implementation Notes

- YAGNI: No caching, no search, no favorites - keep it simple
- Story limit: 30 per category balances completeness vs performance
- Manual refresh only - no auto-refresh to avoid API spam
- Inline errors provide clear feedback without blocking UI
