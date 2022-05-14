use bevy::prelude::{Mut, Transform};

pub trait UpdateMutTransform {
    fn update(&mut self, transform: Transform);
}

impl UpdateMutTransform for Mut<'_, Transform> {
    fn update(&mut self, transform: Transform) {
        self.translation = transform.translation;
        self.rotation = transform.rotation;
        self.scale = transform.scale;
    }
}
