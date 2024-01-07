use crate::prelude::*;

pub fn reset_actions_when_leaving_play(
    mut cards: Query<
        (
            Option<&mut OnPlay>,
            Option<&mut OnScrap>,
            Option<&mut ComboBlob>,
            Option<&mut ComboMachineCult>,
            Option<&mut ComboTradeFederation>,
            Option<&mut ComboStarEmpire>,
        ),
        Or<(Added<DiscardPile>, Added<Scrapyard>)>,
    >,
) {
    for (o0, o1, o2, o3, o4, o5) in cards.iter_mut() {
        if let Some(mut o) = o0 {
            o.1 = false;
        }
        if let Some(mut o) = o1 {
            o.1 = false;
        }
        if let Some(mut o) = o2 {
            o.1 = false;
        }
        if let Some(mut o) = o3 {
            o.1 = false;
        }
        if let Some(mut o) = o4 {
            o.1 = false;
        }
        if let Some(mut o) = o5 {
            o.1 = false;
        }
    }
}

pub struct ListenersPlugin;

impl Plugin for ListenersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, reset_actions_when_leaving_play);
    }
}
