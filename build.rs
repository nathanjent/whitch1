use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::str::FromStr;
use tiled::PropertyValue;

static LEVEL_NAMES: &[&str] = &["level1", "level2"];

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
        pub static LEVELS_MAP: &[&[TileSetting]] = &[#(#levels_tiles),*];
    };

    let levels_output = quote! {
        pub static LEVELS: &[Level] = &[#(#levels_data),*];
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

fn load_level<'a>(loader: &'a mut tiled::Loader, filename: &'a str) -> (TokenStream, Level) {
    let level_map = load_tmx(loader, filename);
    let tiles = export_tiles(&level_map, quote!(level));
    let data = export_level(&level_map);

    (tiles, data)
}

fn load_tmx(loader: &mut tiled::Loader, filename: &str) -> tiled::Map {
    println!("cargo:rerun-if-changed={filename}");
    loader.load_tmx_map(filename).expect("failed to load map")
}

enum EntityType {
    Player,
    Bat,
    Door,
}

impl FromStr for EntityType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use EntityType::*;

        Ok(match s {
            "PLAYER" => Player,
            "BAT" => Bat,
            "DOOR" => Door,
            _ => return Err(()),
        })
    }
}

impl quote::ToTokens for EntityType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use EntityType::*;

        tokens.append_all(match self {
            Bat => quote!(EntityType::Bat),
            Player => quote!(EntityType::Player),
            Door => quote!(EntityType::Door),
        })
    }
}

impl quote::ToTokens for Behavior {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use Behavior::*;

        tokens.append_all(match self {
            Input => quote!(Behavior::Input),
            Player => quote!(Behavior::Player),
            Flap => quote!(Behavior::Flap),
        })
    }
}

enum Behavior {
    Input,
    Player,
    Flap,
}

impl FromStr for Behavior {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Behavior::*;

        Ok(match s {
            "Input" => Input,
            "Player" => Player,
            "Flap" => Flap,
            _ => return Err(()),
        })
    }
}

struct Entity(
    EntityType,
    (i32, i32),
    Option<(i32, i32)>,
    Vec<Behavior>,
    (i32, i32),
);
struct CollisionRect((i32, i32), (i32, i32));

impl quote::ToTokens for Entity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let entity_type = &self.0;
        let pos_x = &self.1 .0;
        let pos_y = &self.1 .1;
        let location = quote!(Vector2D::new(#pos_x, #pos_y));
        let size = match &self.2 {
            Some((width, height)) => quote!(Some(Vector2D::new(#width, #height))),
            None => quote!(None),
        };
        let behaviors = &self.3;

        let offset_x = &self.4 .0;
        let offset_y = &self.4 .1;
        let sprite_offset = quote!(Vector2D::new(#offset_x, #offset_y));

        tokens.append_all(
            quote!(Entity(#entity_type, #location, #size, &[#(#behaviors),*], #sprite_offset)),
        )
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

impl<'a> quote::ToTokens for Level {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let width = &self.width;
        let height = &self.height;
        let starting_positions = &self.starting_positions;
        let name = &self.name;
        let collision_rects = &self.collision_rects;

        tokens.append_all(quote! {
            Level::new(
                #width,
                #height,
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
    width: u32,
    height: u32,
    starting_positions: Vec<Entity>,
    name: String,
    collision_rects: Vec<CollisionRect>,
}

fn export_level(map: &tiled::Map) -> Level {
    let entity_layer = map
        .layers()
        .find(|layer| layer.name == "entities")
        .and_then(|layer| layer.as_object_layer())
        .expect("The 'entities' object layer should exist");

    let starting_positions = entity_layer
        .objects()
        .into_iter()
        .filter_map(|obj| match obj.user_type.as_str() {
            "ENTITY" => {
                let entity_type = obj
                    .name
                    .parse()
                    .unwrap_or_else(|_| panic!("unknown object type {}", obj.name));

                let behaviors: Vec<Behavior> = match obj.properties.get("behaviors") {
                    Some(behaviors) => match behaviors {
                        PropertyValue::StringValue(text) => text
                            .split("\n")
                            .map(Behavior::from_str)
                            .filter_map(Result::ok)
                            .collect(),
                        _ => panic!("behaviors should be a string value"),
                    },
                    None => Vec::new(),
                };

                let offset_x = obj
                    .properties
                    .get("offset_x")
                    .map(|p| match p {
                        PropertyValue::IntValue(x) => *x,
                        _ => panic!("offset_x should be an int value"),
                    })
                    .unwrap_or(0);
                let offset_y = obj
                    .properties
                    .get("offset_y")
                    .map(|p| match p {
                        PropertyValue::IntValue(y) => *y,
                        _ => panic!("offset_x should be an int value"),
                    })
                    .unwrap_or(0);

                match obj.shape {
                    tiled::ObjectShape::Rect { width, height } => Some(Entity(
                        entity_type,
                        (obj.x as i32, obj.y as i32),
                        Some((width as i32, height as i32)),
                        behaviors,
                        (offset_x, offset_y),
                    )),
                    tiled::ObjectShape::Point(x, y) => Some(Entity(
                        entity_type,
                        (x as i32, y as i32),
                        None,
                        behaviors,
                        (offset_x, offset_y),
                    )),
                    _ => None,
                }
            }
            _ => None,
        })
        .collect();

    let collision_layer = map
        .layers()
        .find(|layer| layer.name == "collision")
        .and_then(|layer| layer.as_object_layer())
        .expect("The 'collision' object layer should exist");

    let collision_rects = collision_layer
        .objects()
        .into_iter()
        .map(|obj| match obj.shape {
            tiled::ObjectShape::Rect { width, height } => {
                CollisionRect((obj.x as i32, obj.y as i32), (width as i32, height as i32))
            }
            _ => panic!("expected rectangles only"),
        })
        .collect();

    let Some(tiled::PropertyValue::StringValue(level_name)) = map.properties.get("NAME") else {
        panic!("Level property 'NAME' must be a string")
    };

    Level {
        width: map.width,
        height: map.height,
        starting_positions,
        name: level_name.clone(),
        collision_rects,
    }
}
