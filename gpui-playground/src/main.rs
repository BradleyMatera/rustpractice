mod assets;
mod backend;
mod ui;

use gpui::{App, AppContext, WindowOptions};
use gpui_platform::application;
use std::{path::PathBuf, sync::Arc};

use assets::FileAssetSource;
use backend::ChannelBackend;
use ui::CrabCordShell;

fn main() {
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
    let (backend_tx, backend_rx) = std::sync::mpsc::channel();
    let backend = Arc::new(ChannelBackend::new(backend_tx));

    #[cfg(feature = "axiom-backend")]
    std::thread::spawn(move || {
        while let Ok(command) = backend_rx.recv() {
            let _ = backend::to_axiom_message(command);
        }
    });

    #[cfg(not(feature = "axiom-backend"))]
    std::thread::spawn(move || while backend_rx.recv().is_ok() {});

    application()
        .with_assets(FileAssetSource::new(assets_dir))
        .run(move |cx: &mut App| {
            let backend = Arc::clone(&backend);
            let _ = cx.open_window(WindowOptions::default(), move |_, cx| {
                let backend = Arc::clone(&backend);
                cx.new(move |_| CrabCordShell::new_with_backend(backend))
            });
        });
}
