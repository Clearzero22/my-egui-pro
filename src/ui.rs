use eframe::egui;
use crate::{app::{HackerNewsApp, ViewMode}, story::StoryDisplay};

pub fn render_sidebar(ctx: &egui::Context, app: &mut HackerNewsApp) {
    egui::SidePanel::left("sidebar")
        .default_width(200.0)
        .show(ctx, |ui| {
            ui.heading("View");
            ui.add_space(5.0);

            // View mode toggle
            ui.horizontal(|ui| {
                let fetched_selected = app.view_mode == ViewMode::Fetched;
                let saved_selected = app.view_mode == ViewMode::Saved;

                if ui.selectable_label(fetched_selected, "Fetched").clicked() {
                    app.set_view_mode(ViewMode::Fetched);
                }

                if ui.selectable_label(saved_selected, "Saved").clicked() {
                    app.set_view_mode(ViewMode::Saved);
                }
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // Theme toggle
            ui.heading("Theme");
            ui.add_space(5.0);

            if ui.button(app.theme.display_name()).clicked() {
                app.toggle_theme();
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // Categories (only show in Fetched mode)
            if app.view_mode == ViewMode::Fetched {
                ui.heading("Categories");
                ui.add_space(10.0);

                for category in crate::category::Category::ALL {
                    let is_selected = app.current_category == category;

                    if ui.selectable_label(is_selected, category.display_name()).clicked() {
                        app.current_category = category;
                        app.fetch_current_category();
                    }
                }

                ui.add_space(20.0);

                if ui.button("🔄 Refresh").clicked() {
                    app.fetch_current_category();
                }
            } else {
                ui.heading("Saved Favorites");
                ui.add_space(10.0);
                ui.label(format!("{} stories", app.saved_stories.len()));
            }
        });
}

pub fn render_story_list(ctx: &egui::Context, app: &mut HackerNewsApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let (title, show_refresh) = match app.view_mode {
            ViewMode::Fetched => (format!("{} Stories", app.current_category.display_name()), true),
            ViewMode::Saved => (format!("Saved Favorites ({})", app.saved_stories.len()), false),
        };

        ui.heading(title);

        if let Some(ref error) = app.error_message {
            ui.add_space(10.0);
            ui.colored_label(egui::Color32::RED, format!("❌ {}", error));
            if show_refresh && ui.button("🔄 Retry").clicked() {
                app.fetch_current_category();
            }
            return;
        }

        if app.is_loading {
            ui.add_space(10.0);
            ui.spinner();
            ui.label("Loading...");
            return;
        }

        // Collect story data to avoid borrow conflicts
        let stories: Vec<(StoryDisplay, bool)> = match app.view_mode {
            ViewMode::Fetched => {
                app.stories.iter()
                    .map(|s| (s.clone(), app.is_favorite(s.story.id)))
                    .collect()
            }
            ViewMode::Saved => {
                app.saved_stories.iter()
                    .map(|s| (s.clone(), true))
                    .collect()
            }
        };

        if stories.is_empty() {
            ui.add_space(10.0);
            let msg = match app.view_mode {
                ViewMode::Fetched => "No stories available.",
                ViewMode::Saved => "No saved favorites yet. Click the ⭐ button to save stories.",
            };
            ui.label(msg);
            return;
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
            for (story_display, is_favorite) in stories {
                render_story_card(ui, app, &story_display, is_favorite);
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
            }
        });
    });
}

fn render_story_card(ui: &mut egui::Ui, app: &mut HackerNewsApp, story_display: &StoryDisplay, is_favorite: bool) {
    let story = &story_display.story;

    ui.horizontal(|ui| {
        ui.label(format!("⬤ {} points by {} | {}", story.score, story.by, story_display.time_ago()));
    });

    ui.add_space(5.0);

    let hn_url = story_display.hn_url();
    let url_to_open = story.url.as_ref().unwrap_or(&hn_url);
    ui.hyperlink_to(format!("🔗 {}", story.title), url_to_open);

    if let Some(ref domain) = story_display.domain {
        ui.label(format!("({})", domain));
    }

    ui.add_space(5.0);

    ui.horizontal(|ui| {
        ui.label(format!("💬 {} comments", story.descendants.unwrap_or(0)));

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // Star button
            let star_label = if is_favorite { "⭐" } else { "☆" };
            if ui.button(star_label).clicked() {
                app.toggle_favorite(story);
            }

            if ui.button("💬 Discuss").clicked() {
                opener::open(story_display.hn_url().as_str()).ok();
            }

            if let Some(ref url) = story.url {
                if ui.button("🔗 Story").clicked() {
                    opener::open(url.as_str()).ok();
                }
            } else if ui.button("🔗 Story").clicked() {
                opener::open(story_display.hn_url().as_str()).ok();
            }
        });
    });
}
