use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    // Chemin vers le fichier de logs
    let chemin_fichier_logs = "/Users/nathanbinet/Desktop/ssl_access_log";

    // Début du comptage du temps d'exécution
    let debut = Instant::now();

    // Ouverture du fichier
    let fichier = match File::open(chemin_fichier_logs) {
        Ok(fichier) => fichier,
        Err(erreur) => panic!("Impossible d'ouvrir le fichier : {}", erreur),
    };

    // Initialisation des variables pour stocker les statistiques
    let mut total_requetes = 0;
    let mut status_codes: HashMap<u16, u32> = HashMap::new();
    let mut adresses_ip: HashMap<String, u32> = HashMap::new();

    // Parcours du fichier ligne par ligne
    for ligne in BufReader::new(fichier).lines() {
        if let Ok(ligne) = ligne {
            // Split de la ligne pour extraire les informations pertinentes
            let elements: Vec<&str> = ligne.split_whitespace().collect();
            if elements.len() >= 9 {
                let status_code: u16 = elements[8].parse().unwrap_or(0);
                let adresse_ip = elements[0].to_string();

                // Mise à jour des statistiques
                total_requetes += 1;
                *status_codes.entry(status_code).or_insert(0) += 1;
                *adresses_ip.entry(adresse_ip).or_insert(0) += 1;
            }
        }
    }

    // Affichage des statistiques
    println!("Nombre total de requêtes: {}", total_requetes);
    for (status, count) in &status_codes {
        println!("Statut HTTP {} : {}", status, count);
    }
    println!("Nombre total d'adresses IP uniques : {}", adresses_ip.len());

    // Recherche de l'adresse IP la plus fréquente
    let (ip_frequente, nombre_requetes) = adresses_ip.iter().max_by_key(|&(_, count)| count).unwrap();
    let pourcentage = (*nombre_requetes as f32 / total_requetes as f32) * 100.0;
    println!("Adresse IP la plus fréquente : {} ({} requêtes, {:.2}% du total)", ip_frequente, nombre_requetes, pourcentage);

    // Affichage du temps d'exécution
    let temps_execution = debut.elapsed().as_secs_f64();
    println!("Temps d'exécution du script: {} secondes.", temps_execution);
}