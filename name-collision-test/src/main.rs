
use bevy::{asset::Assets, ecs::system::ResMut, math::Vec4};
use bevy_hanabi::{ColorOverLifetimeModifier, EffectAsset, Gradient, Module, ParticleEffect, SpawnerSettings};

pub fn spawn_particles(effects: &mut ResMut<Assets<EffectAsset>>) -> ParticleEffect {
    let module = Module::default();
    let gradient: Gradient<Vec4> = Gradient::from_keys([(0.0, Vec4::ZERO)]);
    let effect = effects.add(
        EffectAsset::new(100, SpawnerSettings::rate(5.0.into()), module).render(ColorOverLifetimeModifier::new(gradient))
    );
    
    ParticleEffect::new(effect)
}

fn main() {
    println!("Hello, world!");
}
