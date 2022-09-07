use crate::{colors::ColorOverrides, config};
use futures::{channel::mpsc::channel, SinkExt, StreamExt};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

enum Event {
    UpdateColors,
}

// #[cfg(feature="gtk4")]
// // load selected cosmic-theme
// pub fn load_gtk() -> anyhow::Result<()> {
//     adw::gtk::init()?;
//     adw::init();
//     let config_dir_path = config::Config::init()?;
//     let color_dir_path = ColorOverrides::init()?;

//     let config = config::Config::load()?;
//     config.apply()?;

//     let main_context = MainContext::default();
//     let (tx, rx) = MainContext::channel(Priority::default());
//     let tx_clone = tx.clone();
//     main_context.spawn_local(async move {
//         let style_manager = StyleManager::default();
//         style_manager.connect_color_scheme_notify(move |_| {
//             let _ = tx_clone.send(Event::UpdateColors);
//         });
//     });

//     let tx_clone = tx.clone();
//     main_context.spawn_local(async move {
//         let (mut tx, mut rx) = channel(1);

//         // Automatically select the best implementation for your platform.
//         // You can also access each implementation directly e.g. INotifyWatcher.
//         let mut watcher = RecommendedWatcher::new(
//             move |res| {
//                 futures::executor::block_on(async {
//                     tx.send(res).await.unwrap();
//                 })
//             },
//             notify::Config::default(),
//         )
//         .unwrap();
//         let _ = watcher
//             .watch(&config_dir_path, RecursiveMode::Recursive)
//             .unwrap();
//         let _ = watcher.watch(&color_dir_path.as_ref(), RecursiveMode::Recursive);

//         while let Some(res) = rx.next().await {
//             match res {
//                 Ok(e) => match e.kind {
//                     // TODO only notify for changed data file if it is the active file
//                     notify::EventKind::Create(_) | notify::EventKind::Modify(_) => {
//                         let _ = tx_clone.send(Event::UpdateColors);
//                     }
//                     _ => {}
//                 },
//                 Err(e) => eprintln!("watch error: {:?}", e),
//             }
//         }
//     });

//     rx.attach(Some(&main_context), move |_| {
//         if let Ok(config) = config::Config::load() {
//             let _ = config.apply();
//         }
//         adw::prelude::Continue(true)
//     });
//     let main_loop = MainLoop::new(Some(&main_context), true);
//     main_loop.run();
//     Ok(())
// }

#[cfg(feature = "iced")]
pub fn load_iced() -> anyhow::Result<()> {
    todo!()
}
