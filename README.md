
# LA PROCEDURA EGUI FUNZIONANTE
@GIACOMO@COMPRIMI@CARTELLE
Importa le librerie necessarie da eframe e std

```rust
mod cartelle;
mod gui;

use crate::gui::MyApp;
use eframe::egui;
use eframe::App;
// Per interagire con il filesystem
use std::io::BufRead;
// Per la lettura dell'output dei processi

// Funzione principale che avvia l'applicazione
fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        //@finestra@base@windosw_(crea la finestra base di larghezza ed altezza predefiniti)
        viewport: egui::ViewportBuilder::default().with_inner_size([620.0, 320.0]), //larghezza+ altezza
        ..Default::default()
    };
    // attiva il ciclo di di rendering ed agni iterazione del ciclo chiama il metodo App::update()
    // nel quale è presente il codice della grafica.
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| { // creo MyApp sull'heap grazie a Box(smart pointer)
            println!("Ho creato la MyApp con un puntatore sull'heap che puo essere modificato");
            return Ok(Box::<MyApp>::default()); // crea + restituisce il puntatore alla Myapp sull'heap
        }),
    )
}
```


