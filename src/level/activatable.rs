use bevy::prelude::*;
use bevy_ecs_ldtk::*;
use prelude::LdtkFields;

use crate::light::sensor::update_light_sensors;

pub struct ActivatablePlugin;

impl Plugin for ActivatablePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActivatableCache>()
            .add_event::<GroupTriggeredEvent>()
            .add_systems(
                Update,
                (
                    setup_activatables,
                    update_activatables.after(update_light_sensors),
                )
                    .chain(),
            );
    }
}

#[derive(Default, Component)]
pub struct Activatable {
    pub id: i32,
    pub init_active: bool,
}

#[derive(Default, Component)]
#[component(storage = "SparseSet")]
pub struct Activated;

#[derive(Default, Resource)]
pub struct ActivatableCache {
    table: std::collections::HashMap<i32, Vec<Entity>>,
}

#[derive(Event)]
pub struct GroupTriggeredEvent {
    pub id: i32,
}

pub fn update_activatables(
    mut commands: Commands,
    mut ev_group_triggered: EventReader<GroupTriggeredEvent>,
    activatable_cache: Res<ActivatableCache>,
    q_activated: Query<&Activated>,
) {
    for event in ev_group_triggered.read() {
        let id = event.id;
        if !activatable_cache.table.contains_key(&id) {
            continue;
        }

        for &entity in activatable_cache.table[&id].iter() {
            if let Ok(_) = q_activated.get(entity) {
                commands.entity(entity).remove::<Activated>();
            } else {
                commands.entity(entity).insert(Activated);
            }
        }
    }
}

pub fn init_activatable(entity_instance: &EntityInstance) -> Activatable {
    let id = entity_instance
        .get_int_field("id")
        .expect("Activatable id should exist and be an integer");

    let active = entity_instance
        .get_bool_field("active")
        .expect("Activatable active status should exist and be a boolean");

    Activatable {
        id: *id,
        init_active: *active,
    }
}

pub fn setup_activatables(
    mut commands: Commands,
    mut activatable_cache: ResMut<ActivatableCache>,
    q_activatable: Query<(Entity, &Activatable), Added<Activatable>>,
) {
    for (entity, activatable) in q_activatable.iter() {
        if activatable.init_active {
            commands.entity(entity).insert(Activated);
        }

        activatable_cache
            .table
            .entry(activatable.id)
            .or_default()
            .push(entity);
    }
}
