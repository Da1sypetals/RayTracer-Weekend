use raytrace::config::materials::MaterialMap;

fn main() {
    let mats = MaterialMap::configured("config/materials.toml").unwrap();
    mats.map
        .into_iter()
        .for_each(|(name, m)| println!("{} => {:?}", name, m));
}
