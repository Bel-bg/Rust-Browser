#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{WebviewUrl, WebviewWindowBuilder, Manager};

#[tauri::command]
fn changer_url(handle: tauri::AppHandle, url: String) {
    if let Some(webview) = handle.get_webview_window("contenu") {
        let _ = webview.navigate(url.parse().unwrap());
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // 1. Fenêtre principale
            let window = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("Mon Navigateur Léger")
                .inner_size(1000.0, 700.0)
                .build()?;

            // 2. Barre d'outils HTML (Hauteur : 50px)
            let _barre = window.add_child(
                tauri::webview::WebviewBuilder::new("barre", WebviewUrl::App("index.html".into()))
                    .auto_resize()
            , tauri::LogicalPosition::new(0.0, 0.0), tauri::LogicalSize::new(1000.0, 50.0))?;

            // 3. Zone de contenu web (Prend le reste de l'écran)
            let _contenu = window.add_child(
                tauri::webview::WebviewBuilder::new("contenu", WebviewUrl::External("https://tauri.app".parse().unwrap()))
                    .auto_resize()
            , tauri::LogicalPosition::new(0.0, 50.0), tauri::LogicalSize::new(1000.0, 650.0))?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![changer_url])
        .run(tauri::generate_context!())
        .expect("erreur lors du lancement du navigateur");
}
