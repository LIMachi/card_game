use crate::game::events::{BuyFrom, CardActions, GameEvent, GameEvents};
use crate::players::LocalPlayer;
use crate::prelude::*;
use crate::states::turn::TurnStates;
use crate::utils::ray_caster::RayCastHit;
use bevy_rapier3d::prelude::DebugRenderContext;

pub fn card_click_handler(
    mut commands: Commands,
    caster: Query<&RayCaster>,
    buttons: Res<Input<MouseButton>>,
    cards: Query<(
        &CardOwners,
        &Stacks,
        &CardIndex,
        &CardVisibility,
        Option<&Ship>,
    )>,
    mut events: ResMut<GameEvent>,
    local_player: Res<LocalPlayer>,
    turn: Res<State<TurnStates>>,
    debug: Res<DebugRenderContext>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let TurnStates::PlayerTurn(playing) = turn.get() {
            if let Some(RayCastHit {
                entity,
                toi,
                point,
                normal,
                relative,
                back,
                percent,
            }) = caster.get_single().ok().and_then(|r| r.hit)
            {
                if *playing == local_player.0 || debug.enabled {
                    //disable most actions if it's not our turn
                    if let Ok((&owner, &stack, &index, &visibility, ship)) = cards.get(entity) {
                        if stack == Stacks::Hand && owner == CardOwners::Player(*playing) {
                            events.push(GameEvents::PlayCard(index.0 as u8));
                            if ship.is_some() {
                                events.push(GameEvents::ActivateCard {
                                    base: false,
                                    index: index.0 as u32,
                                    action: CardActions::Primary,
                                });
                            }
                        }
                        if stack == Stacks::Bases {
                            if let CardOwners::Player(p) = owner {
                                if p == *playing {
                                    //FIXME: need validation
                                    events.push(GameEvents::ActivateCard {
                                        base: true,
                                        index: index.0 as u32,
                                        action: CardActions::Primary,
                                    });
                                } else {
                                    //FIXME: need validation
                                    events.push(GameEvents::Attack {
                                        as_much_as_possible: false,
                                        player: p,
                                        base_index: Some(index.0 as u32),
                                    });
                                }
                            }
                        }
                        if stack == Stacks::MarketRow {
                            //FIXME: need validation
                            events.push(GameEvents::BuyCard(BuyFrom::Market(index.0 as u8)));
                        }
                        if stack == Stacks::JokerDeck {
                            //FIXME: need validation
                            events.push(GameEvents::BuyCard(BuyFrom::Joker));
                        }
                    }
                }
            }
        }
    }
}
