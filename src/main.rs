use bevy::prelude::*;
mod gameplay; //add gameplay module
mod networking; //add networking module

fn main(){
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(gameplay::GameplayPlugin)
    .add_plugins(networking::NetworkPlugin);
}