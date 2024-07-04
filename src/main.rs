use ggez::*;
use std::f64::consts::PI;

struct Planete {
    nom: String,
    distance: f64,
    vitesse: f64,
    angle: f64,
    rayon: f64,
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
                    rayon: 2.0,
                },
                Planete {
                    nom: "Venus".to_string(),
                    distance: 108.2,
                    vitesse: 35.0,
                    angle: 0.0,
                    rayon: 3.0,
                },
                Planete {
                    nom: "Terre".to_string(),
                    distance: 149.6,
                    vitesse: 29.8,
                    angle: 0.0,
                    rayon: 4.0,
                },
                Planete {
                    nom: "Mars".to_string(),
                    distance: 227.9,
                    vitesse: 24.1,
                    angle: 0.0,
                    rayon: 3.5,
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

    fn dessiner(&self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::WHITE);
        for planete in &self.planetes {
            let x = 400.0 + planete.distance as f32 * planete.angle.cos() as f32;
            let y = 300.0 + planete.distance as f32 * planete.angle.sin() as f32;
            let cercle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                [x, y],
                planete.rayon as f32,
                0.1,
                graphics::Color::BLACK,
            )?;
            graphics::draw(ctx, &cercle, graphics::DrawParam::default())?;
        }
        graphics::present(ctx)
    }
}

struct Jeu {
    systeme_solaire: SystemeSolaire,
}

impl event::EventHandler for Jeu {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.systeme_solaire.mettre_a_jour(0.1);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.systeme_solaire.dessiner(ctx)
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("Systeme Solaire", "Auteur")
        .build()
        .expect("Erreur lors de la création du contexte");
    let jeu = Jeu {
        systeme_solaire: SystemeSolaire::new(),
    };
    event::run(ctx, event_loop, jeu)
}
