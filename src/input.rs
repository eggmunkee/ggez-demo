

use ggez::{Context};
use ggez::event::{KeyCode,KeyMods};
use specs::{World, WorldExt};

use crate::resources::{InputResource};

pub enum InputKey {
    Left,
    Right,
    Up,
    Down,
    SpaceAction,
    Exit
}


pub struct InputMap;

impl InputMap {
    pub fn map_keycode(keycode: &KeyCode) -> Option<InputKey> {
        //Some(InputKey::Left)

        match keycode {
            KeyCode::A => {
                Some(InputKey::Left)
            },
            KeyCode::D => {
                Some(InputKey::Right)
            },
            KeyCode::W => {
                Some(InputKey::Up)
            },
            KeyCode::S => {
                Some(InputKey::Down)
            },
            KeyCode::Space => {
                Some(InputKey::SpaceAction)
            },
            KeyCode::Escape => {
                Some(InputKey::Exit)
            },
            _ => None
        }
    }

    pub fn key_down(world: &mut World,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,) {

        match Self::map_keycode(&keycode) {
            Some(key) => match key {
                InputKey::Left => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_left(true);
                },
                InputKey::Right => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_right(true);
                },
                InputKey::Up => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_up(true);
                },
                InputKey::Down => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_down(true);
                },
                InputKey::SpaceAction => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_jump(true);
                },
                InputKey::Exit => {
                    ggez::event::quit(ctx);
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn key_up(world: &mut World,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,) {

        match Self::map_keycode(&keycode) {
            Some(key) => match key {
                InputKey::Left => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_left(false);
                },
                InputKey::Right => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_right(false);
                },
                InputKey::Up => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_up(false);
                },
                InputKey::Down => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_down(false);
                },
                InputKey::SpaceAction => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_jump(false);
                },
                _ => {}
            },
            _ => {}
        }
    }
}