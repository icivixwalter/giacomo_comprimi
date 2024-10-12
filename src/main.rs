/*
TOML
queste dipendenze
[package]
name = "simple_egui_app"
version = "0.1.0"
edition = "2021"

[dependencies]
eframe = "0.16.0"  # Mantieni questa versione
egui = "0.16.0"    # Assicurati che la versione di egui sia la stessa di eframe



Descrizione del Codice
Il codice implementa un'applicazione grafica per gestire l'esecuzione di file eseguibili (.exe) presenti in una cartella specificata. L'applicazione utilizza la libreria eframe e egui per l'interfaccia utente.

Modifiche Apportate
Rimozione della Casella di Testo per l'Input:

La casella di testo che permetteva di inserire del testo è stata rimossa. Questo significa che ora l'applicazione non ha più un'area dove l'utente può digitare un testo.
Mantenimento della Casella di Testo per l'Output:

È rimasta la casella di testo che mostra l'output degli eseguibili quando vengono eseguiti. Questo è utile per vedere il risultato delle operazioni eseguite dagli eseguibili selezionati.
Regolazione dell'Interfaccia Utente:

L'interfaccia utente è stata semplificata. Ora ci sono solo le checkbox per selezionare gli eseguibili, un ComboBox per selezionare un'opzione e la casella di testo che mostra l'output.
Funzionalità del Codice
Caricamento degli Eseguibili:

All'avvio dell'app, il codice legge tutti i file .exe presenti nella cartella specificata e li memorizza in un vettore (executables). Le checkbox corrispondenti a ciascun file eseguibile vengono create dinamicamente.
Selezione e Esecuzione degli Eseguibili:

L'utente può selezionare uno o più eseguibili tramite le checkbox e può eseguirli cliccando sui pulsanti "Esegui selezionati" o "Esegui Tutti". L'output di ciascun eseguibile viene catturato e visualizzato nella casella di output.
ComboBox:

C'è una casella combinata (ComboBox) che permette di selezionare diverse opzioni. Anche se non ha un impatto diretto sull'esecuzione degli eseguibili nel codice attuale, può essere utilizzata per future espansioni.
Uscita dall'Applicazione:

L'utente può uscire dall'applicazione cliccando sul pulsante "Esci".


scrive sulla casella di testo.


l codice fornito implementa un'applicazione GUI in Rust utilizzando le librerie eframe e egui. Questa applicazione è progettata per gestire l'esecuzione di file eseguibili (.exe) in una cartella specificata. Di seguito, troverai una spiegazione dettagliata delle sue funzionalità, insieme a una versione corretta del codice, se necessario.

Funzionalità dell'Applicazione
Caricamento degli Eseguibili:

All'avvio, l'app legge tutti i file .exe nella cartella specificata (in questo caso c:\CASA\CDM\LeTorri) e li memorizza in un vettore chiamato executables.
Interfaccia Utente:

La UI è composta da:
Checkbox: Permette all'utente di selezionare uno o più eseguibili.
Pulsanti: Include "Esegui selezionati", "Esegui Tutti" e "Esci".
ComboBox: Permette di selezionare un'opzione (anche se al momento non influisce sull'esecuzione degli eseguibili).
Casella di Output: Mostra l'output degli eseguibili quando vengono eseguiti.
Esecuzione degli Eseguibili:

L'utente può eseguire gli eseguibili selezionati o tutti gli eseguibili presenti nella lista.
L'output generato da ogni eseguibile viene catturato e visualizzato nella casella di output.

*/




/*

    MODIFICHE:

    @10_SCROLLAREA_AGGIUNTA         =  Aggiungi una ScrollArea per la casella di output
    @11_ATTIVATA_INTESTAZIONE        = Aggiungi intestazione di inizio e fine
*/



// Importa le librerie necessarie da eframe e std

mod cartelle;

use eframe::{egui, epi}; // eframe è una libreria per GUI in Rust
use std::process::{Command, Stdio}; // Per eseguire comandi e gestire i processi
use std::path::Path; // Per lavorare con i percorsi dei file
use std::fs; // Per interagire con il filesystem
use std::io::{self, BufRead}; // Per la lettura dell'output dei processi
use egui::CtxRef; // Contesto di rendering di egui

// Costante per il percorso della cartella contenente gli eseguibili
const ESEGUIBILI_PATH: &str = r"c:\CASA\CDM\LeTorri";

#[derive(Default)]
// Struttura per l'applicazione che gestisce lo stato
struct MyApp {
    selected: Vec<bool>, // Stato delle checkbox (se selezionate o meno)
    executables: Vec<String>, // Lista di eseguibili trovati nella cartella
    combo_box_selection: String, // Opzione attualmente selezionata nella ComboBox
    output: String, // Output catturato dagli eseguibili
}

impl epi::App for MyApp {
    // Nome dell'applicazione
    fn name(&self) -> &str {
        "Gestione Eseguibili"
    }

    // Funzione di aggiornamento dell'interfaccia utente
    fn update(&mut self, ctx: &CtxRef, _frame: &epi::Frame) {
        // Carica gli eseguibili dalla cartella solo se la lista è vuota
        if self.executables.is_empty() {
            self.load_executables(); // Chiama la funzione per caricare gli eseguibili
            //TUTTE LE CHECK BOX = FALSE
            self.selected = vec![false; self.executables.len()]; // Inizializza lo stato delle checkbox
        }

        egui::SidePanel::right("prova").show(ctx, |ui| {
            // Area laterale a destra con la casella combinata e l'output
            ui.vertical(|ui| {
                ui.label("Seleziona un'opzione:"); // Etichetta per la ComboBox

                // Opzioni per la ComboBox, inclusa l'opzione per svuotare l'output
                let options = ["Opzione 1", "Opzione 2", "Opzione 3", "Svuota Output", "Opzione 4"];

                // Imposta la selezione iniziale se non è già impostata
                if self.combo_box_selection.is_empty() {
                    self.combo_box_selection = options[0].to_string(); // Seleziona la prima opzione
                }

                // Visualizza la casella combinata
                egui::ComboBox::from_label("Opzioni")
                    .selected_text(&self.combo_box_selection) // Mostra l'opzione selezionata
                    .show_ui(ui, |ui| {
                        // Aggiunge le opzioni alla ComboBox
                        for &option in options.iter() {
                            ui.selectable_value(&mut self.combo_box_selection, option.to_string(), option); // Aggiungi l'opzione
                        }
                    });

                // Svuota l'output se l'utente ha selezionato l'opzione "Svuota Output"
                if self.combo_box_selection == "Svuota Output" {
                    self.output.clear(); // Svuota la casella di output
                    self.combo_box_selection = options[0].to_string(); // Resetta la selezione della ComboBox
                }

                // Mostra l'output degli eseguibili
                ui.label("Output degli eseguibili:"); // Etichetta per l'output

                //@10_SCROLLAREA_AGGIUNTA =  Aggiungi una ScrollArea per la casella di output
                egui::ScrollArea::vertical() // Inizia un'area di scorrimento verticale
                    .max_height(300.0) // Imposta un'altezza massima per l'area di scorrimento
                    .show(ui, |ui| {
                        ui.text_edit_multiline(&mut self.output); // Mostra l'output in una casella di testo multilinea
                    });
            });
        });


        // Costruisce l'interfaccia utente
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Pannello sinistro per gli eseguibili
                ui.vertical(|ui| {
                    ui.label("Seleziona gli eseguibili da avviare:"); // Etichetta per la selezione degli eseguibili

                    // Mostra una lista di checkbox per ogni eseguibile
                    for (i, executable) in self.executables.iter().enumerate() {
                        ui.checkbox(&mut self.selected[i], executable); // Checkbox per ciascun eseguibile
                    }

                    // Pulsante per eseguire gli eseguibili selezionati
                    if ui.button("Esegui selezionati").clicked() {

                        self.run_selected(); // Chiama la funzione per eseguire gli eseguibili selezionati


                    }

                    // Pulsante per eseguire tutti gli eseguibili
                    if ui.button("Esegui Tutti").clicked() {

                        self.run_all(); // Chiama la funzione per eseguire tutti gli eseguibili

                    }

                    // Pulsante per uscire dall'applicazione
                    if ui.button("Esci").clicked() {
                        std::process::exit(0); // Termina l'applicazione
                    }
                });
            });
        });
    }
}

impl MyApp {
    //SALVA I PATH DEGLI ESEGUIBILI E LI METTE NEL VETTORE
    // Funzione per caricare gli eseguibili dalla cartella specificata
    fn load_executables(&mut self) {
        let path = Path::new(ESEGUIBILI_PATH); // Crea un Path per la cartella degli eseguibili
        if let Ok(entries) = fs::read_dir(path) { // Legge le directory nella cartella
            for entry in entries.filter_map(Result::ok) { // Filtra le voci valide
                if let Some(filename) = entry.file_name().to_str() { // Ottiene il nome del file
                    if filename.ends_with(".exe") { // Controlla se il file è un eseguibile
                        self.executables.push(filename.to_string()); // Aggiunge l'eseguibile alla lista
                    }
                }
            }
        }
    }

    // Funzione per eseguire gli eseguibili selezionati
    fn run_selected(&mut self) {
        self.output.clear(); // Pulisce l'output prima di eseguire

        // Crea un vettore per gli eseguibili selezionati
        let selected_executables: Vec<String> = self.selected.iter()
            .enumerate()
            .filter_map(|(i, &is_selected)| if is_selected { Some(self.executables[i].clone()) } else { None })
            .collect();

        // Esegue gli eseguibili selezionati
        for executable in selected_executables {
            self.run_executable(&executable); // Chiama la funzione per eseguire l'eseguibile
        }
    }

    // Funzione per eseguire tutti gli eseguibili
    fn run_all(&mut self) {
        self.output.clear(); // Pulisce l'output prima di eseguire

        // Clona la lista di eseguibili e li esegue tutti
        for executable in self.executables.clone() {
            self.run_executable(&executable); // Chiama la funzione per eseguire l'eseguibile
        }
    }

    // Funzione per eseguire un singolo eseguibile
    fn run_executable(&mut self, executable: &str) {
        let path = format!("{}\\{}", ESEGUIBILI_PATH, executable); // Crea il percorso completo per l'eseguibile
        if Path::new(&path).exists() { // Controlla se il file esiste
            println!("Eseguendo: {}", executable); // Stampa il nome dell'eseguibile in esecuzione

            // @11_ATTIVATA_INTESTAZIONE        = Aggiungi intestazione di inizio e fine
            self.output.push_str(&format!("\n            ATTIVATA  {}\n", executable)); // Intestazione di attivazione
            self.output.push_str("//-------------------------------------------------------------//\n"); // Linea di separazione

            let process = Command::new(&path) // Crea un nuovo processo per l'eseguibile
                .stdout(Stdio::piped()) // Cattura l'output standard
                .spawn(); // Avvia il processo

            match process { // Gestisce l'esito del tentativo di avvio del processo
                Ok(mut child) => { // Se il processo è stato avviato correttamente
                    if let Some(stdout) = child.stdout.take() { // Prende lo stdout del processo
                        let reader = io::BufReader::new(stdout); // Crea un lettore per l'output
                        // Legge l'output riga per riga
                        for line in reader.lines() {
                            if let Ok(output_line) = line { // Controlla se la lettura della riga è andata a buon fine
                                self.output.push_str(&output_line); // Aggiunge la riga all'output
                                self.output.push('\n'); // Aggiunge una nuova riga
                            }
                        }
                    }
                    child.wait().expect("Errore durante l'esecuzione del processo"); // Aspetta la fine dell'esecuzione
                }
                Err(e) => { // Gestisce l'errore in caso di problemi nell'avvio del processo
                    println!("Errore nell'avviare {}: {}", executable, e); // Stampa un messaggio di errore
                }
            }

            // Aggiungi intestazione di fine
            self.output.push_str("\n//-------------------------------------------------------------//\n"); // Linea di separazione
            self.output.push_str("//                 *** fine ***\n"); // Intestazione di fine
            self.output.push_str("//-------------------------------------------------------------//\n\n\n"); // Linea di separazione
        } else {
            println!("File non trovato: {}", path); // Stampa un messaggio se il file non esiste
        }
    }
}

// Funzione principale che avvia l'applicazione
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = MyApp::default(); // Crea un'istanza predefinita dell'app
    eframe::run_native(Box::<dyn epi::App>::from(Box::new(app)), eframe::NativeOptions::default()); // Avvia l'applicazione
}
