use std::sync::Mutex;
use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder},
    webview::PageLoadEvent,
    AppHandle, Builder, Listener, Manager, WindowEvent,
};
use tauri_plugin_notification::NotificationExt;

struct AppTray(Mutex<TrayIcon>);

fn focus_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let unread_icon = Some(Image::from_bytes(include_bytes!("../icons/unread.png")).unwrap());

    Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            focus_main_window(app);
        }))
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();
            let open_i = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&open_i, &quit_i])?;
            let tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(true)
                .build(app)?;

            app.manage(AppTray(Mutex::new(tray.clone())));
            app.listen("notif", move |event| {
                tray.set_icon(unread_icon.clone()).unwrap();

                if let Ok(payload) = serde_json::from_str::<serde_json::Value>(event.payload()) {
                    let mut builder = handle
                        .notification()
                        .builder()
                        .title(payload["title"].as_str().unwrap())
                        .sound("message-new-instant");

                    if let Some(body) = payload["body"].as_str() {
                        builder = builder.body(body);
                    }

                    if let Some(icon) = payload["icon"].as_str() {
                        builder = builder.icon(icon);
                    }

                    let _ = builder.show();
                }
            });

            Ok(())
        })
        .on_page_load(|window, payload| {
            if matches!(payload.event(), PageLoadEvent::Started { .. }) && window.label() == "main"
            {
                let _ = window.eval(
                    r#"
                    window.Notification = function (title, options) {
                        window.__TAURI__.event.emit('notif', { title, ...options });
                        return {
                            close: () => {},
                            onclick: null,
                            onshow: null,
                            onerror: null,
                            onclose: null,
                            title,
                            ...options,
                            permission: 'granted',
                        };
                    };
                    window.Notification.permission = 'granted';
                    window.Notification.requestPermission = () => Promise.resolve('granted');
                    "#,
                );
            }
        })
        .on_menu_event(|app, event| match event.id.as_ref() {
            "open" => {
                focus_main_window(app);
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }

            if let WindowEvent::Focused(true) = event {
                let app = window.app_handle();

                if let Some(tray_state) = app.try_state::<AppTray>() {
                    let tray = tray_state.0.lock().unwrap();
                    let _ = tray.set_icon(Some(app.default_window_icon().unwrap().clone()));
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("whoopsie");
}
