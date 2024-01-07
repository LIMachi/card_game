use crate::game::events::{BuyFrom, CardActions, GameEvent, GameEvents};
use crate::players::LocalPlayer;
use crate::prelude::*;
use crate::utils::ray_caster::RayCastHit;

pub fn card_click_handler(
    mut commands: Commands,
    caster: Query<&RayCaster>,
    buttons: Res<Input<MouseButton>>,
    cards: Query<(&CardOwners, &Stacks, &CardIndex, &CardVisibility)>,
    mut events: ResMut<GameEvent>,
    local_player: Res<LocalPlayer>,
) {
    if buttons.just_pressed(MouseButton::Left) {
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
            if let Ok((&owner, &stack, &index, &visibility)) = cards.get(entity) {
                if stack == Stacks::Hand && owner == CardOwners::Player(local_player.0) {
                    events.push(GameEvents::PlayCard(index.0 as u8));
                }
                if stack == Stacks::Bases {
                    if let CardOwners::Player(p) = owner {
                        if p == local_player.0 {
                            //FIXME: need validation
                            events.push(GameEvents::ActivateCard {
                                base: true,
                                index: index.0 as u32,
                                action: CardActions::Primary,
                            });
                        } else {
                            //FIXME: need validation
                            events.push(GameEvents::Attack {
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
