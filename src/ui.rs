use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.8, 0.8, 0.8);
const HOVERED_BUTTON: Color = Color::rgb(0.4, 0.8, 0.8);
const PRESSED_BUTTON: Color = Color::rgb(0.4, 1.0, 1.0);

#[derive(Event)]
pub struct GameExitEvent;

#[derive(Event)]
pub struct SimulationStartEvent;

#[derive(Event)]
pub struct SimulationStopEvent;

#[derive(Component)]
struct Button(ButtonType);

enum ButtonType {
    Start,
    Stop,
    Exit,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameExitEvent>()
            .add_event::<SimulationStartEvent>()
            .add_event::<SimulationStopEvent>()
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::FlexEnd,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(100.0),
                        border: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgb(0.2, 0.2, 0.2)),
                    border_color: BorderColor(Color::BLACK),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(build_button(&asset_server))
                        .with_children(|parent| {
                            parent.spawn(build_text("PLAY", &asset_server));
                        })
                        .insert(Button(ButtonType::Start));

                    parent
                        .spawn(build_button(&asset_server))
                        .with_children(|parent| {
                            parent.spawn(build_text("STOP", &asset_server));
                        })
                        .insert(Button(ButtonType::Stop));

                    parent
                        .spawn(build_button(&asset_server))
                        .with_children(|parent| {
                            parent.spawn(build_text("EXIT", &asset_server));
                        })
                        .insert(Button(ButtonType::Exit));
                });
        });
}

fn build_button(asset_server: &Res<AssetServer>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(50.0),
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: BackgroundColor(NORMAL_BUTTON),
        image: UiImage::new(asset_server.load("sprites/button.png")),
        ..default()
    }
}

fn build_text(value: &str, asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle {
        text: Text::from_section(
            value,
            TextStyle {
                font: asset_server.load("fonts/Symtext.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        ),
        ..default()
    }
}
