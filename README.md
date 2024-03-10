# LoadBalancer

Ce projet est un LoadBalancer simple développé avec Actix Web en Rust. Il permet de rediriger les requêtes vers plusieurs serveurs backend.

## Fonctionnalités

- Routage des requêtes vers plusieurs serveurs backend
- Gestion de l'équilibrage de charge basique
- Utilisation de reqwest pour transférer les requêtes

## Installation

Assurez-vous d'avoir Rust et Cargo installés sur votre système.

1. Clonez ce dépôt :

git clone [https://github.com/votre-utilisateur/actix-web-proxy.git](https://github.com/Retr0Lovelace/LoadBalancer-ESGI)

2. Accédez au répertoire du projet :

cd actix-web-proxy

3. Exécutez l'application :

cargo run


L'application sera exécutée sur `http://127.0.0.1:8080`.

## Configuration

Vous pouvez modifier la liste des serveurs backend dans la fonction `main` du fichier `src/main.rs`.

## Contributions

Les contributions sont les bienvenues ! N'hésitez pas à ouvrir une issue ou à proposer une pull request pour améliorer ce projet.

## Auteurs

- Benjamin FRANCISCO | 4SI3
- Thibaut GUESDON | 4SI3
