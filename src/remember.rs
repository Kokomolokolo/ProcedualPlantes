let continent_frequency = 1.3;
let continent_noise = perlin.get([
    direction.x as f64 * continent_frequency, direction.y as f64 * continent_frequency, direction.z as f64 * continent_frequency
]);

let height_freq = planet_info.frequency as f64;
let height_noise = perlin.get([
    direction.x as f64 * height_freq, direction.y as f64 * height_freq, direction.z as f64 * height_freq
]);

// Landmaske
// Wenn das contitent noise unter der Küstenlinie leigt, dann ist die Maske 0. Wenn sie darüber liegt
// dann werden die Berge immer mit dem Kontinent Noise größer, sodass große Berge weiter entfernt von Wasser sind.
let land_mask = if continent_noise > -0.5 {(continent_noise + 0.1).min(1.0)} else { 0.0 };
let combined_noise = continent_noise + (height_noise * 0.8 * land_mask); // Landmask 0 => glatten Boden

// Die Höhe aus dem errechneten Noise mit amplitude
let height_modifier = combined_noise as f32 * planet_info.amplitude;

// combined noise bewegt sich jetzt zwischen -1.2..1.2 deswegen folgende Normalisierung
let t = ((combined_noise + 1.8) / 3.4).clamp(0.0, 1.0) as f32;

// Farben auf den Planeten basierend auf dem Noise:
let deep_sea = LinearRgba::new(0.05, 0.15, 0.45, 1.0);
let shallow_water = LinearRgba::new(0.1, 0.3, 0.65, 1.0);
let sand = LinearRgba::new(0.85, 0.75, 0.55, 1.0);
let grass = LinearRgba::new(0.15, 0.5, 0.2, 1.0);
let mountain = LinearRgba::new(0.4, 0.35, 0.3, 1.0);
let snow = LinearRgba::new(0.95, 0.95, 0.95, 1.0);


let color = if t < 0.3 {
    deep_sea.lerp(shallow_water, t / 0.3)
} else if t < 0.45 {
    let factor = (t - 0.3) / (0.45 - 0.3);
    shallow_water.lerp(sand, factor)
} else if t < 0.65 {
    let factor = (t-0.45) / (0.65 - 0.45);
    sand.lerp(grass, factor)
} else if t < 0.82 {
    let factor = (t - 0.65) / (0.82 - 0.65);
    grass.lerp(mountain, factor)
} else {
    let factor = (t - 0.82) / (1.0 - 0.82);
    mountain.lerp(snow, factor)
};