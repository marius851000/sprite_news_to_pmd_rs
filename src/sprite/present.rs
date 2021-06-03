use crate::{KindChange, output::{ChangeIllustration, ChangeIllustrationImage}, sprite::SpriteSheet};

pub fn present_sprites_images(changes: KindChange<SpriteSheet>, slug: &str) -> Vec<ChangeIllustration> {
    let mut illustrations = Vec::new();

    if !changes.added.is_empty() {
        for change in &changes.added {
            illustrations.push(ChangeIllustration {
                filename_tip: slug.into(),
                title: format!("new {} animation", change.0),
                image: ChangeIllustrationImage::Sprite(change.1.clone().scale(4)),

            })
        }
    }

    if changes.have_change() {
        for change in &changes.changed {
            illustrations.push(ChangeIllustration {
                filename_tip: slug.into(),
                title: format!("changed {} animation (old -> new)", change.0),
                image: ChangeIllustrationImage::Sprite(change.1.side_by_side(&change.2).scale(4))
            })
        }
    }

    if !changes.removed.is_empty() {
        for change in &changes.removed {
            illustrations.push(ChangeIllustration {
                filename_tip: slug.into(),
                title: format!("deleted {} animation", change.0),
                image: ChangeIllustrationImage::Sprite(change.1.clone().scale(4)),

            })
        }
    }

    illustrations
}