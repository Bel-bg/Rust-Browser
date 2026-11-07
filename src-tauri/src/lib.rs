use tauri::webview::PageLoadEvent;

// Barre d'adresse injectee dans chaque page chargee.
// Sur Android, Tauri ne gere qu'une seule webview par app: on ne peut donc pas
// avoir une webview "barre" + une webview "contenu". On injecte la barre dans
// la page elle-meme a chaque fin de chargement.
const TOOLBAR_JS: &str = include_str!("toolbar.js");

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .on_page_load(|webview, payload| {
            if let PageLoadEvent::Finished = payload.event() {
                let _ = webview.eval(TOOLBAR_JS);
            }
        })
        .run(tauri::generate_context!())
        .expect("erreur lors du lancement du navigateur");
}
