use bevy::prelude::*;

mod movement; //add submodule movement

pub struct GameplayPlugin; //this defines a type.we will use it as name of our gameplay plugin that we can plug into App inside main.rs .this holds no field.
/*
a Plugin is a trait provided by rust.If we implement Plugin on a type then 
on adding that plugin ,our build function will be called immedietely.
When an App registers a plugin, the plugin’s Plugin::build function is run. By default, a plugin can only be added once to an App.
*/
impl Plugin for GameplayPlugin{ //implementation of Plugin trait
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,movement::setup) //runs once before the first frame when the app starts
        .add_systems(Update,movement::move_player); //runs every frame
    }
}