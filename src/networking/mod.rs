use bevy::prelude::*;

mod client;
mod server;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin{
    fn build(&self,app:&mut App){
        app.add_systems(Startup,client::connect);
    }
}