use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::str::FromStr;
use tiled::ObjectLayer;

use tiled::Layer;

const LEVEL_NAMES: &[&str] = &["level1"];

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR environment variable must be specified");

    let mut tile_loader = tiled::Loader::new();

    let levels = LEVEL_NAMES
        .iter()
        .map(|x| x.to_string())
        .map(|level| load_level(&mut tile_loader, &format!("./maps/levels/{level}.tmx")))
        .collect::<Vec<_>>();
    let levels_tiles = levels.iter().map(|level| &level.0);
    let levels_data = levels.iter().map(|level| &level.1);

    let tilemaps_output = quote! {
        use agb::display::tiled::TileSetting;

        pub const LEVELS_MAP: &[&[TileSetting]] = &[#(#levels_tiles),*];
    };

    let levels_output = quote! {
        pub const LEVELS: &[Level] = &[#(#levels_data),*];
    };

    {
        let tilemaps_output_file = File::create(format!("{out_dir}/tilemaps.rs"))
            .expect("Failed to open tilemaps.rs for writing");
        let mut tilemaps_writer = BufWriter::new(tilemaps_output_file);
        write!(&mut tilemaps_writer, "{tilemaps_output}").unwrap();
    }

    {
        let levels_output_file = File::create(format!("{out_dir}/levels.rs"))
            .expect("Failed to open levels.rs for writing");
        let mut levels_output_writer = BufWriter::new(levels_output_file);

        write!(&mut levels_output_writer, "{levels_output}").unwrap();
    }
}

fn load_level(loader: &mut tiled::Loader, filename: &str) -> (TokenStream, Level) {
    let level_map = load_tmx(loader, filename);
    let tiles = export_tiles(&level_map, quote!(level));
    let data = export_level(&level_map);

    (tiles, data)
}

fn load_tmx(loader: &mut tiled::Loader, filename: &str) -> tiled::Map {
    println!("cargo:rerun-if-changed={filename}");
    loader.load_tmx_map(filename).expect("failed to load map")
}

enum Entity {
    Player,
    Bat,
    Door,
}

impl FromStr for Entity {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Entity::*;

        Ok(match s {
            "PLAYER" => Player,
            "BAT" => Bat,
            "DOOR" => Door,
            _ => return Err(()),
        })
    }
}

impl quote::ToTokens for Entity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use Entity::*;

        tokens.append_all(match self {
            Bat => quote!(Item::Bat),
            Player => quote!(Item::Player),
            Door => quote!(Item::Door),
        })
    }
}

struct EntityWithPosition(Entity, (i32, i32));

impl quote::ToTokens for EntityWithPosition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let pos_x = self.1 .0;
        let pos_y = self.1 .1;
        let location = quote!(Vector2D::new(#pos_x, #pos_y));
        let item = &self.0;

        tokens.append_all(quote!(Entity(#item, #location)))
    }
}

impl quote::ToTokens for Level {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let starting_positions = &self.starting_positions;
        let name = &self.name;

        tokens.append_all(quote! {
            Level::new(
                &[#(#starting_positions),*],
                #name,
            )
        })
    }
}

struct Level {
    starting_positions: Vec<EntityWithPosition>,
    name: String,
}

fn extract_objects_from_layer(
    objects: ObjectLayer<'_>,
) -> impl Iterator<Item = EntityWithPosition> + '_ {
    objects.objects().map(|obj| {
        let entity: Entity = obj
            .name
            .parse()
            .unwrap_or_else(|_| panic!("unknown object type {}", obj.name));

        let x = (obj.x / 16.0) as i32;
        let y = (obj.y / 16.0) as i32;

        EntityWithPosition(entity, (x, y))
    })
}

fn export_tiles(map: &tiled::Map, background: TokenStream) -> TokenStream {
    let ground_layer = map
        .layers()
        .find(|l| l.name == "ground")
        .expect("The ground layer should exist");

    let map_tiles = ground_layer
        .as_tile_layer()
        .expect("The ground layer should be a tile layer");

    let width = map_tiles.width().unwrap();
    let height = map_tiles.height().unwrap();

    let map_tiles = (0..(height * width))
        .map(|pos| (pos % width, pos / width))
        .map(|(x, y)| {
        let tile = map_tiles.get_tile(x as i32, y as i32);

        match tile {
            Some(tile) => {
                let vflip = tile.flip_v;
                let hflip = tile.flip_h;
                let tile_id = tile.id();

                quote! { backgrounds::#background.tile_settings[#tile_id as usize].hflip(#hflip).vflip(#vflip) }
            }
            None => {
                quote! { TileSetting::BLANK }
            }
        }
    });

    quote! {&[#(#map_tiles),*]}
}

fn export_level(map: &tiled::Map) -> Level {
    let objects = map
        .layers()
        .find(|layer| layer.name == "entities")
        .and_then(|layer| layer.as_object_layer())
        .expect("The entities object layer should exist");

    let starting_positions = extract_objects_from_layer(objects);

    let Some(tiled::PropertyValue::StringValue(level_name)) = map.properties.get("NAME") else {
        panic!("Level name must be a string")
    };

    Level {
        starting_positions: starting_positions.collect(),
        name: level_name.clone(),
    }
}

fn get_spawn_locations(object_group: &Layer, enemy_type: &str) -> (Vec<u16>, Vec<u16>) {
    let mut spawns = object_group
        .as_object_layer()
        .unwrap()
        .objects()
        .filter(|object| object.user_type == enemy_type)
        .map(|object| (object.x as u16, object.y as u16))
        .collect::<Vec<_>>();

    spawns.sort_by(|a, b| a.0.cmp(&b.0));

    let xs = spawns.iter().map(|pos| pos.0).collect::<Vec<_>>();
    let ys = spawns.iter().map(|pos| pos.1).collect::<Vec<_>>();

    (xs, ys)
}
