use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::str::FromStr;
use tiled::ObjectLayer;

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

enum MapEntity {
    Player,
    Bat,
    Door,
}

impl FromStr for MapEntity {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MapEntity::*;

        Ok(match s {
            "PLAYER" => Player,
            "BAT" => Bat,
            "DOOR" => Door,
            _ => return Err(()),
        })
    }
}

impl quote::ToTokens for MapEntity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use MapEntity::*;

        tokens.append_all(match self {
            Bat => quote!(Entity::Bat),
            Player => quote!(Entity::Player),
            Door => quote!(Entity::Door),
        })
    }
}

struct EntityWithPosition(MapEntity, (i32, i32));
struct CollisionRect((i32, i32), (i32, i32));

impl quote::ToTokens for EntityWithPosition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let pos_x = self.1 .0;
        let pos_y = self.1 .1;
        let location = quote!(Vector2D::new(#pos_x, #pos_y));
        let item = &self.0;

        tokens.append_all(quote!(EntityWithPosition(#item, #location)))
    }
}

impl quote::ToTokens for CollisionRect {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let pos_x = self.0 .0;
        let pos_y = self.0 .1;
        let location = quote!(Vector2D::new(#pos_x, #pos_y));
        let width = &self.1 .0;
        let height = &self.1 .1;
        let size = quote!(Vector2D::new(#width, #height));
        tokens.append_all(quote! {
            Rect { position: #location, size: #size }
        })
    }
}

impl quote::ToTokens for Level {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let starting_positions = &self.starting_positions;
        let name = &self.name;
        let collision_rects = &self.collision_rects;

        tokens.append_all(quote! {
            Level::new(
                &[#(#starting_positions),*],
                #name,
                &[#(#collision_rects),*],
            )
        })
    }
}

fn export_tiles(map: &tiled::Map, background: TokenStream) -> TokenStream {
    let ground_layer = map
        .layers()
        .find(|l| l.name == "ground")
        .expect("The ground layer should exist");

    let map_tiles = ground_layer
        .as_tile_layer()
        .expect("The ground layer should be a tile layer");

    let width = map_tiles.width().expect("Map should be finite");
    let height = map_tiles.height().unwrap();

    let map_tiles = (0..(height * width))
        .map(|pos| (pos % width, pos / width))
        .map(|(x, y)| {
            let tile = map_tiles.get_tile(x as i32, y as i32);

            match tile {
                Some(tile) => {
                    let tile_id = tile.id();
                    let vflip = tile.flip_v;
                    let hflip = tile.flip_h;

                    quote! {
                        backgrounds::#background.tile_settings[#tile_id as usize]
                            .hflip(#hflip)
                            .vflip(#vflip)
                    }
                }
                None => {
                    quote! { TileSetting::BLANK }
                }
            }
        });

    quote! {&[#(#map_tiles),*]}
}

struct Level {
    starting_positions: Vec<EntityWithPosition>,
    name: String,
    collision_rects: Vec<CollisionRect>,
}

enum Shape {
    Rect(CollisionRect),
    Point(EntityWithPosition),
}

fn export_level(map: &tiled::Map) -> Level {
    let entity_layer = map
        .layers()
        .find(|layer| layer.name == "entities")
        .and_then(|layer| layer.as_object_layer())
        .expect("The 'entities' object layer should exist");

    let starting_positions = extract_objects_from_layer(entity_layer)
        .map(|shape| match shape {
            Shape::Point(entity) => entity,
            _ => panic!("expected points only"),
        })
        .collect();

    let collision_layer = map
        .layers()
        .find(|layer| layer.name == "collision")
        .and_then(|layer| layer.as_object_layer())
        .expect("The 'collision' object layer should exist");

    let collision_rects = extract_objects_from_layer(collision_layer)
        .map(|shape| match shape {
            Shape::Rect(rect) => rect,
            _ => panic!("expected rectangles only"),
        })
        .collect();

    let Some(tiled::PropertyValue::StringValue(level_name)) = map.properties.get("NAME") else {
        panic!("Level property 'NAME' must be a string")
    };

    Level {
        starting_positions,
        name: level_name.clone(),
        collision_rects,
    }
}

fn extract_objects_from_layer(objects: ObjectLayer<'_>) -> impl Iterator<Item = Shape> + '_ {
    objects.objects().into_iter().map(|obj| match obj.shape {
        tiled::ObjectShape::Rect { width, height } => Shape::Rect(CollisionRect(
            (obj.x as i32, obj.y as i32),
            (width as i32, height as i32),
        )),
        tiled::ObjectShape::Point(x, y) => {
            let entity: MapEntity = obj
                .name
                .parse()
                .unwrap_or_else(|_| panic!("unknown object type {}", obj.name));

            Shape::Point(EntityWithPosition(entity, (x as i32, y as i32)))
        }
        _ => panic!("unsupported object shape"),
    })
}
