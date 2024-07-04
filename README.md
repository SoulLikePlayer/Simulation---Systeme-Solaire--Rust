# Projet de Simulation de Système Solaire en Rust avec ggez

Ce projet est une simulation interactive d'un système solaire basée sur Rust, utilisant le framework de jeu ggez. Il permet de visualiser les orbites des planètes et de leurs lunes autour du soleil, ainsi que de naviguer dans le système avec des commandes clavier pour le déplacement de la caméra et le zoom.

## Fonctionnalités

    - Simulation du Système Solaire : Les planètes (Mercure, Venus, Terre, Mars, Jupiter, Saturne, Uranus, Neptune) sont représentées avec leurs propriétés orbitales telles que la distance au soleil, la vitesse orbitale et leurs lunes respectives.
    - Interactivité : Utilisation des touches directionnelles pour déplacer la caméra à travers le système solaire. Les touches Z et S permettent de zoomer et dézoomer pour explorer le système à différentes échelles.
    - Affichage Graphique : Utilisation de primitives graphiques pour dessiner les planètes, leurs orbites, les lunes, ainsi que leurs noms.
    - Modularité : Le code est structuré avec des structures (structs) pour représenter les planètes, les lunes et le système solaire, avec des méthodes pour la mise à jour de la simulation et le rendu graphique.

## Prérequis

    Rust 1.0 ou supérieur
    Cargo (le gestionnaire de paquets de Rust)

## Installation

 1) Clonez ce repository :

git clone https://github.com/votre_utilisateur/systeme-solaire-rust.git

 2) Déplacez-vous dans le répertoire du projet :

cd systeme-solaire-rust

 3) Compilez et lancez le projet avec Cargo :



    cargo run --release

## Contrôles

    - Utilisez les touches fléchées (Up, Down, Left, Right) pour déplacer la caméra.
    - Utilisez les touches Z pour zoomer et S pour dézoomer.
