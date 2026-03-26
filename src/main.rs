use shumate::{MapLayer, MapSourceRegistry, prelude::*};

use shumate::{Map};
use gtk::{Application, ApplicationWindow, glib};

mod trace;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("de.simon0302010.trazer")
        .build();

    app.connect_activate(setup_gui);

    app.run()
}

fn setup_gui(app: &Application) {
    let map = create_map();

    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(400)
        .child(&map)
        .build();

    window.show();
}

fn create_map() -> Map {
    let map = Map::new();
    let viewport = map.viewport().expect("Failed to get map viewport");

    let registry = MapSourceRegistry::new();
    registry.populate_defaults();

    let source = registry.by_id(shumate::MAP_SOURCE_OSM_MAPNIK).expect("Failed to find map source");

    map.set_map_source(&source);
    map.add_layer(&MapLayer::new(&source, &viewport));

    viewport.set_zoom_level(3.0);

    map
}