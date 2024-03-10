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
//!     // Implémentation de la fonction main()
//! }
//! ```


