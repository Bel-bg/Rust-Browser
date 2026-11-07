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
            // Notre code HTML directement en chaîne de caractères Rust !
            let html_barre = r#"
            <!DOCTYPE html>
            <html lang="fr">
            <head>
              <meta charset="UTF-8">
              <style>
                body { margin: 0; background: #222; font-family: sans-serif; display: flex; padding: 5px; gap: 5px; overflow: hidden; }
                input { flex: 1; padding: 8px; border: 1px solid #444; background: #333; color: white; border-radius: 4px; font-size: 14px; }
                button { padding: 8px 16px; background: #0f766e; color: white; border: none; border-radius: 4px; cursor: pointer; font-weight: bold; }
                button:hover { background: #115e59; }
              </style>
            </head>
            <body>
              <input type="text" id="url-input" value="https://tauri.app" placeholder="Entrez une URL..." />
              <button id="go-btn">Aller</button>
              <script>
                const { invoke } = window.__TAURI__.core;
                const input = document.getElementById('url-input');
                const button = document.getElementById('go-btn');
                function naviguer() {
                  let url = input.value.trim();
                  if (!url.startsWith('http://') && !url.startsWith('https://')) {
                    url = 'https://' + url;
                  }
                  invoke('changer_url', { url: url });
                }
                button.addEventListener('click', naviguer);
                input.addEventListener('keypress', (e) => { if (e.key === 'Enter') naviguer(); });
              </script>
            </body>
            </html>
            "#;

            // 1. Fenêtre principale
            let window = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("Mon Navigateur Léger")
                .inner_size(1000.0, 700.0)
                .build()?;

            // 2. Barre d'outils (On lui passe directement le texte HTML)
            let _barre = window.add_child(
                tauri::webview::WebviewBuilder::new("barre", WebviewUrl::Custom(html_barre.to_string()))
                    .auto_resize()
            , tauri::LogicalPosition::new(0.0, 0.0), tauri::LogicalSize::new(1000.0, 50.0))?;

            // 3. Zone de contenu web
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
