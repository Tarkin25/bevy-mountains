use std::sync::Arc;

use bevy::prelude::*;
use noise::{MultiFractal, *};

use crate::mountain::terrain_noise::cache::SyncCache;

mod cache;
mod phantom_seedable;
mod worley;

pub struct NoisePlugin;

impl Plugin for NoisePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, insert_terrain_generator);
    }
}

#[derive(Clone, Resource)]
pub struct TerrainGenerator(Arc<dyn NoiseFn<f64, 2> + Send + Sync>);

fn insert_terrain_generator(mut commands: Commands) {
    //let noise = create_noise();

    // Mountains & Valleys
    let mountains = RidgedMulti::<Perlin>::default().set_octaves(4).set_frequency(0.5).set_persistence(0.5);
    let mountains = ScaleBias::new(mountains).set_scale(0.5);
    let river_valley = RidgedMulti::<Perlin>::default().set_octaves(1).set_frequency(0.2);
    let river_valley = ScaleBias::new(river_valley).set_scale(2.0);
    
    // Hills
    let hills = Fbm::<Perlin>::default().set_octaves(5).set_frequency(0.5);
    let hills = ScaleBias::new(hills).set_scale(0.25).set_bias(-1.0);
    
    let noise = Blend::new(mountains, hills, river_valley); // where river valley is low, mountain is strong, where river valley is high, hills is strong

    let height = 100.0;
    let noise = ScaleBias::new(noise).set_scale(height / 2.0); // Amplitude on y-axis
    let scale = 1.0 / height;
    let noise = ScalePoint::new(noise).set_all_scales(scale, scale, scale, scale); // Scale applied to coordinates before sampling noise

    commands.insert_resource(TerrainGenerator(Arc::new(noise)));
}

#[allow(unused)]
fn create_noise() -> impl NoiseFn<f64, 2> {
    /// Planet seed. Change this to generate a different planet.
    const CURRENT_SEED: u32 = 0;

    /// Frequency of the planet's continents. Higher frequency produces
    /// smaller, more numerous continents. This value is measured in radians.
    const CONTINENT_FREQUENCY: f64 = 1.0;

    /// Lacunarity of the planet's continents. Changing this value produces
    /// slightly different continents. For the best results, this value should
    /// be random, but close to 2.0.
    const CONTINENT_LACUNARITY: f64 = 2.208984375;

    /// Lacunarity of the planet's mountains. Changing the value produces
    /// slightly different mountains. For the best results, this value should
    /// be random, but close to 2.0.
    const MOUNTAIN_LACUNARITY: f64 = 2.142578125;

    /// Lacunarity of the planet's hills. Changing this value produces
    /// slightly different hills. For the best results, this value should be
    /// random, but close to 2.0.
    const HILLS_LACUNARITY: f64 = 2.162109375;

    /// Lacunarity of the planet's plains. Changing this value produces
    /// slightly different plains. For the best results, this value should be
    /// random, but close to 2.0.
    const PLAINS_LACUNARITY: f64 = 2.314453125;

    /// Lacunarity of the planet's badlands. Changing this value produces
    /// slightly different badlands. For the best results, this value should
    /// be random, but close to 2.0.
    const BADLANDS_LACUNARITY: f64 = 2.212890625;

    /// Specifies the "twistiness" of the mountains.
    const MOUNTAINS_TWIST: f64 = 1.0;

    /// Specifies the "twistiness" of the hills.
    const HILLS_TWIST: f64 = 1.0;

    /// Specifies the "twistiness" of the badlands.
    const BADLANDS_TWIST: f64 = 1.0;

    /// Specifies the planet's sea level. This value must be between -1.0
    /// (minimum planet elevation) and +1.0 (maximum planet elevation).
    const SEA_LEVEL: f64 = 0.0;

    /// Specifies the level on the planet in which continental shelves appear.
    /// This value must be between -1.0 (minimum planet elevation) and +1.0
    /// (maximum planet elevation), and must be less than `SEA_LEVEL`.
    const SHELF_LEVEL: f64 = -0.375;

    /// Determines the amount of mountainous terrain that appears on the
    /// planet. Values range from 0.0 (no mountains) to 1.0 (all terrain is
    /// covered in mountains). Mountains terrain will overlap hilly terrain.
    /// Because the badlands terrain may overlap parts of the mountainous
    /// terrain, setting `MOUNTAINS_AMOUNT` to 1.0 may not completely cover the
    /// terrain in mountains.
    const MOUNTAINS_AMOUNT: f64 = 0.5;

    /// Determines the amount of hilly terrain that appears on the planet.
    /// Values range from 0.0 (no hills) to 1.0 (all terrain is covered in
    /// hills). This value must be less than `MOUNTAINS_AMOUNT`. Because the
    /// mountains terrain will overlap parts of the hilly terrain, and the
    /// badlands terrain may overlap parts of the hilly terrain, setting
    /// `HILLS_AMOUNT` to 1.0 may not completely cover the terrain in hills.
    const HILLS_AMOUNT: f64 = (1.0 + MOUNTAINS_AMOUNT) / 2.0;

    /// Determines the amount of badlands terrain that covers the planet.
    /// Values range from 0.0 (no badlands) to 1.0 (all terrain is covered in
    /// badlands). Badlands terrain will overlap any other type of terrain.
    const BADLANDS_AMOUNT: f64 = 0.3125;

    /// Offset to apply to the terrain type definition. Low values (< 1.0)
    /// cause the rough areas to appear only at high elevations. High values
    /// (> 2.0) cause the rough areas to appear at any elevation. The
    /// percentage of rough areas on the planet are independent of this value.
    const TERRAIN_OFFSET: f64 = 1.0;

    /// Specifies the amount of "glaciation" on the mountains. This value
    /// should be close to 1.0 and greater than 1.0.
    const MOUNTAIN_GLACIATION: f64 = 1.375;

    /// Scaling to apply to the base continent elevations, in planetary
    /// elevation units.
    const CONTINENT_HEIGHT_SCALE: f64 = (1.0 - SEA_LEVEL) / 4.0;

    /// Maximum depth of the rivers, in planetary elevation units.
    const RIVER_DEPTH: f64 = 0.0234375;

    // ////////////////////////////////////////////////////////////////////////
    // Function group: continent definition
    // ////////////////////////////////////////////////////////////////////////

    // ////////////////////////////////////////////////////////////////////////
    // Function subgroup: base continent definition (7 noise functions)
    //
    // This subgroup roughly defines the positions and base elevations of the
    // planet's continents.
    //
    // The "base elevation" is the elevation of the terrain before any terrain
    // features (mountains, hills, etc.) are placed on that terrain.
    //
    // -1.0 represents the lowest elevations and +1.0 represents the highest
    // elevations.
    //
    fn base_continent_def() -> impl NoiseFn<f64, 2> {
        // 1: [Continent module]: This FBM module generates the continents. This
        // noise function has a high number of octaves so that detail is visible at
        // high zoom levels.
        let continent = Fbm::<Perlin>::default()
            .set_frequency(CONTINENT_FREQUENCY)
            .set_octaves(14)
            .set_persistence(0.5)
            .set_lacunarity(CONTINENT_LACUNARITY);
        
        // 2: [Continent-with-ranges module]: Next, a curve module modifies the
        // output value from the continent module so that very high values appear
        // near sea level. This defines the positions of the mountain ranges.
        let continent_with_ranges = Curve::new(continent)
            .add_control_point(-2.0000 + SEA_LEVEL, -1.625 + SEA_LEVEL)
            .add_control_point(-1.0000 + SEA_LEVEL, -1.375 + SEA_LEVEL)
            .add_control_point(0.0000 + SEA_LEVEL, -0.375 + SEA_LEVEL)
            .add_control_point(0.0625 + SEA_LEVEL, 0.125 + SEA_LEVEL)
            .add_control_point(0.1250 + SEA_LEVEL, 0.250 + SEA_LEVEL)
            .add_control_point(0.2500 + SEA_LEVEL, 1.000 + SEA_LEVEL)
            .add_control_point(0.5000 + SEA_LEVEL, 0.250 + SEA_LEVEL)
            .add_control_point(0.7500 + SEA_LEVEL, 0.250 + SEA_LEVEL)
            .add_control_point(1.0000 + SEA_LEVEL, 0.500 + SEA_LEVEL)
            .add_control_point(2.0000 + SEA_LEVEL, 0.500 + SEA_LEVEL);
        
        // 3: [Carver module]: This higher-frequency BasicMulti module will be
        // used by subsequent noise functions to carve out chunks from the
        // mountain ranges within the continent-with-ranges module so that the
        // mountain ranges will not be completely impassible.
        let carver = Fbm::<Perlin>::new(CURRENT_SEED + 1)
            .set_frequency(CONTINENT_FREQUENCY * 4.34375)
            .set_persistence(0.5)
            .set_lacunarity(CONTINENT_LACUNARITY)
            .set_octaves(11);

        // 4: [Scaled-carver module]: This scale/bias module scales the output
        // value from the carver module such that it is usually near 1.0. This
        // is required for step 5.
        let scaled_carver = ScaleBias::new(carver)
            .set_scale(0.375)
            .set_bias(0.625);

        // 5: [Carved-continent module]: This minimum-value module carves out
        // chunks from the continent-with-ranges module. it does this by ensuring
        // that only the minimum of the output values from the scaled-carver
        // module and the continent-with-ranges module contributes to the output
        // value of this subgroup. Most of the time, the minimum value module will
        // select the output value from the continent-with-ranges module since the
        // output value from the scaled-carver is usually near 1.0. Occasionally,
        // the output from the scaled-carver module will be less than the output
        // value from the continent-with-ranges module, so in this case, the output
        // value from the scaled-carver module is selected.
        let carved_continent = Min::new(scaled_carver, continent_with_ranges);

        // 6: [Clamped-continent module]: Finally, a clamp module modifies the
        // carved continent module to ensure that the output value of this subgroup
        // is between -1.0 and 1.0.
        let base_continent_clamp = Clamp::new(carved_continent).set_bounds(-1.0, 1.0);

        // 7: [Base-continent-definition subgroup]: Caches the output value from
        // the clamped-continent module.
        let base_continent = SyncCache::new(base_continent_clamp);
        base_continent
    }

    // ////////////////////////////////////////////////////////////////////////
    // Function subgroup: continent definition (5 noise functions)
    //
    // This subgroup warps the output value from the base-continent-definition
    // subgroup, producing more realistic terrain.
    //
    // Warping the base continent definition produces lumpier terrain with
    // cliffs and rifts.
    //
    // -1.0 represents the lowest elevations and +1.0 represents the highest
    // elevations.
    //

    // 1: [Coarse-turbulence module]: This turbulence module warps the output
    // value from the base-continent-definition subgroup, adding some coarse
    // detail to it.
    let coarse_turbulence = Turbulence::<_, Perlin>::new(base_continent_def())
    .set_seed(CURRENT_SEED+10)
    .set_frequency(CONTINENT_FREQUENCY*15.25)
    .set_power(CONTINENT_FREQUENCY / 113.75)
    .set_roughness(13);

    // 2: [Intermediate-turbulence module]: This turbulence module warps the
    // output value from the coarse-turbulence module. This turbulence has a
    // higher frequency, but lower power, than the coarse-turbulence module,
    // adding some intermediate detail to it.
    let intermediate_turbulence = Turbulence::<_, Perlin>::new(coarse_turbulence)
    .set_seed(CURRENT_SEED+11)
    .set_frequency(CONTINENT_FREQUENCY*47.25)
    .set_power(CONTINENT_FREQUENCY/433.75)
    .set_roughness(12);

    // 3: [Warped-base-continent-definition module]: This turbulence module
    // warps the output value from the intermediate-turbulence module. This
    // turbulence has a higher frequency, but lower power, than the
    // intermediate-turbulence module, adding some fine detail to it.
    let warped_base_continent_definition = Turbulence::<_, Perlin>::new(intermediate_turbulence)
    .set_seed(CURRENT_SEED+12)
    .set_frequency(CONTINENT_FREQUENCY*95.25)
    .set_power(CONTINENT_FREQUENCY / 1019.75)
    .set_roughness(11);

    // 4: [Select-turbulence module]: At this stage, the turbulence is applied
    // to the entire base-continent-definition subgroup, producing some very
    // rugged, unrealistic coastlines.  This selector module selects the
    // output values from the (unwarped) base-continent-definition subgroup
    // and the warped-base-continent-definition module, based on the output
    // value from the (unwarped) base-continent-definition subgroup.  The
    // selection boundary is near sea level and has a relatively smooth
    // transition.  In effect, only the higher areas of the base-continent-
    // definition subgroup become warped; the underwater and coastal areas
    // remain unaffected.
    let select_turbulence = Select::new(base_continent_def(), warped_base_continent_definition, base_continent_def())
    .set_bounds(SEA_LEVEL - 0.0375, SEA_LEVEL + 1000.0375)
    .set_falloff(0.0625);

    // 5: [Continent-definition group]: Caches the output value from the
    // clamped-continent module. This is the output value for the entire
    // continent-definition group.
    let continent_definition = SyncCache::new(select_turbulence);

     // ////////////////////////////////////////////////////////////////////////
    // Function group: terrain type definition
    // ////////////////////////////////////////////////////////////////////////

    // ////////////////////////////////////////////////////////////////////////
    // Function subgroup: terrain type definition (3 noise functions)
    //
    // This subgroup defines the positions of the terrain types on the planet.
    //
    // Terrain types include, in order of increasing roughness, plains, hills,
    // and mountains.
    //
    // This subgroup's output value is based on the output value from the
    // continent-definition group. Rougher terrain mainly appears at higher
    // elevations.
    //
    // -1.0 represents the smoothest terrain types (plains and underwater) and
    // +1.0 represents the roughest terrain types (mountains).
    //

    // 1: [Warped-continent module]: This turbulence module slightly warps the
    // output value from the continent-definition group. This prevents the
    // rougher terrain from appearing exclusively at higher elevations. Rough
    // areas may now appear in the the ocean, creating rocky islands and
    // fjords.
    let warped_continent = Turbulence::<_, Perlin>::new(&continent_definition)
    .set_seed(CURRENT_SEED+20)
    .set_frequency(CONTINENT_FREQUENCY*18.125)
    .set_power(CONTINENT_FREQUENCY/20.59375*TERRAIN_OFFSET)
    .set_roughness(3);

    // 2: [Roughness-probability-shift module]: This terracing module sharpens
    // the edges of the warped-continent module near sea level and lowers the
    // slope towards the higher-elevation areas. This shrinks the areas in
    // which the rough terrain appears, increasing the "rarity" of rough
    // terrain.
    let roughness_probability_shift = Terrace::new(warped_continent)
    .add_control_point(-1.00)
    .add_control_point(SHELF_LEVEL+SEA_LEVEL / 2.0)
    .add_control_point(1.00);

     // 3: [Terrain-type-definition group]: Caches the output value from the
    // roughness-probability-shift module. This is the output value for the
    // entire terrain-type-definition group.
    let terrain_type_definition = SyncCache::new(roughness_probability_shift);

    // /////////////////////////////////////////////////////////////////////////
    // Function group: mountainous terrain
    // /////////////////////////////////////////////////////////////////////////

    // /////////////////////////////////////////////////////////////////////////
    // Function subgroup: mountain base definition (9 noise functions)
    //
    // This subgroup generates the base-mountain elevations. Other subgroups
    // will add the ridges and low areas to the base elevations.
    //
    // -1.0 represents low mountainous terrain and +1.0 represents high
    // mountainous terrain.
    //

    // 1: [Mountain-ridge module]: This ridged-multifractal-noise function
    // generates the mountain ridges.
    let mountain_ridge = RidgedMulti::<Perlin>::new(CURRENT_SEED+30)
    .set_frequency(1723.0)
    .set_lacunarity(MOUNTAIN_LACUNARITY)
    .set_octaves(4);

    // 2: [Scaled-mountain-ridge module]: Next, a scale/bias module scales the
    // output value from the mountain-ridge module so that its ridges are not
    // too high. The reason for this is that another subgroup adds actual
    // mountainous terrain to these ridges.
    let scaled_mountain_ridge = ScaleBias::new(mountain_ridge)
    .set_scale(0.5)
    .set_bias(0.375);

    // 3: [River-valley module]: This ridged-multifractal-noise function
    // generates the river valleys.  It has a much lower frequency than the
    // mountain-ridge module so that more mountain ridges will appear outside
    // of the valleys. Note that this noise function generates ridged-multifractal
    // noise using only one octave; this information will be important in the
    // next step.
    let river_valley = RidgedMulti::<Perlin>::new(CURRENT_SEED+31)
    .set_frequency(367.0)
    .set_lacunarity(MOUNTAIN_LACUNARITY)
    .set_octaves(1);

    // 4: [Scaled-river-valley module]: Next, a scale/bias module applies a
    // scaling factor of -2.0 to the output value from the river-valley module.
    // This stretches the possible elevation values because one-octave ridged-
    // multifractal noise has a lower range of output values than multiple-
    // octave ridged-multifractal noise. The negative scaling factor inverts
    // the range of the output value, turning the ridges from the river-valley
    // module into valleys.
    let scaled_river_valley = ScaleBias::new(river_valley)
    .set_scale(-2.0)
    .set_bias(-0.5);

    // 5: [Low-flat module]: This low constant value is used by step 6.
    let low_flat = Constant::new(-1.0);

    // 6: [Mountains-and-valleys module]: This blender module merges the scaled-
    // mountain-ridge module and the scaled-river-valley module together. It
    // causes the low-lying areas of the terrain to become smooth, and causes
    // the high-lying areas of the terrain to contain ridges. To do this, it
    // uses the scaled-river-valley module as the control module, causing the
    // low-flat module to appear in the lower areas and causing the scaled-
    // mountain-ridge module to appear in the higher areas.
    let mountains_and_valleys = Blend::new(
        &low_flat,
        &scaled_mountain_ridge,
        &scaled_river_valley,
    );

    scaled_river_valley
}

#[derive(Default)]
struct Custom;

impl Seedable for Custom {
    fn seed(&self) -> u32 {
        0
    }

    fn set_seed(self, _seed: u32) -> Self {
        self
    }
}

impl NoiseFn<f64, 2> for Custom {
    fn get(&self, [x, z]: [f64; 2]) -> f64 {
        (x + z) / 2.0
    }
}
