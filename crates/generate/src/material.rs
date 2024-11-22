use std::{fs, io::Write};

use rand::{thread_rng, Rng};
use raytrace::{
    config::materials::MaterialMap, helpers::types::color, materials::material::Material,
};

fn random_color<R: Rng>(rng: &mut R) -> color {
    color::new(
        rng.gen_range(0.0..=1.0),
        rng.gen_range(0.0..=1.0),
        rng.gen_range(0.0..=1.0),
    )
}

fn generate_random_material<R: Rng>(rng: &mut R) -> Material {
    match rng.gen_range(0..4) {
        0 => Material::Lambertian {
            albedo: random_color(rng),
        },
        1 => Material::Metal {
            albedo: random_color(rng),
        },
        2 => Material::FuzzedMetal {
            albedo: random_color(rng),
            fuzz: rng.gen_range(0.0..=0.4),
        },
        3 => Material::Dielectric {
            eta: {
                let scale = rng.gen_range(1.0..2.0);
                let inv = rng.gen_bool(0.5);
                if inv {
                    1.0 / scale
                } else {
                    scale
                }
            },
        },
        _ => unreachable!(),
    }
}

fn main() {
    let n = 32;
    let output_path = "generated/materials.toml";

    let mut rng = thread_rng();
    let mats = MaterialMap {
        map: (0..n)
            .into_iter()
            .map(|i| {
                let name = format!("mat{}", i);
                let mat = generate_random_material(&mut rng);
                (name, mat)
            })
            .collect(),
    };

    let mats_str = toml::to_string(&mats).unwrap();
    let mut file = fs::File::create(output_path).unwrap();
    file.write_all(mats_str.as_bytes()).unwrap();
}
