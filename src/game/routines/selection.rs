use crate::game::routines::card_action::{Selectable, Selected, Selection};
use crate::game::routines::{RoutineManager, Routines, SelectionFilter};
use crate::game::GameStates;
use crate::prelude::*;
use crate::ui::SelectionValidationButton;

pub fn selection(
    mut commands: Commands,
    mut manager: ResMut<RoutineManager>,
    mut selection: ResMut<Selection>,
    selectable: Query<Entity, With<Selectable>>,
    selected: Query<Entity, With<Selected>>,
    mut game_state: ResMut<NextState<GameStates>>,
    all_cards: Query<(Entity, &CardOwners, &Stacks, &CardCost, &CardKinds)>,
    mut validation_button: Query<&mut Visibility, With<SelectionValidationButton>>,
) {
    if let Some(Routines::Selection {
        filters,
        min,
        max,
        running,
    }) = manager.routine_mut()
    {
        if !*running {
            //clear the Selection resource
            selection.finished = false;
            selection.cards = Vec::new();
            selection.min = *min;
            selection.max = *max;
            //get all valid cards and insert the Selectable component
            for (card, &owner, &stack, &CardCost(cost), &kind) in all_cards.iter() {
                for SelectionFilter {
                    stacks,
                    owners,
                    kinds,
                    min_cost,
                    max_cost,
                } in filters.iter()
                {
                    if owners.contains(&owner)
                        && stacks.contains(&stack)
                        && kind.in_mask(*kinds)
                        && cost >= *min_cost as i32
                        && cost <= *max_cost as i32
                    {
                        commands.entity(card).insert(Selectable);
                    }
                }
            }
            //make the validation button visible
            *validation_button.get_single_mut().unwrap() = Visibility::Visible;
            //change state to selection state
            game_state.set(GameStates::SelectionInput);
            *running = true;
        }
        if !selection.finished {
            return;
        } else {
            for card in selectable.iter() {
                commands.entity(card).remove::<Selectable>();
            }
            for card in selected.iter() {
                selection.cards.push(card);
                commands.entity(card).remove::<Selected>();
            }
            game_state.set(GameStates::MainLoop);
            *validation_button.get_single_mut().unwrap() = Visibility::Hidden;
            manager.finish();
        }
    }
}
