use bevy :: prelude :: *;

pub struct AnimationEntityLinkPlugin;

impl Plugin for AnimationEntityLinkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, link_animations);
    }
}

#[derive(Component)]
pub struct AnimationEntityLinkPlugin(pub Entity);

// Add this component to stop the root parent search and return
#[derive(Component)]
pub struct AnimationEntityLinkTrap;

fn get_top_parent(
    mut curr_entity: Entity,
    parent_query: &Query<(&Parent, Option<&AnimationEntityLinkTrap>)>,
) -> Entity {
    loop {
        if let Ok(parent, trap) = parent_query.get(curr_entity) {
            match trap {
                Some(_) => break,
                None => curr_entity = parent.get(),
            }
        } else {
            break;
        }
    }
    curr_entity
}

pub fn link_animations(
    player_query: Query<Entity, Added<AnimationPlayer>>,
    parent_query: Query<(&Parent, Option<&AnimationEntityLinkTrap>)>,
    mut commands: Commands
) {
    for entity in player_query.iter() {
        let top_entity = get_top_parent(entity, &parent_query);
        if animations_entity_link_query.get(top_entity).is_ok() {
            warn!("Problem with multiple animationplayer for the same top parent")
        } else {
            commands
            .entity(entity)
            .insert(AnimationEntityLink(entity.clone()));
        }
    }
}