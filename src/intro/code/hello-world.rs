// Copyright (c) godot-rust; Bromeon and contributors.
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// ANCHOR: entry-point
// ANCHOR: class-declaration
use godot::prelude::*;
// ANCHOR_END: class-declaration

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
// ANCHOR_END: entry-point

// ANCHOR: class-declaration
use godot::classes::Sprite2D;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f32,
    angular_speed: f32,

    base: Base<Sprite2D>,
}
// ANCHOR_END: class-declaration

// ANCHOR: init
// ANCHOR: physics-process
// ANCHOR: rotate
use godot::classes::ISprite2D;

#[godot_api]
impl ISprite2D for Player {
    // ANCHOR_END: init
    // ANCHOR_END: physics-process
    // ANCHOR_END: rotate
    // ANCHOR: init
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            speed: 400.0,
            angular_speed: std::f32::consts::PI,
            base,
        }
    }
    // ANCHOR_END: init

    // delta may also be f64.
    // ANCHOR: physics-process
    fn physics_process(&mut self, delta: f32) {
        // In GDScript, this would be:
        // rotation += angular_speed * delta

        let radians = self.angular_speed * delta;
        self.base_mut().rotate(radians);
        // The 'rotate' method requires a f32,
        // therefore we convert 'self.angular_speed * delta' which is a f64 to a f32
    }
    // ANCHOR_END: physics-process
    // ANCHOR: init
}
// ANCHOR_END: init
// ANCHOR_END: rotate

// amount is allowed to be f64 too
// ANCHOR: custom-api
#[godot_api]
impl Player {
    #[func]
    fn increase_speed(&mut self, amount: f32) {
        self.speed += amount;
        self.signals().speed_increased().emit();
    }

    #[signal]
    fn speed_increased();
}
// ANCHOR_END: custom-api
