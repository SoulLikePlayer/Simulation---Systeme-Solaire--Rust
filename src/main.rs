// src/main.rs

struct Planete {
    nom: String,
    distance: f64,
    vitesse: f64,
}

struct SystemeSolaire {
    planetes: Vec<Planete>,
}

impl SystemeSolaire {
    fn new() -> SystemeSolaire {
        SystemeSolaire {
            planetes: vec![
                Planete {
                    nom: "Mercure".to_string(),
                    distance: 57.9,
                    vitesse: 47.4,
                },
                Planete {
                    nom: "Venus".to_string(),
                    distance: 108.2,
                    vitesse: 35.0,
                },
                Planete {
                    nom: "Terre".to_string(),
                    distance: 149.6,
                    vitesse: 29.8,
                },
                Planete {
                    nom: "Mars".to_string(),
                    distance: 227.9,
                    vitesse: 24.1,
                },
            ],
        }
    }

    fn afficher(&self) {
        for planete in &self.planetes {
            println!(
                "Planète: {}, Distance: {} millions de km, Vitesse: {} km/s",
                planete.nom, planete.distance, planete.vitesse
            );
        }
    }
}

fn main() {
    let systeme_solaire = SystemeSolaire::new();
    systeme_solaire.afficher();
}
