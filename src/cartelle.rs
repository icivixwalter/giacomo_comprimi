use std::process::Command;
use std::path::Path;
use std::io::{self, BufRead};
use std::fs::File;

fn main3() {
    // Definire la path base
    let base_dir = "c:\\CASA\\CDM\\LeTorri";

    // Lista delle cartelle da comprimere e i file .txt associati
    let folders_and_files = vec![
        ("c:\\CASA\\CDM\\LeTorri\\2008\\", "c:\\CASA\\CDM\\LeTorri\\Zip_N70_ElencoFile_LeTorri2008.txt"),
        ("c:\\CASA\\CDM\\LeTorri\\2009\\", "c:\\CASA\\CDM\\LeTorri\\Zip_N70_ElencoFile_LeTorri2009.txt"),
        ("c:\\CASA\\CDM\\LeTorri\\2010\\", "c:\\CASA\\CDM\\LeTorri\\Zip_N70_ElencoFile_LeTorri2010.txt"),
        ("c:\\CASA\\CDM\\LeTorri\\2011\\", "c:\\CASA\\CDM\\LeTorri\\Zip_N70_ElencoFile_LeTorri2011.txt"),
        ("c:\\CASA\\CDM\\LeTorri\\2012\\", "c:\\CASA\\CDM\\LeTorri\\Zip_N70_ElencoFile_LeTorri2012.txt"),
        ("c:\\CASA\\CDM\\LeTorri\\2013\\", "c:\\CASA\\CDM\\LeTorri\\Zip_N70_ElencoFile_LeTorri2013.txt"),
        ("c:\\CASA\\CDM\\LeTorri\\2014\\", "c:\\CASA\\CDM\\LeTorri\\Zip_N70_ElencoFile_LeTorri2014.txt"),
        ("c:\\CASA\\CDM\\LeTorri\\2015\\", "c:\\CASA\\CDM\\LeTorri\\Zip_N70_ElencoFile_LeTorri2015.txt"),
        ("c:\\CASA\\CDM\\LeTorri\\2017\\", "c:\\CASA\\CDM\\LeTorri\\Zip_N70_ElencoFile_LeTorri2017.txt"),
        ("c:\\CASA\\CDM\\LeTorri\\2018\\", "c:\\CASA\\CDM\\LeTorri\\Zip_N70_ElencoFile_LeTorri2018.txt"),
        ("c:\\CASA\\CDM\\LeTorri\\2020\\", "c:\\CASA\\CDM\\LeTorri\\Zip_N70_ElencoFile_LeTorri2020.txt"),
        ("c:\\CASA\\CDM\\LeTorri\\2021\\", "c:\\CASA\\CDM\\LeTorri\\Zip_N70_ElencoFile_LeTorri2021.txt"),
        ("c:\\CASA\\CDM\\LeTorri\\2022\\", "c:\\CASA\\CDM\\LeTorri\\Zip_N70_ElencoFile_LeTorri2022.txt"),
        ("c:\\CASA\\CDM\\LeTorri\\2023\\", "c:\\CASA\\CDM\\LeTorri\\Zip_N70_ElencoFile_LeTorri2023.txt"),
        ("c:\\CASA\\CDM\\LeTorri\\2024\\", "c:\\CASA\\CDM\\LeTorri\\Zip_N70_ElencoFile_LeTorri2024.txt"),
        ("c:\\CASA\\CDM\\LeTorri\\CONTRATTI_BOX\\", "c:\\CASA\\CDM\\LeTorri\\Zip_N70_ElencoFile_LeTorri_CONTRATTI_BOX.txt"),
        ("c:\\CASA\\CDM\\LeTorri\\CONTRATTO_DI_ACQUISTO\\", "c:\\CASA\\CDM\\LeTorri\\Zip_N70_ElencoFile_LeTorri_CONTRATTO_DI_ACQUISTO.txt"),
        ("c:\\CASA\\CDM\\LeTorri\\IN_LAVORAZIONE\\", "c:\\CASA\\CDM\\LeTorri\\Zip_N70_ElencoFile_LeTorri_IN_LAVORAZIONE.txt"),
        ("c:\\CASA\\CDM\\LeTorri\\REGOLAMENTO\\", "c:\\CASA\\CDM\\LeTorri\\Zip_N70_ElencoFile_LeTorri_REGOLAMENTO.txt"),
        ("c:\\CASA\\CDM\\LeTorri\\RIPARTO_CICCOTTI\\", "c:\\CASA\\CDM\\LeTorri\\Zip_N70_ElencoFile_LeTorri_RIPARTO_CICCOTTI.txt"),
        ("c:\\CASA\\CDM\\LeTorri\\RUST\\", "c:\\CASA\\CDM\\LeTorri\\Zip_N70_ElencoFile_LeTorri_RUST.txt"),
        ("c:\\CASA\\CDM\\LeTorri\\STUDIO_MILLESIMI_LE_TORRI\\", "c:\\CASA\\CDM\\LeTorri\\Zip_N70_ElencoFile_LeTorri_STUDIO_MILLESIMI_LE_TORRI.txt"),
    ];

    // Comprimere ogni cartella specificata
    for (folder, file_list) in &folders_and_files {
        compress_folder(folder, file_list, base_dir);
    }

    // Comprimere la cartella principale in "LeTorri_archivio.zip"
    let main_folder = "c:\\CASA\\CDM\\LeTorri\\";
    let main_file_list = "c:\\CASA\\CDM\\LeTorri\\Zip_N70_ElencoFile_LeTorri_archivio.txt";
    compress_main_folder(main_folder, main_file_list, base_dir);
}

// Funzione per comprimere una cartella specifica utilizzando il file di elenco corrispondente
fn compress_folder(folder: &str, file_list_path: &str, base_dir: &str) {
    let folder_name = Path::new(folder).file_name().unwrap().to_str().unwrap();
    let output_zip = format!("{}\\00_SALVATAGGI\\{}.zip", base_dir, folder_name);

    if !Path::new(file_list_path).exists() {
        eprintln!("File di elenco non trovato: {}", file_list_path);
        return;
    }

    let files = match read_file_list(file_list_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Errore nella lettura del file di elenco: {}", e);
            return;
        }
    };

    if let Err(e) = execute_7zip(&output_zip, &files) {
        eprintln!("Errore nella creazione dell'archivio: {}", e);
    } else {
        println!("Archivio creato correttamente: {}", output_zip);
    }
}

// Funzione per comprimere la cartella principale "LeTorri" in un unico archivio
fn compress_main_folder(_main_folder: &str, file_list_path: &str, base_dir: &str) {
    let output_zip = format!("{}\\00_SALVATAGGI\\LeTorri_archivio.zip", base_dir);

    if !Path::new(file_list_path).exists() {
        eprintln!("File di elenco non trovato: {}", file_list_path);
        return;
    }

    let files = match read_file_list(file_list_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Errore nella lettura del file di elenco: {}", e);
            return;
        }
    };

    if let Err(e) = execute_7zip(&output_zip, &files) {
        eprintln!("Errore nella creazione dell'archivio principale: {}", e);
    } else {
        println!("Archivio principale creato correttamente: {}", output_zip);
    }
}

// Funzione per leggere la lista dei file da un file .txt
fn read_file_list(file_list_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_list_path)?;
    let reader = io::BufReader::new(file);
    let mut files = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if !line.trim().is_empty() {
            files.push(line);
        }
    }
    Ok(files)
}

// Funzione per eseguire 7-Zip con la lista di file
fn execute_7zip(output_zip: &str, files: &[String]) -> io::Result<()> {
    let status = Command::new(r"C:\Program Files\7-Zip\7z.exe")
        .arg("u") // Aggiungere o creare l'archivio
        .arg("-tzip") // Specificare il formato ZIP
        .arg("-r") // Ricorsivo
        .arg(output_zip) // Output ZIP file
        .args(files) // Lista dei file da aggiungere
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Errore durante l'esecuzione di 7-Zip"))
    }
}
