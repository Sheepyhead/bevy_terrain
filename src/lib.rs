use bevy::{prelude::*, utils::HashMap};
use rand::{prelude::SliceRandom, thread_rng, Rng};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Terrain::generation);
    }
}

#[derive(Bundle, Default)]
pub struct TerrainBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub terrain: Terrain,
}

#[derive(Component)]
pub struct Terrain {
    gen_type: GenerationType,
    dimensions: UVec2,
    module_dimensions: Vec2,
    map: HashMap<UVec2, TerrainModule>,
    modules: Vec<TerrainModule>,
    state: GenerationState,
}

impl Default for Terrain {
    fn default() -> Self {
        Self {
            gen_type: GenerationType::WaveCollapse,
            dimensions: Default::default(),
            module_dimensions: Default::default(),
            map: Default::default(),
            modules: Default::default(),
            state: GenerationState::JustStarted,
        }
    }
}

impl Terrain {
    pub fn new(gen_type: GenerationType, dimensions: UVec2, module_dimensions: Vec2) -> Self {
        Terrain {
            gen_type,
            dimensions,
            module_dimensions,
            map: HashMap::new(),
            modules: vec![],
            state: GenerationState::JustStarted,
        }
    }

    pub fn with_module(mut self, module: TerrainModule) -> Terrain {
        self.modules.push(module);
        self
    }

    fn generation(
        mut commands: Commands,
        mut stalemate_amount: Local<u32>,
        mut terrains: Query<(Entity, &mut Terrain)>,
    ) {
        for (entity, mut terrain) in terrains.iter_mut() {
            match &terrain.state.clone() {
                GenerationState::JustStarted => {
                    *stalemate_amount = 0;
                    if terrain.modules.is_empty() {
                        warn!("No terrain modules added!");
                        terrain.state = GenerationState::Finished;
                    } else {
                        // Unwrap is fine because we already checked if the vec is empty
                        let mut rng = thread_rng();
                        let module = terrain.modules.choose(&mut rng).unwrap().clone();
                        let x = rng.gen_range(0..terrain.dimensions.x);
                        let y = rng.gen_range(0..terrain.dimensions.y);
                        let pos = UVec2::new(x, y);
                        terrain.map.insert(pos, module);
                        terrain.state = GenerationState::PlacedModules(vec![pos]);
                    }
                }
                GenerationState::PlacedModules(modules) => {
                    *stalemate_amount = 0;
                    let mut inserted_positions = vec![];
                    for pos in modules.iter() {
                        let adjacents = get_adjacent_positions(pos);
                        for adjacent in adjacents.iter() {
                            if terrain.map.contains_key(adjacent)
                                || adjacent.x > terrain.dimensions.x
                                || adjacent.y > terrain.dimensions.y
                            {
                                continue;
                            }
                            let mut allowed_modules = vec![];
                            for module in terrain.modules.iter() {
                                let mut allowed = true;
                                allowed = (module.generation_rule)(Adjacents::get(
                                    *adjacent,
                                    &terrain.map,
                                )) && allowed;
                                if allowed {
                                    allowed_modules.push(module.clone());
                                }
                            }
                            if allowed_modules.len() == 1 {
                                terrain.map.insert(*adjacent, allowed_modules[0].clone());
                                inserted_positions.push(*adjacent);
                            }
                        }
                    }
                    if !inserted_positions.is_empty() {
                        terrain.state = GenerationState::PlacedModules(inserted_positions);
                    } else if terrain.map.len()
                        == (terrain.dimensions.x * terrain.dimensions.y) as usize
                    {
                        terrain.state = GenerationState::Finished;
                    } else {
                        terrain.state = GenerationState::Stalemate;
                    }
                }
                GenerationState::Finished => {
                    *stalemate_amount = 0;
                    for (pos, module) in terrain.map.iter() {
                        commands.entity(entity).with_children(|parent| {
                            parent.spawn_bundle(SpriteBundle {
                                transform: Transform::from_xyz(
                                    pos.x as f32 * terrain.module_dimensions.x,
                                    -(pos.y as f32) * terrain.module_dimensions.y,
                                    0.0,
                                ),
                                texture: module.image.clone(),
                                ..default()
                            });
                        });
                    }
                    commands.entity(entity).remove::<Terrain>();
                }
                GenerationState::Stalemate => {
                    *stalemate_amount += 1;

                    let mut rng = thread_rng();
                    let x = rng.gen_range(0..terrain.dimensions.x);
                    let y = rng.gen_range(0..terrain.dimensions.y);
                    let mut pos = UVec2::new(x, y);
                    while terrain.map.contains_key(&pos) {
                        let x = rng.gen_range(0..terrain.dimensions.x);
                        let y = rng.gen_range(0..terrain.dimensions.y);
                        pos = UVec2::new(x, y);
                    }
                    let mut allowed_modules = vec![];
                    for module in terrain.modules.iter() {
                        if (module.generation_rule)(Adjacents::get(pos, &terrain.map)) {
                            allowed_modules.push(module.clone());
                        }
                    }
                    if let Some(module) = allowed_modules.choose(&mut rng).cloned() {
                        terrain.map.insert(pos, module);
                        terrain.state = GenerationState::PlacedModules(vec![pos]);
                    } else if *stalemate_amount > 10 {
                        warn!("Stalemated too much, aborting");

                        terrain.state = GenerationState::Finished;
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
enum GenerationState {
    JustStarted,
    PlacedModules(Vec<UVec2>),
    Stalemate,
    Finished,
}

pub enum GenerationType {
    WaveCollapse,
}

pub struct Adjacents {
    pub n: Option<TerrainModule>,
    pub s: Option<TerrainModule>,
    pub e: Option<TerrainModule>,
    pub w: Option<TerrainModule>,
}

impl Adjacents {
    pub fn get(pos: UVec2, map: &HashMap<UVec2, TerrainModule>) -> Self {
        Self {
            n: (pos.y != 0)
                .then(|| map.get(&UVec2::new(pos.x, pos.y - 1)))
                .flatten()
                .cloned(),
            s: map.get(&UVec2::new(pos.x, pos.y + 1)).cloned(),
            e: map.get(&UVec2::new(pos.x + 1, pos.y)).cloned(),
            w: (pos.x != 0)
                .then(|| map.get(&UVec2::new(pos.x - 1, pos.y)))
                .flatten()
                .cloned(),
        }
    }

    pub fn has_any(&self) -> bool {
        self.n.is_some() || self.s.is_some() || self.e.is_some() || self.w.is_some()
    }

    pub fn all_are(&self, condition: fn(module: &TerrainModule) -> bool) -> bool {
        (self.n.is_none() || (condition)(self.n.as_ref().unwrap()))
            && (self.s.is_none() || (condition)(self.s.as_ref().unwrap()))
            && (self.e.is_none() || (condition)(self.e.as_ref().unwrap()))
            && (self.w.is_none() || (condition)(self.w.as_ref().unwrap()))
    }

    pub fn list(&self) -> Vec<Option<&TerrainModule>> {
        vec![
            self.n.as_ref(),
            self.s.as_ref(),
            self.e.as_ref(),
            self.w.as_ref(),
        ]
    }
}

#[derive(Clone)]
pub struct TerrainModule {
    pub generation_rule: fn(adjacents: Adjacents) -> bool,
    pub id: u32,
    pub image: Handle<Image>,
}

/// Only checks on lower bounds of u32, because I mean c'mon
fn get_adjacent_positions(pos: &UVec2) -> Vec<UVec2> {
    let mut posses = vec![UVec2::new(pos.x, pos.y + 1), UVec2::new(pos.x + 1, pos.y)];
    (pos.x != 0).then(|| posses.push(UVec2::new(pos.x - 1, pos.y)));
    (pos.y != 0).then(|| posses.push(UVec2::new(pos.x, pos.y - 1)));
    posses
}
