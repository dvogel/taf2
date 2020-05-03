use std::borrow::Borrow;
use amethyst::{
    assets::{Handle, AssetStorage, Loader},
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    ecs::prelude::{Component, DenseVecStorage},
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};

use log::info;

#[derive(Clone, Copy)]
enum FBPosition {
    QB,
    HB,
    FB,
    WR,
    TE,
    LB,
    DE,
    DT,
    NT,
    FS,
    SS,
    CB,
}

#[derive(Clone, Copy)]
pub struct FBPlayer {
    number: u8,
    position: FBPosition,
    team: u8,
}

pub struct FBState {
    players: Vec<FBPlayer>,
}

impl FBState {
    fn build_std_21_offense(team: u8) -> Vec<FBPlayer> {
        let qb = FBPlayer{ number: 12 + team, position: FBPosition::QB, team: team };
        let hb = FBPlayer{ number: 20 + team, position: FBPosition::HB, team: team };
        let fb = FBPlayer{ number: 33 + team, position: FBPosition::FB, team: team };
        let te = FBPlayer{ number: 88 + team, position: FBPosition::TE, team: team };
        let wr1 = FBPlayer{ number: 84 + team, position: FBPosition::WR, team: team };
        let wr2 = FBPlayer{ number: 81 + team, position: FBPosition::WR, team: team };
        let c = FBPlayer{ number: 68 + team, position: FBPosition::HB, team: team };
        let lg = FBPlayer{ number: 77 + team, position: FBPosition::HB, team: team };
        let rg = FBPlayer{ number: 68 + team, position: FBPosition::HB, team: team };
        let lt = FBPlayer{ number: 69 + team, position: FBPosition::HB, team: team };
        let rt = FBPlayer{ number: 71 + team, position: FBPosition::HB, team: team };
        return vec![qb, hb, fb, te, wr1, wr2, c, lg, rg, lt, rt];
    }

    fn build_std_34_defense(team: u8) -> Vec<FBPlayer> {
        let nt = FBPlayer{ number: 91 + team, position: FBPosition::DT, team: team };
        let dt1 = FBPlayer{ number: 94 + team, position: FBPosition::DE, team: team };
        let dt2 = FBPlayer{ number: 96 + team, position: FBPosition::DE, team: team };
        let ilb1 = FBPlayer{ number: 56 + team, position: FBPosition::LB, team: team };
        let ilb2 = FBPlayer{ number: 60 + team, position: FBPosition::LB, team: team };
        let olb1 = FBPlayer{ number: 63 + team, position: FBPosition::LB, team: team };
        let olb2 = FBPlayer{ number: 52 + team, position: FBPosition::LB, team: team };
        let cb1 = FBPlayer{ number: 20 + team, position: FBPosition::CB, team: team };
        let cb2 = FBPlayer{ number: 26 + team, position: FBPosition::CB, team: team };
        let fs = FBPlayer{ number: 30 + team, position: FBPosition::FS, team: team };
        let ss = FBPlayer{ number: 36 + team, position: FBPosition::SS, team: team };
        return vec![nt, dt1, dt2, ilb1, ilb2, olb1, olb2, cb1, cb2, fs, ss];
    }

    fn build_std_team() -> Vec<FBPlayer> {
        let mut players = FBState::build_std_21_offense(0);
        players.extend(FBState::build_std_34_defense(1));
        return players;
    }

    pub fn new() -> FBState {
        let players = Vec::new();
        return FBState { players };
    }
}

impl Component for FBPlayer {
    type Storage = DenseVecStorage<Self>;
}

impl SimpleState for FBState {
    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<FBPlayer>();

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // Load our sprites and display them
        let player_sprite_sheet = load_player_sprites(world);
        let player_sprites: Vec<SpriteRender> = (0..2).map(|i| SpriteRender {
            sprite_sheet: player_sprite_sheet.clone(),
            sprite_number: i,
        }).collect();
        init_player_sprites(world, &self.players, &player_sprites, &dimensions);

        // Place the camera
        init_camera(world, &dimensions);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        // Keep going
        Trans::None
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    // Center the camera in the middle of the screen, and let it cover
    // the entire screen
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

fn load_player_sprites(world: &mut World) -> Handle<SpriteSheet> {
    // Load the texture for our sprites. We'll later need to
    // add a handle to this texture to our `SpriteRender`s, so
    // we need to keep a reference to it.
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/GBP.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    // Load the spritesheet definition file, which contains metadata on our
    // spritesheet texture.
    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites/GBP.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    return sheet_handle;
}

fn init_player_sprites(world: &mut World, players: &[FBPlayer], sprites: &[SpriteRender], dimensions: &ScreenDimensions) {
    const PLAYER_WIDTH: f32 = 40.;
    const PLAYER_HEIGHT: f32 = 40.;
    for (i, player) in players.iter().enumerate() {
        let mut transform = Transform::default();
        transform.set_translation_xyz(PLAYER_WIDTH * i as f32, player.team as f32 * PLAYER_HEIGHT, 0.0);
        let cloned_player: FBPlayer = player.clone();
        world.create_entity()
            .with(cloned_player)
            .with(sprites[i % players.len()].clone())
            .with(transform)
            .build();
    }
}
