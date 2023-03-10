use bevy::app::AppExit;

use crate::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::MainMenu, setup.label(MenuSystems::Setup))
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::MainMenu)
                    .with_system(butt_interact_visual)
                    // our menu button handlers
                    .with_system(butt_quit.run_if(on_butt_interact::<QuitButt>))
                    .with_system(butt_game.run_if(on_butt_interact::<EnterButt>))
                    .into(),
            )
            .add_exit_system(
                AppState::MainMenu,
                cleanup_on::<Menu>.label(MenuSystems::Cleanup),
            );
    }
}

// Marker Component for main menu
#[derive(Debug, Component)]
struct Menu;

#[derive(Debug, Component)]
struct Selected;

/// Marker for the "Quit" button
#[derive(Component)]
struct QuitButt;

/// Marker for the "New Adventure" button
#[derive(Component)]
struct EnterButt;

#[allow(unused)]
#[derive(Debug, SystemLabel, PartialEq, Eq, Clone, Hash)]
enum MenuSystems {
    Setup,
    Draw,
    SelctionSet,
    Cleanup,
}

fn setup(mut cmd: Commands, ui_font: Res<Fonts>) {
    let menu = cmd
        .spawn((
            Menu,
            NodeBundle {
                background_color: BackgroundColor(Color::rgb(0.5, 0.5, 0.5)),
                style: Style {
                    size: Size::new(Val::Auto, Val::Auto),
                    margin: UiRect::all(Val::Auto),
                    align_self: AlignSelf::Center,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let butt_style = Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(8.0)),
        margin: UiRect::all(Val::Px(4.0)),
        flex_grow: 1.0,
        ..default()
    };

    let butt_text_style = TextStyle {
        font: ui_font.ibm_bios.clone(),
        color: Color::BLACK,
        font_size: 24.0,
        ..default()
    };

    let butt_start = cmd
        .spawn((
            ButtonBundle {
                style: butt_style.clone(),
                ..default()
            },
            EnterButt,
        ))
        .with_children(|btn| {
            btn.spawn(TextBundle {
                text: Text::from_section("New Adventure", butt_text_style.clone()),
                ..default()
            });
        })
        .id();

    let butt_quit = cmd
        .spawn((
            ButtonBundle {
                style: butt_style.clone(),
                ..default()
            },
            QuitButt,
        ))
        .with_children(|btn| {
            btn.spawn(TextBundle {
                text: Text::from_section("Quit", butt_text_style.clone()),
                ..default()
            });
        })
        .id();

    cmd.entity(menu).push_children(&[butt_start, butt_quit]);
}

/// Handler for the Enter Game button
fn butt_game(mut cmd: Commands) {
    // queue state transition
    cmd.insert_resource(NextState(AppState::InGame));
}

/// Handler for the Quit button
fn butt_quit(mut ev: EventWriter<AppExit>) {
    ev.send(AppExit);
}

/// Change button color on interaction
fn butt_interact_visual(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *color = BackgroundColor(Color::rgb(0.75, 0.75, 0.75));
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::rgb(0.8, 0.8, 0.8));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::rgb(1.0, 1.0, 1.0));
            }
        }
    }
}

/// Condition to help with handling multiple buttons
///
/// Returns true when a button identified by a given component is clicked.
fn on_butt_interact<B: Component>(
    query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<B>)>,
) -> bool {
    for interaction in query.iter() {
        if *interaction == Interaction::Clicked {
            return true;
        }
    }

    false
}
