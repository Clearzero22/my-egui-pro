# Favorites Feature Design

**Date:** 2025-01-18
**Author:** Claude Code
**Status:** Approved

## Overview

Add ability to save favorite Hacker News stories, stored locally in SQLite, with a view toggle to switch between fetched stories and saved favorites.

## Requirements

### Functional Requirements
- Star button on each story to save/unsave as favorite
- Toggle button to switch between "Fetched Stories" and "Saved Favorites"
- Favorites persist across app restarts
- Favorites sorted by story date (newest first)

### Non-Functional Requirements
- Database errors handled gracefully with inline messages
- Cross-platform storage location
- Fast local queries

## Architecture

### Database Schema

```sql
CREATE TABLE favorites (
    id INTEGER PRIMARY KEY,  -- HN story ID
    title TEXT NOT NULL,
    url TEXT,
    by TEXT NOT NULL,
    score INTEGER NOT NULL,
    time INTEGER NOT NULL,  -- Unix timestamp
    descendants INTEGER,
    saved_at INTEGER NOT NULL  -- When user saved it
);

CREATE INDEX idx_time ON favorites(time DESC);
```

### Database Location

| Platform | Path |
|----------|------|
| Linux    | `~/.local/share/my_egui_pro/favorites.db` |
| macOS    | `~/Library/Application Support/my_egui_pro/favorites.db` |
| Windows  | `%APPDATA%\my_egui_pro\favorites.db` |

## UI Design

### Story Card Updates
Add star button (⭐/☆) alongside existing buttons:
- Filled star (⭐) = saved as favorite
- Empty star (☆) = not saved
- Click to toggle state

### Sidebar Updates
Add view toggle at top:
```
[Fetched] [Saved]
--- Categories below ---
Top
New
Best
...
```

### View Modes

**Fetched Mode (current behavior):**
- Shows stories from HN API
- Refresh button active
- Star button shows save state

**Saved Mode:**
- Shows favorites from database
- Sorted by story time DESC
- "Saved Favorites (N)" header
- Refresh button hidden (data is local)
- All stars filled (⭐)

## Data Structures

### App State Addition
```rust
pub struct HackerNewsApp {
    // ... existing fields ...

    // New fields
    view_mode: ViewMode,
    db: FavoritesDB,
    favorite_ids: HashSet<u64>,  // Cache for quick lookup
    saved_stories: Vec<StoryDisplay>,
}

pub enum ViewMode {
    Fetched,
    Saved,
}
```

### New Module: storage.rs
```rust
pub struct FavoritesDB {
    conn: Connection,
}

impl FavoritesDB {
    pub fn new() -> Result<Self, Error>
    pub fn add_favorite(&self, story: &Story) -> Result<()>
    pub fn remove_favorite(&self, id: u64) -> Result<()>
    pub fn is_favorite(&self, id: u64) -> Result<bool>
    pub fn get_all(&self) -> Result<Vec<Story>>
}
```

## Dependencies

Add to `Cargo.toml`:
```toml
[dependencies]
# ... existing ...
rusqlite = { version = "0.32", features = ["bundled"] }
dirs = "5.0"
```

## Error Handling

| Error Type | Display | Action |
|------------|---------|--------|
| DB open failed | "Database error: {reason}" inline | Show in both views |
| Add failed | Silent | User can retry |
| Load failed | "Failed to load favorites" inline | Show in saved view |
| Remove failed | Silent | User can retry |

## Implementation Notes

- **YAGNI**: No export, no tags, no folders - just favorites
- **Cache**: `favorite_ids` HashSet for O(1) lookup in UI
- **Initial load**: Populate `favorite_ids` and `saved_stories` on startup
- **Toggle sync**: When toggling star, update DB + cache + current view
- **Sort by story time**: Not saved_at - users want newest stories first

## Build & Run

```bash
cargo run
```

First run creates database file. Favorites persist across sessions.
