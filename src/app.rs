use crate::{
    category::Category,
    hn_api::{create_client, fetch_category},
    story::StoryDisplay,
    ui,
};
use eframe::egui;
use reqwest::Client;
use std::sync::{Arc, Mutex};

pub struct HackerNewsApp {
    pub current_category: Category,
    pub stories: Vec<StoryDisplay>,
    pub is_loading: bool,
    pub error_message: Option<String>,
    runtime: tokio::runtime::Runtime,
    client: Client,
    pending_stories: Arc<Mutex<Option<Vec<StoryDisplay>>>>,
    pending_error: Arc<Mutex<Option<String>>>,
}

impl HackerNewsApp {
    pub fn new() -> Self {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let client = create_client();

        let mut app = Self {
            current_category: Category::default(),
            stories: Vec::new(),
            is_loading: false,
            error_message: None,
            runtime,
            client,
            pending_stories: Arc::new(Mutex::new(None)),
            pending_error: Arc::new(Mutex::new(None)),
        };

        app.fetch_current_category();
        app
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

        ui::render_sidebar(ctx, self);
        ui::render_story_list(ctx, self);

        ctx.request_repaint();
    }
}
