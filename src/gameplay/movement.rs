use bevy::prelude::*;

//marker component
#[derive(Component)]
pub struct Player;//this is a marker component.a struct with zero fields.it's like a sticky note.it's to be attached on a component to identify that as Player.
//i kept Player as public so later we can use it in another module like networking.
pub fn setup(mut commands: Commands){
    //spawns a 2d camera
    commands.spawn(Camera2d);
    //spawn a white square
    commands.spawn((
        Sprite{//Sprite is a built in component that will be drawn in 2d
            color:Color::srgb(1.0,0.0,0.0),
            custom_size: Some(Vec2::new(100.0,100.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player,
    ));
}
pub fn move_player(
    keyboard:Res<ButtonInput<KeyCode>>,//to get keyboard state
    mut query:Query<&mut Transform,With<Player>>,//Query is a built in data_type that has two parameter:Query<DATA,FILTER>
    time:Res<Time>
){
    //to extract exactly one player's transform.if 0 player or more than 1 player then it will panic and crash
    let mut transform=query.single_mut().expect("either no player or multiple player");
    let speed=200.0;
    let mut direction=Vec2::ZERO; //Vec2 is from Bevy's math library.ZERO is shortcut for x:0.0,y:0.0

    //check keyboard input
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp){
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown){
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight){
        direction.x += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft){
        direction.x -= 1.0;
    }
    if direction.length()>0.0{//to prevent division by 0
        direction=direction.normalize();//to fix faster diaginal movement issue
    }
    //now change the transform based on input and speed.delta_secs() is to make it framerate-independent
    transform.translation.x+=direction.x*speed*time.delta_secs();
    transform.translation.y+=direction.y*speed*time.delta_secs();

}