// Called once per game tick. The mod is empty: add your logic here.
fn gravity_changer_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::gravity_changer as m;
    // Turned off in Options > Mods: nothing until the next build leaves it out.
    if !game.native_on(m::ID) {
        return;
    }
}
