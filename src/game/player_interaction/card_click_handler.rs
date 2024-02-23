use crate::cards::assets::Card;
use crate::cards::components::Focused;
use crate::cards::transition::{ResetFocus, StartFocus};
use crate::game::events::{BuyFrom, CardActions, GameEvent, GameEvents};
use crate::game::routines::card_action::{Selectable, Selected};
use crate::game::routines::RoutineManager;
use crate::players::{LocalPlayer, Player, PlayerTurnTracker};
use crate::prelude::*;
use crate::states::turn::TurnStates;
use crate::utils::ray_caster::RayCastHit;
use bevy_rapier3d::prelude::DebugRenderContext;

//given an action count and index, return the hitbox coresponding to this action
pub fn check_area(count: u8, ship: bool, index: u8) -> Rect {
    if count == 0 {
        return Rect::default();
    }
    if ship {
        if count == 1 {
            //full block
            return Rect::new(0.07, 0.055, 0.93, 0.315);
        }
        if index + 1 == count {
            //bottom block
            Rect::new(0.07, 0.055, 0.93, 0.14)
        } else if index == 0 {
            if count == 2 {
                //big top block
                Rect::new(0.07, 0.14, 0.93, 0.315)
            } else {
                //small top block
                Rect::new(0.07, 0.23, 0.93, 0.315)
            }
        } else if index == 1 && count == 3 {
            //middle block
            Rect::new(0.07, 0.14, 0.93, 0.23)
        } else {
            Rect::default()
        }
    } else {
        if count == 1 {
            //full block
            Rect::new(0.69, 0.04, 0.94, 0.96)
        } else if index == 0 {
            //top block
            Rect::new(0.69, 0.04, 0.815, 0.96)
        } else {
            if count == 3 {
                //smal bottom blocks
                if index == 1 {
                    //left block
                    Rect::new(0.815, 0.04, 0.94, 0.5)
                } else if index == 2 {
                    //right block
                    Rect::new(0.815, 0.5, 0.94, 0.96)
                } else {
                    Rect::default()
                }
            } else {
                //big bottom block
                Rect::new(0.815, 0.04, 0.94, 0.96)
            }
        }
    }
}

pub fn draw_hit_box(gizmos: &mut Gizmos, card: &GlobalTransform, hit_box: Rect, color: Color) {
    let x_offset = 0.005;
    let y_offset = 0.002;
    let affine = card.affine();
    let top_left = affine.transform_point3(Vec3::new(
        CARD_WIDTH / 2.,
        CARD_DEPTH / 1.5,
        -CARD_HEIGHT / 2.,
    ));
    let top_right = affine.transform_point3(Vec3::new(
        -CARD_WIDTH / 2.,
        CARD_DEPTH / 1.5,
        -CARD_HEIGHT / 2.,
    ));
    let bottom_left = affine.transform_point3(Vec3::new(
        CARD_WIDTH / 2.,
        CARD_DEPTH / 1.5,
        CARD_HEIGHT / 2.,
    ));
    let bottom_right = affine.transform_point3(Vec3::new(
        -CARD_WIDTH / 2.,
        CARD_DEPTH / 1.5,
        CARD_HEIGHT / 2.,
    ));
    let top_min_x = top_left.lerp(top_right, hit_box.min.x + x_offset);
    let bottom_min_x = bottom_left.lerp(bottom_right, hit_box.min.x + x_offset);
    let top_max_x = top_left.lerp(top_right, hit_box.max.x - x_offset);
    let bottom_max_x = bottom_left.lerp(bottom_right, hit_box.max.x - x_offset);
    let end = top_min_x.lerp(bottom_min_x, hit_box.min.y + y_offset);
    let lines = [
        end,
        top_min_x.lerp(bottom_min_x, hit_box.max.y - y_offset),
        top_max_x.lerp(bottom_max_x, hit_box.max.y - y_offset),
        top_max_x.lerp(bottom_max_x, hit_box.min.y + y_offset),
        end,
    ];
    gizmos.linestrip(lines, color);
}

pub fn card_hover(
    mut caster: Query<&mut RayCaster>,
    mut gizmos: Gizmos,
    cards: Query<
        (
            &crate::cards::actions::CardActions,
            Option<&Ship>,
            &GlobalTransform,
            &CardOwners,
        ),
        Or<(With<UsedCards>, With<Bases>)>,
    >,
    turn: Res<State<TurnStates>>,
    player_0_tracker: Query<&PlayerTurnTracker, With<Player<0>>>,
    player_1_tracker: Query<&PlayerTurnTracker, With<Player<1>>>,
) {
    if let TurnStates::PlayerTurn(playing) = turn.get() {
        if let Ok(tracker) = if *playing == 0 {
            player_0_tracker.get_single()
        } else {
            player_1_tracker.get_single()
        } {
            if let Ok(mut caster) = caster.get_single_mut() {
                if let Some(RayCastHit {
                    entity,
                    toi,
                    point,
                    normal,
                    relative,
                    back,
                    percent,
                    action,
                }) = caster.hit
                {
                    if let Ok((actions, ship, transform, owner)) = cards.get(entity) {
                        if *owner == CardOwners::Player(*playing) {
                            let len = actions.len();
                            for i in 0..len {
                                if !(i == 0 && ship.is_some()) && actions.is_action_real(i) {
                                    let rect = check_area(len, ship.is_some(), i);
                                    draw_hit_box(
                                        &mut gizmos,
                                        transform,
                                        rect,
                                        if actions.is_action_available(i, tracker) {
                                            if percent.x > rect.min.x
                                                && percent.x < rect.max.x
                                                && percent.y > rect.min.y
                                                && percent.y < rect.max.y
                                            {
                                                caster.hit.as_mut().unwrap().action = Some(i);
                                                Color::GREEN
                                            } else {
                                                Color::ORANGE
                                            }
                                        } else {
                                            Color::RED
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn focus_card_handler(
    mut commands: Commands,
    caster: Query<&RayCaster>,
    buttons: Res<ButtonInput<MouseButton>>,
    focused: Query<Entity, With<Focused>>,
    cards: Query<(Entity, &CardVisibility), With<CardVisibility>>,
) {
    if buttons.just_pressed(MouseButton::Right) {
        if let Some(RayCastHit { entity, .. }) = caster.get_single().ok().and_then(|r| r.hit) {
            if let Ok((card, visibility)) = cards.get(entity) {
                if *visibility == CardVisibility::Visible
                    && focused.get_single().map_or(true, |f| f != card)
                {
                    commands.entity(card).insert(StartFocus { length: 0.5 });
                }
            }
        }
        if let Ok(focused) = focused.get_single() {
            commands.entity(focused).insert(ResetFocus { length: 0.5 });
        }
    }
}

pub fn selection_feedback(
    mut gizmos: Gizmos,
    selected: Query<Entity, With<Selected>>,
    selectable: Query<(Entity, &GlobalTransform), With<Selectable>>,
) {
    for (card, tr) in selectable.iter() {
        draw_hit_box(
            &mut gizmos,
            tr,
            Rect::new(0., 0., 1., 1.),
            if selected.contains(card) {
                Color::GREEN
            } else {
                Color::ORANGE
            },
        );
    }
}

pub fn selection_click(
    caster: Query<&RayCaster>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    selectable: Query<Entity, With<Selectable>>,
    selected: Query<Entity, With<Selected>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(RayCastHit { entity, .. }) = caster.get_single().ok().and_then(|r| r.hit) {
            if selectable.contains(entity) {
                if selected.contains(entity) {
                    commands.entity(entity).remove::<Selected>();
                } else {
                    commands.entity(entity).insert(Selected);
                }
            }
        }
    }
}

pub fn card_click_handler(
    caster: Query<&RayCaster>,
    buttons: Res<ButtonInput<MouseButton>>,
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
    routines: Res<RoutineManager>,
) {
    if routines.is_empty() {
        //prevent click to trigger anything while a routine is running (prevent weird behavior of clicking on cards while they travel across the screen/clicking on market while selecting an option)
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
                    action,
                }) = caster.get_single().ok().and_then(|r| r.hit)
                {
                    if *playing == local_player.0 || debug.enabled {
                        //disable most actions if it's not our turn
                        if let Ok((&owner, &stack, &index, &visibility, ship)) = cards.get(entity) {
                            if stack == Stacks::Hand && owner == CardOwners::Player(*playing) {
                                events.push(GameEvents::PlayCard(index.0 as u8));
                            }
                            if stack == Stacks::UsedCards && owner == CardOwners::Player(*playing) {
                                if let Some(action) = action {
                                    events.push(GameEvents::ActivateCard {
                                        base: false,
                                        index: index.0 as u32,
                                        action,
                                    });
                                }
                            }
                            if stack == Stacks::Bases {
                                if let CardOwners::Player(p) = owner {
                                    if p == *playing {
                                        if let Some(action) = action {
                                            events.push(GameEvents::ActivateCard {
                                                base: true,
                                                index: index.0 as u32,
                                                action,
                                            });
                                        }
                                    } else {
                                        events.push(GameEvents::Attack {
                                            as_much_as_possible: true,
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
}
