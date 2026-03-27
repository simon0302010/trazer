use gtk::gdk::Display;
use shumate::{MapLayer, MapSourceRegistry, prelude::*};

use shumate::{Map};
use gtk::{Application, ApplicationWindow, CssProvider, Grid, SearchBar, SearchEntry, glib};

mod trace;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("de.simon0302010.trazer")
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(setup_gui);

    app.run()
}

fn setup_gui(app: &Application) {
    let grid = Grid::builder()
        .row_spacing(8).column_spacing(8)
        .margin_top(12).margin_bottom(12).margin_end(12).margin_start(12)
        .build();

    let ip_input = SearchEntry::default();
    grid.attach(&ip_input, 0, 0, 1, 1);

    let map = create_map();
    map.set_hexpand(true); map.set_vexpand(true);
    map.add_css_class("rounded");
    grid.attach(&map, 0, 1, 1, 1);

    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(400)
        .child(&grid)
        .build();

    window.show();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect a display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );
}

fn create_map() -> Map {
    let map = Map::new();
    let viewport = map.viewport().expect("Failed to get map viewport");

    let registry = MapSourceRegistry::new();
    registry.populate_defaults();

    let source = registry.by_id(shumate::MAP_SOURCE_OSM_MAPNIK).expect("Failed to find map source");

    map.set_map_source(&source);
    map.add_layer(&MapLayer::new(&source, &viewport));

    viewport.set_zoom_level(5.0);
    viewport.set_location(52.520007, 13.404954);

    map
}
