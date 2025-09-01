use bevy::prelude::*;

use crate::components::BallCounterText;

pub fn update_ball_counter(
    ball_query: Query<&crate::components::BouncyBall>,
    mut text_query: Query<&mut Text, With<BallCounterText>>,
) {
    if let Ok(mut text) = text_query.single_mut() {
        let count = ball_query.iter().count();
        text.0 = format!("Balls: {}", count);
    }
}

pub fn spawn_ui(commands: &mut Commands, font: Handle<Font>) {
    // counter top-left
    commands.spawn((
        Node { position_type: PositionType::Absolute, top: Val::Px(8.0), left: Val::Px(8.0), padding: UiRect::axes(Val::Px(6.0), Val::Px(4.0)), ..default() },
        BackgroundColor(Color::srgba(0.0,0.0,0.0,0.55)),
    )).with_children(|parent| {
        parent.spawn((
            Text::new("Balls: 1"),
            TextFont { font: font.clone(), font_size: 22.0, ..default() },
            TextColor(Color::srgb(1.0, 1.0, 0.8)),
            BallCounterText,
        ));
    });

    // reset hint bottom-right
    commands.spawn((
        Node { position_type: PositionType::Absolute, bottom: Val::Px(6.0), right: Val::Px(6.0), padding: UiRect::axes(Val::Px(5.0), Val::Px(3.0)), ..default() },
        BackgroundColor(Color::srgba(0.0,0.0,0.0,0.4)),
    )).with_children(|parent| {
        parent.spawn((
            Text::new("Press R to reset"),
            TextFont { font, font_size: 16.0, ..default() },
            TextColor(Color::srgb(0.85,0.85,0.85)),
        ));
    });
}
