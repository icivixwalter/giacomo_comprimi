/*
    IL TUTORIAL DELLE EGUI
        in rust si trova qui:
            https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/panels.rs
*/


use eframe::{App, Frame};
use egui::{Context, Ui};

// Costante per il percorso della cartella contenente gli eseguibili
const ESEGUIBILI_PATH: &str = r"c:\CASA\CDM\LeTorri";

// Struttura per l'applicazione che gestisce lo stato
pub struct MyApp {
    // definito tutti i campi delle struttura
    checkbox_tutti: bool,
    my_checkboxes: Vec<bool>, // Stato delle checkbox (se selezionate o meno)
    // executables: Vec<String>, // Lista di eseguibili trovati nella cartella todo: sostituisce con path del file esterno
    // combo_box_selection: String, // Opzione attualmente selezionata nella ComboBox
    // output: String, // Output catturato dagli eseguibili
}

// implementato a mano il trait default
impl Default for MyApp {
    fn default() -> Self {
        Self {
            checkbox_tutti: false,          //la checkbox tutti base false
            my_checkboxes: vec![false; 9],  //tutte le altre
        }
    }
}
// App e' un trait = interfaccia = collezione di metodi non definiti
// impl trait for struct/enum
impl App for MyApp {
    //FUNZONE UPDATE DI AGGIORNAMENTO INTERFACCIA UTETENTE
    // viene chiamata automaticamente con MyApp
    // implemento le CHECK BOX
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.checkbox(&mut self.selected[0], "2008");
            // ui.checkbox(&mut self.selected[1], "2009");
            // ui.checkbox(&mut self.selected[2], "2010");


            //CREO IL BUTTON PRIMA DI TUTTO
            //________________________________________________________________//
                let btn_comprimi = ui.button("COMPRIMI");
                if btn_comprimi.clicked() {
                    //stampo il nome ed il valore della check box cliccata.
                    println!("click Button comprimi");
                }
                ui.add_space(10.0);

            //________________________________________________________________//

            //NELLA MEMORIA HEAP
            //INCOLONNA + VISUALIZZA IL VALORE IMPOSTATO IL VALORE FALSE/TRUE DELLA MEMORIA HEAP
            let checkbox_tutti = ui.checkbox(&mut self.checkbox_tutti, "Tutti");
            if checkbox_tutti.clicked() {
                println!("Checkbox Tutti clicked");
                for my_checkbox in self.my_checkboxes.iter_mut() {
                    *my_checkbox = self.checkbox_tutti;  //permette la selezione delle check box in base al valore della prima
                }
            }

            //NELLA MEMORIA HEAP
            //CICLO DA 2008 +1 PER NOVE VOLTE COME IL VETTORE
            //INCOLONNA + VISUALIZZA IL VALORE CORRENTE DELL'HEAP DELLE CHECK BOX
            //definisco my_bool il valore i-esimo del vettore
            let mut i = 0;
            for my_bool in self.my_checkboxes.iter_mut() {
                //assegno alla variabile il nome della cartella costruita +1
                let my_cartella = format!("{}", 2008 + i);

                //
                ui.horizontal(|ui| {
                    //assegno alla checkbox il valore bool + il nome costruito
                    let checkbox = ui.checkbox(my_bool, &my_cartella);
                    //evento click checkbox
                    if checkbox.clicked() {
                        //stampo il nome ed il valore della check box cliccata.
                        println!("Checkbox con indice {i} clicked, nome = {}", &my_cartella);
                    }

                    ui.label("2008 prova");
                });

                i += 1;
            }
        });
    }
}

//TODO: come prendere per l'etichetta il nome della cartella finale
//    ed assegnalo all'etichetta della check box?? si trova i cartella.rs

/*
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
*/