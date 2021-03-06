use super::*;

use rand::Rng;
use rand::seq::SliceRandom;

#[derive(Default)]
pub struct Random;
impl RenderPass for Random {
    fn expand<'a>(&self,
        atom: &Atom<'a>,
        objtree: &'a ObjectTree,
        output: &mut Vec<Atom<'a>>,
    ) -> bool {
        let mut rng = ::rand::thread_rng();

        if atom.istype("/obj/machinery/vending/snack/random/") {
            if let Some(root) = objtree.find("/obj/machinery/vending/snack") {
                let mut machines = Vec::new();
                for child in root.children() {
                    if child.name != "random" {
                        machines.push(child.get());
                    }
                }
                if let Some(&replacement) = machines.choose(&mut rng) {
                    output.push(Atom::from(replacement));
                    return false;  // consumed
                }
            }
        } else if atom.istype("/obj/machinery/vending/cola/random/") {
            if let Some(root) = objtree.find("/obj/machinery/vending/cola") {
                let mut machines = Vec::new();
                for child in root.children() {
                    if child.name != "random" {
                        machines.push(child.get());
                    }
                }
                if let Some(&replacement) = machines.choose(&mut rng) {
                    output.push(Atom::from(replacement));
                    return false;  // consumed
                }
            }
        } else if atom.istype("/obj/item/bedsheet/random/") {
            if let Some(root) = objtree.find("/obj/item/bedsheet") {
                let mut sheets = vec![root.get()];  // basic bedsheet is included
                for child in root.children() {
                    if child.name != "random" {
                        sheets.push(child.get());
                    }
                }
                if let Some(&replacement) = sheets.choose(&mut rng) {
                    output.push(Atom::from(replacement));
                    return false;  // consumed
                }
            }
        }
        true
    }

    fn adjust_sprite<'a>(&self,
        atom: &Atom<'a>,
        sprite: &mut Sprite<'a>,
        objtree: &'a ObjectTree,
        bump: &'a bumpalo::Bump,
    ) {
        let mut rng = ::rand::thread_rng();

        const CONTRABAND_POSTERS: u32 = 44;
        const LEGIT_POSTERS: u32 = 35;

        if atom.istype("/obj/structure/sign/poster/contraband/random/") {
            sprite.icon_state = bumpalo::format!(in bump, "poster{}", rng.gen_range(1, 1 + CONTRABAND_POSTERS)).into_bump_str();
        } else if atom.istype("/obj/structure/sign/poster/official/random/") {
            sprite.icon_state = bumpalo::format!(in bump, "poster{}_legit", rng.gen_range(1, 1 + LEGIT_POSTERS)).into_bump_str();
        } else if atom.istype("/obj/structure/sign/poster/random/") {
            let i = 1 + rng.gen_range(0, CONTRABAND_POSTERS + LEGIT_POSTERS);
            if i <= CONTRABAND_POSTERS {
                sprite.icon_state = bumpalo::format!(in bump, "poster{}", i).into_bump_str();
            } else {
                sprite.icon_state = bumpalo::format!(in bump, "poster{}_legit", i - CONTRABAND_POSTERS).into_bump_str();
            }
        } else if atom.istype("/obj/item/twohanded/required/kirbyplants/random/") {
            sprite.icon = "icons/obj/flora/plants.dmi";
            let random = rng.gen_range(0, 26);
            if random == 0 {
                sprite.icon_state = "applebush";
            } else {
                sprite.icon_state = bumpalo::format!(in bump, "plant-{:02}", random).into_bump_str();
            }
        } else if atom.istype("/obj/structure/sign/barsign/") {
            if let Some(root) = objtree.find("/datum/barsign") {
                let mut signs = Vec::new();
                for child in root.children() {
                    if let Some(v) = child.vars.get("hidden") {
                        if !v.value.constant.as_ref().map_or(false, |c| c.to_bool()) {
                            continue;
                        }
                    }
                    if let Some(icon) = child.get().vars.get("icon") {
                        if let Some(c) = icon.value.constant.as_ref() {
                            if let Some(text) = c.as_str() {
                                signs.push(text);
                            }
                        }
                    }
                }
                if let Some(c) = signs.choose(&mut rng) {
                    sprite.icon_state = c;
                }
            }
        }

        if atom.istype("/obj/item/lipstick/random/") {
            sprite.icon_state = "lipstick";
            // random color is not outwardly visible
        } else if atom.istype("/obj/item/device/tape/random/") {
            sprite.icon_state = [
                "tape_white",
                "tape_blue",
                "tape_red",
                "tape_yellow",
                "tape_purple",
            ].choose(&mut rng).unwrap();
        }
    }
}
