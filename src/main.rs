use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Font, Mesh, Text};
use ggez::input::{self, keyboard};
use ggez::{Context, ContextBuilder, GameResult};
use nalgebra as na;
use std::f64::consts::PI;

struct Lune {
    nom: String,
    distance: f64,
    vitesse: f64,
    angle: f64,
    rayon: f64,
}

struct Planete {
    nom: String,
    distance: f64,
    vitesse: f64,
    angle: f64,
    rayon: f64,
    couleur: Color,
    lunes: Vec<Lune>,
}

struct SystemeSolaire {
    planetes: Vec<Planete>,
    zoom: f32,
    camera_position: na::Point2<f32>,
}

impl SystemeSolaire {
    fn new() -> SystemeSolaire {
        SystemeSolaire {
            planetes: vec![
                Planete {
                    nom: "Mercure".to_string(),
                    distance: 40.0,
                    vitesse: 47.4,
                    angle: 0.0,
                    rayon: 2.0,
                    couleur: Color::new(0.5, 0.5, 0.5, 1.0), // Gris
                    lunes: vec![],
                },
                Planete {
                    nom: "Venus".to_string(),
                    distance: 60.0,
                    vitesse: 35.0,
                    angle: 0.0,
                    rayon: 3.0,
                    couleur: Color::new(1.0, 0.5, 0.0, 1.0), // Orange
                    lunes: vec![],
                },
                Planete {
                    nom: "Terre".to_string(),
                    distance: 80.0,
                    vitesse: 29.8,
                    angle: 0.0,
                    rayon: 4.0,
                    couleur: Color::new(0.0, 0.0, 1.0, 1.0), // Bleu
                    lunes: vec![Lune {
                        nom: "Lune".to_string(),
                        distance: 8.0,
                        vitesse: 1.0,
                        angle: 0.0,
                        rayon: 1.0,
                    }],
                },
                Planete {
                    nom: "Mars".to_string(),
                    distance: 100.0,
                    vitesse: 24.1,
                    angle: 0.0,
                    rayon: 3.5,
                    couleur: Color::new(1.0, 0.0, 0.0, 1.0), // Rouge
                    lunes: vec![
                        Lune {
                            nom: "Phobos".to_string(),
                            distance: 6.0,
                            vitesse: 2.0,
                            angle: 0.0,
                            rayon: 1.0,
                        },
                        Lune {
                            nom: "Deimos".to_string(),
                            distance: 12.0,
                            vitesse: 1.5,
                            angle: 0.0,
                            rayon: 1.2,
                        },
                    ],
                },
                Planete {
                    nom: "Jupiter".to_string(),
                    distance: 140.0,
                    vitesse: 13.1,
                    angle: 0.0,
                    rayon: 6.0,
                    couleur: Color::new(0.5, 0.4, 0.3, 1.0), // Marron
                    lunes: vec![
                        Lune {
                            nom: "Io".to_string(),
                            distance: 10.0,
                            vitesse: 1.5,
                            angle: 0.0,
                            rayon: 1.5,
                        },
                        Lune {
                            nom: "Europa".to_string(),
                            distance: 12.0,
                            vitesse: 1.3,
                            angle: 0.0,
                            rayon: 1.4,
                        },
                        Lune {
                            nom: "Ganymède".to_string(),
                            distance: 14.0,
                            vitesse: 1.1,
                            angle: 0.0,
                            rayon: 1.8,
                        },
                        Lune {
                            nom: "Callisto".to_string(),
                            distance: 16.0,
                            vitesse: 1.0,
                            angle: 0.0,
                            rayon: 1.6,
                        },
                    ],
                },
                Planete {
                    nom: "Saturne".to_string(),
                    distance: 180.0,
                    vitesse: 9.7,
                    angle: 0.0,
                    rayon: 5.5,
                    couleur: Color::new(1.0, 1.0, 0.0, 1.0), // Jaune
                    lunes: vec![
                        Lune {
                            nom: "Titan".to_string(),
                            distance: 10.0,
                            vitesse: 1.2,
                            angle: 0.0,
                            rayon: 1.4,
                        },
                    ],
                },
                Planete {
                    nom: "Uranus".to_string(),
                    distance: 220.0,
                    vitesse: 6.8,
                    angle: 0.0,
                    rayon: 5.0,
                    couleur: Color::new(0.5, 1.0, 1.0, 1.0), // Cyan
                    lunes: vec![
                        Lune {
                            nom: "Titania".to_string(),
                            distance: 8.0,
                            vitesse: 1.1,
                            angle: 0.0,
                            rayon: 1.2,
                        },
                    ],
                },
                Planete {
                    nom: "Neptune".to_string(),
                    distance: 260.0,
                    vitesse: 5.4,
                    angle: 0.0,
                    rayon: 4.5,
                    couleur: Color::new(0.0, 0.0, 0.5, 1.0), // Bleu foncé
                    lunes: vec![
                        Lune {
                            nom: "Triton".to_string(),
                            distance: 10.0,
                            vitesse: 1.0,
                            angle: 0.0,
                            rayon: 1.3,
                        },
                    ],
                },
            ],
            zoom: 1.0,
            camera_position: na::Point2::new(400.0, 300.0),
        }
    }

    fn mettre_a_jour(&mut self, temps: f64) {
        for planete in &mut self.planetes {
            planete.angle += planete.vitesse * temps / planete.distance;
            planete.angle = planete.angle % (2.0 * PI);
            for lune in &mut planete.lunes {
                lune.angle += lune.vitesse * temps / lune.distance;
                lune.angle = lune.angle % (2.0 * PI);
            }
        }
    }

    fn dessiner(&self, ctx: &mut Context) -> GameResult {
        let (width, height) = graphics::drawable_size(ctx);

        graphics::clear(ctx, Color::WHITE);

        // Dessiner le Soleil
        let soleil = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [self.camera_position.x, self.camera_position.y],
            10.0 * self.zoom,
            0.1,
            Color::YELLOW,
        )?;
        graphics::draw(ctx, &soleil, DrawParam::default())?;

        // Dessiner les orbites et les planètes
        for planete in &self.planetes {
            let x = self.camera_position.x + planete.distance as f32 * planete.angle.cos() as f32 * self.zoom;
            let y = self.camera_position.y + planete.distance as f32 * planete.angle.sin() as f32 * self.zoom;

            // Dessiner l'orbite
            let orbite = Mesh::new_circle(
                ctx,
                DrawMode::stroke(1.0),
                [self.camera_position.x, self.camera_position.y],
                planete.distance as f32 * self.zoom,
                0.1,
                Color::BLACK,
            )?;
            graphics::draw(ctx, &orbite, DrawParam::default())?;

            // Dessiner la planète
            let cercle = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                [x, y],
                planete.rayon as f32 * self.zoom,
                0.1,
                planete.couleur,
            )?;
            graphics::draw(ctx, &cercle, DrawParam::default())?;

            // Dessiner le nom de la planète
            let font = Font::default();
            let text = Text::new((planete.nom.as_str(), font, 14.0));
            let text_pos = [
                x - (text.width(ctx) as f32) / 2.0,
                y - planete.rayon as f32 * self.zoom - 14.0,
            ];
            graphics::draw(ctx, &text, (text_pos, 0.0, Color::BLACK))?;

            // Dessiner les lunes
            for lune in &planete.lunes {
                let lune_x = x + lune.distance as f32 * lune.angle.cos() as f32 * self.zoom;
                let lune_y = y + lune.distance as f32 * lune.angle.sin() as f32 * self.zoom;

                // Dessiner l'orbite de la lune
                let orbite_lune = Mesh::new_circle(
                    ctx,
                    DrawMode::stroke(1.0),
                    [x, y],
                    lune.distance as f32 * self.zoom,
                    0.1,
                    Color::new(0.5, 0.5, 0.5, 1.0), // Gris
                )?;
                graphics::draw(ctx, &orbite_lune, DrawParam::default())?;

                // Dessiner la lune
                let cercle_lune = Mesh::new_circle(
                    ctx,
                    DrawMode::fill(),
                    [lune_x, lune_y],
                    lune.rayon as f32 * self.zoom,
                    0.1,
                    Color::new(0.5, 0.5, 0.5, 1.0),
                )?;
                graphics::draw(ctx, &cercle_lune, DrawParam::default())?;

                // Dessiner le nom de la lune
                let text_lune = Text::new((lune.nom.as_str(), font, 12.0));
                let text_pos_lune = [
                    lune_x - (text_lune.width(ctx) as f32) / 2.0,
                    lune_y - lune.rayon as f32 * self.zoom - 14.0,
                ];
                graphics::draw(ctx, &text_lune, (text_pos_lune, 0.0, Color::BLACK))?;
            }
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

struct Jeu {
    systeme_solaire: SystemeSolaire,
}

impl event::EventHandler for Jeu {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.systeme_solaire.mettre_a_jour(0.1);

        let camera_speed = 10.0;

        if keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::Right) {
            self.systeme_solaire.camera_position.x -= camera_speed;
        }
        if keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::Left) {
            self.systeme_solaire.camera_position.x += camera_speed;
        }
        if keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::Down) {
            self.systeme_solaire.camera_position.y -= camera_speed;
        }
        if keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::Up) {
            self.systeme_solaire.camera_position.y += camera_speed;
        }
        if keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::Z) {
            self.systeme_solaire.zoom *= 1.1;
        }
        if keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::S) {
            self.systeme_solaire.zoom /= 1.1;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.systeme_solaire.dessiner(ctx)?;
        Ok(())
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
