use crate::{
    category::Category,
    hn_api::{create_client, fetch_category},
    story::{Story, StoryDisplay},
    storage::FavoritesDB,
    config::Config,
    theme::{GruvboxTheme, apply_theme},
    ui,
};
use eframe::egui;
use reqwest::Client;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Fetched,
    Saved,
}

pub struct HackerNewsApp {
    pub current_category: Category,
    pub view_mode: ViewMode,
    pub stories: Vec<StoryDisplay>,
    pub saved_stories: Vec<StoryDisplay>,
    pub favorite_ids: HashSet<u64>,
    pub is_loading: bool,
    pub error_message: Option<String>,
    pub theme: GruvboxTheme,
    runtime: tokio::runtime::Runtime,
    client: Client,
    db: FavoritesDB,
    config: Config,
    pending_stories: Arc<Mutex<Option<Vec<StoryDisplay>>>>,
    pending_error: Arc<Mutex<Option<String>>>,
}

impl HackerNewsApp {
    pub fn new() -> Self {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let client = create_client();
        let db = FavoritesDB::new().unwrap_or_else(|e| {
            eprintln!("Database error: {}", e);
            panic!("Failed to open favorites database");
        });

        let config = Config::new();
        let app_config = config.load();
        let theme = app_config.theme;

        let favorite_ids = Self::load_favorites(&db);
        let saved_stories = Self::load_saved_stories(&db);

        let mut app = Self {
            current_category: Category::default(),
            view_mode: ViewMode::Fetched,
            stories: Vec::new(),
            saved_stories,
            favorite_ids,
            is_loading: false,
            error_message: None,
            theme,
            runtime,
            client,
            db,
            config,
            pending_stories: Arc::new(Mutex::new(None)),
            pending_error: Arc::new(Mutex::new(None)),
        };

        app.fetch_current_category();
        app
    }

    fn load_favorites(db: &FavoritesDB) -> HashSet<u64> {
        db.get_all()
            .unwrap_or_default()
            .into_iter()
            .map(|s| s.id)
            .collect()
    }

    fn load_saved_stories(db: &FavoritesDB) -> Vec<StoryDisplay> {
        db.get_all()
            .unwrap_or_default()
            .into_iter()
            .map(StoryDisplay::from_story)
            .collect()
    }

    pub fn fetch_current_category(&mut self) {
        self.is_loading = true;
        self.error_message = None;

        let category = self.current_category;
        let client = self.client.clone();
        let pending_stories = self.pending_stories.clone();
        let pending_error = self.pending_error.clone();

        self.runtime.spawn(async move {
            match fetch_category(&client, category).await {
                Ok(stories) => {
                    let displays: Vec<StoryDisplay> = stories.into_iter().map(StoryDisplay::from_story).collect();
                    *pending_stories.lock().unwrap() = Some(displays);
                }
                Err(e) => {
                    *pending_error.lock().unwrap() = Some(format!("Failed to fetch: {}", e));
                }
            }
        });
    }

    pub fn toggle_favorite(&mut self, story: &Story) {
        if self.favorite_ids.contains(&story.id) {
            let _ = self.db.remove_favorite(story.id);
            self.favorite_ids.remove(&story.id);
            self.saved_stories.retain(|s| s.story.id != story.id);
        } else {
            let _ = self.db.add_favorite(story);
            self.favorite_ids.insert(story.id);
            let display = StoryDisplay::from_story(story.clone());
            self.saved_stories.push(display);
            self.saved_stories.sort_by(|a, b| b.story.time.cmp(&a.story.time));
        }
    }

    pub fn is_favorite(&self, id: u64) -> bool {
        self.favorite_ids.contains(&id)
    }

    pub fn set_view_mode(&mut self, mode: ViewMode) {
        self.view_mode = mode;
        self.error_message = None;

        if mode == ViewMode::Fetched && self.stories.is_empty() && !self.is_loading {
            self.fetch_current_category();
        }
    }

    pub fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            GruvboxTheme::Dark => GruvboxTheme::Light,
            GruvboxTheme::Light => GruvboxTheme::Dark,
        };

        let updated_config = crate::config::AppConfig {
            theme: self.theme,
        };
        let _ = self.config.save(&updated_config);
    }

    fn check_pending_updates(&mut self) {
        if let Ok(mut guard) = self.pending_stories.try_lock() {
            if let Some(stories) = guard.take() {
                self.stories = stories;
                self.is_loading = false;
                self.error_message = None;
            }
        }

        if let Ok(mut guard) = self.pending_error.try_lock() {
            if let Some(error) = guard.take() {
                self.error_message = Some(error);
                self.is_loading = false;
            }
        }
    }
}

impl eframe::App for HackerNewsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.check_pending_updates();

        apply_theme(ctx, &self.theme);

        ui::render_sidebar(ctx, self);
        ui::render_story_list(ctx, self);

        ctx.request_repaint();
    }
}
