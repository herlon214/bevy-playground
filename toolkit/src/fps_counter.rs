use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

pub struct FpsCounterPlugin;

impl Plugin for FpsCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin);
        app.add_systems(Startup, setup);
        app.add_systems(Update, update_fps);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Text::new("FPS: "))
        .with_child((TextSpan::default(), FpsText));
}

fn update_fps(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut TextSpan, With<FpsText>>) {
    for mut span in query.iter_mut() {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                **span = format!("{value:.2}");
            }
        }
    }
}
