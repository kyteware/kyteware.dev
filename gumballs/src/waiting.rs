use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{js_bindings, AvailableBall, DroppingBall, VisState};

pub fn waiting_plugin(app: &mut App) {
    app.add_observer(drop_ball);
}

fn drop_ball(_: On<js_bindings::GumballDrop>, mut commands: Commands, query: Query<(Entity, &Transform), With<AvailableBall>>, mut next_state: ResMut<NextState<VisState>>) {
    let lowest = query.iter().min_by_key(|t| (t.1.translation.y * 10000.) as i64);

    if let Some((lowest_entity, _)) = lowest {
        commands.entity(lowest_entity)
            .remove::<AvailableBall>().insert((DroppingBall::default(), RigidBodyDisabled));
    }

    *next_state = NextState::Pending(VisState::Dropping);
}
