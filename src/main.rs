// src/main.rs

use std::f64::consts::PI;

struct Planete {
    nom: String,
    distance: f64,
    vitesse: f64,
    angle: f64,
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
                    angle: 0.0,
                },
                Planete {
                    nom: "Venus".to_string(),
                    distance: 108.2,
                    vitesse: 35.0,
                    angle: 0.0,
                },
                Planete {
                    nom: "Terre".to_string(),
                    distance: 149.6,
                    vitesse: 29.8,
                    angle: 0.0,
                },
                Planete {
                    nom: "Mars".to_string(),
                    distance: 227.9,
                    vitesse: 24.1,
                    angle: 0.0,
                },
            ],
        }
    }

    fn mettre_a_jour(&mut self, temps: f64) {
        for planete in &mut self.planetes {
            planete.angle += planete.vitesse * temps / planete.distance;
            planete.angle = planete.angle % (2.0 * PI);
        }
    }

    fn afficher(&self) {
        for planete in &self.planetes {
            println!(
                "Planète: {}, Distance: {} millions de km, Vitesse: {} km/s, Angle: {} radians",
                planete.nom, planete.distance, planete.vitesse, planete.angle
            );
        }
    }
}

fn main() {
    let mut systeme_solaire = SystemeSolaire::new();
    systeme_solaire.afficher();
    systeme_solaire.mettre_a_jour(1.0);
    println!("\nAprès 1 seconde :");
    systeme_solaire.afficher();
}
