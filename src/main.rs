//! # Pool de Serveurs Actix
//!
//! Ce module implémente un pool de serveurs pour équilibrer la charge des demandes HTTP entrantes
//! entre plusieurs serveurs.
//!
//! Chaque serveur est représenté par une adresse et le nombre de connexions actives.
//!
//! ## Structures
//!
//! Structure représentant un serveur avec son adresse et le nombre de connexions actives.
//!
//! ```rust
//! struct Server {
//!     address: String,
//!     connections: usize,
//! }
//! ```
//!
//! Structure représentant un pool de serveurs.
//!
//! ```rust
//! struct ServerPool {
//!     servers: Vec<Server>,
//! }
//! ```
//!
//! ## Implémentation de `ServerPool`
//!
//! La structure `ServerPool` contient des méthodes pour gérer les serveurs dans le pool.
//!
//! ```rust
//! impl ServerPool {
//!     /// Méthode de création d'un nouveau pool de serveurs.
//!     ///
//!     /// # Arguments
//!     ///
//!     /// * `servers` - Vecteur d'adresses de serveurs à inclure dans le pool.
//!     ///
//!     /// # Exemple
//!     ///
//!     /// ```
//!     /// let servers = vec![
//!     ///     "localhost:8081".to_string(),
//!     ///     "localhost:8082".to_string(),
//!     /// ];
//!     /// let pool = ServerPool::new(servers);
//!     /// ```
//!     fn new(servers: Vec<String>) -> Self {
//!         // Implémentation de la méthode new()
//!     }
//!
//!     /// Méthode pour obtenir le prochain serveur en fonction du nombre de connexions actives.
//!     ///
//!     /// Cette méthode parcourt tous les serveurs pour trouver celui avec le moins de connexions
//!     /// et retourne une référence mutable vers ce serveur.
//!     ///
//!     /// # Exemple
//!     ///
//!     /// ```
//!     /// let mut pool = ServerPool::new(vec![
//!     ///     "localhost:8081".to_string(),
//!     ///     "localhost:8082".to_string(),
//!     /// ]);
//!     /// let server = pool.next_server();
//!     /// ```
//!     fn next_server(&mut self) -> &mut Server {
//!         // Implémentation de la méthode next_server()
//!     }
//! }
//! ```
//!
//! ## Fonction `index`
//!
//! La fonction `index` est un gestionnaire pour la route racine ("/").
//!
//! ```rust
//! /// Fonction de gestionnaire pour l'index.
//! ///
//! /// Cette fonction est appelée lorsqu'une demande HTTP est reçue sur la racine ("/").
//! ///
//! /// # Arguments
//! ///
//! /// * `req` - La demande HTTP reçue.
//! /// * `pool` - Données partagées contenant le pool de serveurs.
//! ///
//! /// # Retourne
//! ///
//! /// Un objet implémentant le trait `Responder`, généralement une réponse HTTP.
//! ///
//! /// # Exemple d'utilisation
//! ///
//! /// ```rust
//! /// async fn some_handler(req: HttpRequest, pool: web::Data<Arc<Mutex<ServerPool>>>) -> impl Responder {
//! ///     // Traitement de la demande...
//! /// }
//! /// ```
//! async fn index(req: HttpRequest, pool: web::Data<Arc<Mutex<ServerPool>>>) -> impl Responder {
//!     // Implémentation de la fonction index()
//! }
//! ```
//!
//! ## Fonction `main`
//!
//! La fonction `main` est utilisée pour lancer le serveur Actix.
//!
//! ```rust
//! /// Fonction principale pour lancer le serveur.
//! ///
//! /// Cette fonction initialise le pool de serveurs, configure et lance le serveur Actix.
//! ///
//! /// # Retourne
//! ///
//! /// Un `Result` indiquant si le serveur a été démarré avec succès ou s'il y a eu une erreur.
//! ///
//! /// # Exemple d'utilisation
//! ///
//! /// ```rust
//! /// fn main() -> std::io::Result<()> {
//! ///     // Lancement du serveur...
//! /// }
//! /// ```
//! #[actix_web::main]
//! async fn main() -> std::io::Result<()> {

use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use std::sync::{Arc, Mutex};
use reqwest::Client;

// Structure représentant un serveur avec son adresse et le nombre de connexions actives
struct Server {
    address: String,
    connections: usize,
}

// Structure représentant un pool de serveurs
struct ServerPool {
    servers: Vec<Server>,
}

impl ServerPool {
    // Méthode de création d'un nouveau pool de serveurs
    fn new(servers: Vec<String>) -> Self {
        let mut server_vec = Vec::new();
        // Initialise chaque serveur avec une adresse et aucune connexion active
        for server in servers {
            server_vec.push(Server {
                address: server,
                connections: 0,
            });
        }
        ServerPool { servers: server_vec }
    }

    // Méthode pour obtenir le prochain serveur en fonction du nombre de connexions actives
    fn next_server(&mut self) -> &mut Server {
        let mut min_connections = usize::MAX;
        let mut selected_server_index = 0;

        // Parcourt tous les serveurs pour trouver celui avec le moins de connexions
        for (index, server) in self.servers.iter().enumerate() {
            if server.connections < min_connections {
                min_connections = server.connections;
                selected_server_index = index;
            }
        }

        // Retourne une référence mutable vers le serveur sélectionné
        &mut self.servers[selected_server_index]
    }
}

// Fonction de gestionnaire pour l'index
async fn index(req: HttpRequest, pool: web::Data<Arc<Mutex<ServerPool>>>) -> impl Responder {
    let mut pool = pool.lock().unwrap();
    // Sélectionne le prochain serveur en fonction du nombre de connexions actives
    let server = pool.next_server();
    // Incrémente le nombre de connexions du serveur sélectionné
    server.connections += 1;

    let url = format!("http://{}/{}", server.address, req.uri());
    let client = Client::new();

    // Envoie la demande au serveur sélectionné
    match client.get(&url).send().await {
        Ok(response) => {
            let status = response.status();
            let body = response.text().await.unwrap_or_else(|_| String::from("Error reading response body"));
            HttpResponse::build(status).body(body)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let servers = vec![
        "localhost:8081".to_string(),
        "localhost:8082".to_string(),
    ];

    let pool = Arc::new(Mutex::new(ServerPool::new(servers)));

    // Lance le serveur web Actix
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
//!     // Implémentation de la fonction main()
//! }
//! ```


