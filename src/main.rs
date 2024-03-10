extern crate actix_web; // Ajoutez cette ligne pour résoudre l'erreur d'importation pour actix_web

use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use std::sync::{Arc, Mutex};
use reqwest::Client; // Ajoutez cette importation pour reqwest

struct ServerPool {
    servers: Vec<String>,
    current_index: usize,
}

impl ServerPool {
    fn new(servers: Vec<String>) -> Self {
        ServerPool {
            servers,
            current_index: 0,
        }
    }

    // Méthode pour obtenir le prochain serveur de la piscine
    fn next_server(&mut self) -> String {
        let server = self.servers[self.current_index].clone();
        self.current_index = (self.current_index + 1) % self.servers.len();
        server
    }
}

// Fonction de gestionnaire pour l'index
async fn index(req: HttpRequest, pool: web::Data<Arc<Mutex<ServerPool>>>) -> impl Responder {
    let mut pool = pool.lock().unwrap();
    let server = pool.next_server();
    let url = format!("http://{}/{}", server, req.uri());

    // Créez une instance de Client reqwest
    let client = Client::new();

    // Transférer la requête au serveur sélectionné
    match client.get(&url).send().await {
        Ok(response) => {
            let status = response.status();
            let body = response.text().await.unwrap_or_else(|_| String::from("Erreur lors de la lecture du corps de la réponse"));
            HttpResponse::build(status).body(body)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Définissez vos serveurs backend
    let servers = vec![
        "localhost:8081".to_string(),
        "localhost:8082".to_string()
    ];

    // Créez la piscine de serveurs
    let pool = Arc::new(Mutex::new(ServerPool::new(servers)));

    // Lancez le serveur web Actix
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}