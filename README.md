# Analyseur de logs en Rust vs PHP

Cet outil est conçu pour analyser un fichier de logs et fournir des statistiques sur les requêtes HTTP, les adresses IP en plus de partager le temps d'exécution du script.

## Utilisation

1. Assurez-vous d'avoir Rust installé sur votre système.

2. Clonez ce dépôt sur votre machine :

    ```bash
    git clone https://github.com/nbinet/init_rust.git
    ```

3. Accédez au répertoire du projet :

    ```bash
    cd init_rust
    ```

4. Exécutez le programme en spécifiant le chemin vers votre fichier de logs :

    ```bash
    cargo run /chemin/vers/votre/fichier.log
    ```

## Exemple de rendu :

    ```bash
    Nombre total de requêtes: 35094647
    Statut HTTP 400 : 2410
    Statut HTTP 166 : 8
    Statut HTTP 301 : 2230
    Statut HTTP 150 : 2
    Statut HTTP 304 : 77295
    Statut HTTP 405 : 1076
    Statut HTTP 503 : 17
    Statut HTTP 206 : 2657
    Statut HTTP 500 : 83306
    Statut HTTP 401 : 1381
    Statut HTTP 403 : 95140
    Statut HTTP 46876 : 1
    Statut HTTP 504 : 131
    Statut HTTP 204 : 9
    Statut HTTP 404 : 494339
    Statut HTTP 0 : 155
    Statut HTTP 302 : 1180893
    Statut HTTP 499 : 18802
    Statut HTTP 200 : 33124246
    Statut HTTP 502 : 10549
    Nombre total d'adresses IP uniques : 13701
    Adresse IP la plus fréquente : 184.30.41.150 (33124246 requêtes, 94.39% du total)
    Temps d'exécution du script: 288.313851291 secondes.
    ```
