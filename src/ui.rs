use eframe::egui;
use crate::{app::HackerNewsApp, story::StoryDisplay};

pub fn render_sidebar(ctx: &egui::Context, app: &mut HackerNewsApp) {
    egui::SidePanel::left("sidebar")
        .default_width(200.0)
        .show(ctx, |ui| {
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
        });
}

pub fn render_story_list(ctx: &egui::Context, app: &mut HackerNewsApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading(format!("{} Stories", app.current_category.display_name()));

        if let Some(ref error) = app.error_message {
            ui.add_space(10.0);
            ui.colored_label(egui::Color32::RED, format!("❌ {}", error));
            if ui.button("🔄 Retry").clicked() {
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

        if app.stories.is_empty() {
            ui.add_space(10.0);
            ui.label("No stories available.");
            return;
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
            for story_display in &app.stories {
                render_story_card(ui, story_display);
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
            }
        });
    });
}

fn render_story_card(ui: &mut egui::Ui, story_display: &StoryDisplay) {
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
