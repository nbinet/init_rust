use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap};
use std::time::Instant;
use std::env;
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    // Récupération des arguments
    let args: Vec<String> = env::args().collect();

    // Vérification du nombre d'arguments
    if args.len() < 2 || args.len() > 3 {
        if args.len() == 1 {
            eprintln!("Erreur : Ce programme nécessite au moins un argument.");
        } else {
            eprintln!("Erreur : Ce programme ne peut pas prendre plus de deux arguments.");
        }
        std::process::exit(1);
    }

    // Chemin vers le fichier de logs
    let chemin_fichier_logs = &args[1];

    // Nombre de threads à utiliser (par défaut 1)
    let nb_threads = if args.len() == 3 {
        match args[2].parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Erreur : Le deuxième argument doit être un nombre entier.");
                std::process::exit(1);
            }
        }
    } else {
        1
    };

    // Début du comptage du temps d'exécution
    let debut = Instant::now();

    // Ouverture du fichier
    let fichier = match File::open(chemin_fichier_logs) {
        Ok(fichier) => fichier,
        Err(erreur) => panic!("Impossible d'ouvrir le fichier : {}", erreur),
    };

    // Initialisation des variables partagées entre les threads
    let total_requetes = std::sync::Arc::new(std::sync::Mutex::new(0));
    let status_codes = std::sync::Arc::new(std::sync::Mutex::new(HashMap::new()));
    let adresses_ip: Arc<Mutex<HashMap<String, u32>>> = Arc::new(Mutex::new(HashMap::new()));

    // Création des threads
    let mut handles = vec![];
    for _ in 0..nb_threads {
        let fichier_clone = fichier.try_clone().expect("Impossible de cloner le fichier");
        let total_requetes_clone = total_requetes.clone();
        let status_codes_clone = status_codes.clone();
        let adresses_ip_clone = adresses_ip.clone();

        let handle = thread::spawn(move || {
            let reader = BufReader::new(fichier_clone);
            for ligne in reader.lines() {
                if let Ok(ligne) = ligne {
                    // Split de la ligne pour extraire les informations pertinentes
                    let elements: Vec<&str> = ligne.split_whitespace().collect();
                    if elements.len() >= 9 {
                        let status_code: u16 = elements[8].parse().unwrap_or(0);
                        let adresse_ip = elements[0].to_string();

                        // Mise à jour des statistiques
                        {
                            let mut total_requetes = total_requetes_clone.lock().unwrap();
                            *total_requetes += 1;
                        }

                        {
                            let mut status_codes = status_codes_clone.lock().unwrap();
                            *status_codes.entry(status_code).or_insert(0) += 1;
                        }

                        {
                            let mut adresses_ip = adresses_ip_clone.lock().unwrap();
                            let count = *status_codes_clone.lock().unwrap().get(&status_code).unwrap_or(&0);
                            // Accès et modification sécurisés de adresses_ip
                            adresses_ip.insert(adresse_ip.clone(), count);
                        }
                    }
                }
            }
        });
        handles.push(handle);
    }

    // Attente de la fin de tous les threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Affichage des statistiques
    let total_requetes = total_requetes.lock().unwrap();
    let status_codes = status_codes.lock().unwrap();
    let adresses_ip = adresses_ip.lock().unwrap();

    println!("Nombre total de requêtes: {}", *total_requetes);
    for (status, count) in status_codes.iter() {
        println!("Statut HTTP {} : {}", status, count);
    }
    println!("Nombre total d'adresses IP uniques : {}", adresses_ip.len());

    // Recherche de l'adresse IP la plus fréquente
    let (ip_frequente, nombre_requetes) = adresses_ip.iter().max_by_key(|&(_, count)| count).unwrap();
    let pourcentage = (*nombre_requetes as f32 / *total_requetes as f32) * 100.0;
    println!("Adresse IP la plus fréquente : {} ({} requêtes, {:.2}% du total)", ip_frequente, nombre_requetes, pourcentage);

    // Affichage du temps d'exécution
    let temps_execution = debut.elapsed().as_secs_f64();
    println!("Temps d'exécution du script: {} secondes.", temps_execution);
}
