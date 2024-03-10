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