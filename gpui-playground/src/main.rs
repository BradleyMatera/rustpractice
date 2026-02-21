mod assets;
mod ui;

use gpui::{App, AppContext, WindowOptions};
use gpui_platform::application;
use std::path::PathBuf;

use assets::FileAssetSource;
use ui::CrabCordShell;

fn main() {
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    application()
        .with_assets(FileAssetSource::new(assets_dir))
        .run(|cx: &mut App| {
            let _ = cx.open_window(WindowOptions::default(), |_, cx| {
                cx.new(|_| CrabCordShell::new())
            });
        });
}
