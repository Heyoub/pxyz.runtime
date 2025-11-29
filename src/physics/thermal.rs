//! # Thermal Noise and Probabilistic Computation
//!
//! Traditional computing fights thermal noise. Physics-native computing uses it.
//!
//! ## The Insight
//!
//! At room temperature, thermal energy kT ≈ 0.026 eV provides random fluctuations.
//! Instead of suppressing these, we can use them as a computational resource
//! for sampling, optimization, and probabilistic inference.
//!
//! ## Boltzmann Predicates
//!
//! Instead of deterministic `x > threshold`:
//! ```text
//! P(true) = sigmoid((x - threshold) / temperature)
//!         = 1 / (1 + exp(-(x - threshold) / T))
//! ```
//!
//! At T → 0: deterministic (standard boolean)
//! At T → ∞: random (50/50)
//! At optimal T: simulated annealing for optimization
//!
//! ## Applications
//!
//! - Probabilistic edge traversal (exploration vs exploitation)
//! - Merge conflict resolution with uncertainty
//! - Sampling from constraint-satisfying states
//! - Energy-based optimization of graph layouts

use std::f64::consts::E;

/// Boltzmann constant × room temperature ≈ 0.026 eV
/// We normalize to 1.0 for our abstract temperature scale
pub const ROOM_TEMPERATURE: f64 = 1.0;

/// Temperature for deterministic behavior (effectively 0)
pub const ZERO_TEMPERATURE: f64 = 0.0001;

/// Temperature for maximum exploration
pub const HIGH_TEMPERATURE: f64 = 100.0;

/// Boltzmann distribution sampler
#[derive(Debug, Clone)]
pub struct BoltzmannSampler {
    /// Current temperature
    temperature: f64,
    /// Random state (simple xorshift)
    rng_state: u64,
}

impl BoltzmannSampler {
    /// Create sampler at room temperature
    pub fn new() -> Self {
        Self {
            temperature: ROOM_TEMPERATURE,
            rng_state: 0x853c49e6748fea9b, // Arbitrary seed
        }
    }

    /// Create with specific temperature
    pub fn with_temperature(temperature: f64) -> Self {
        Self {
            temperature: temperature.max(ZERO_TEMPERATURE),
            rng_state: 0x853c49e6748fea9b,
        }
    }

    /// Set temperature
    pub fn set_temperature(&mut self, temperature: f64) {
        self.temperature = temperature.max(ZERO_TEMPERATURE);
    }

    /// Get current temperature
    pub fn temperature(&self) -> f64 {
        self.temperature
    }

    /// Sigmoid function: 1 / (1 + e^(-x))
    fn sigmoid(x: f64) -> f64 {
        if x > 20.0 {
            1.0
        } else if x < -20.0 {
            0.0
        } else {
            1.0 / (1.0 + E.powf(-x))
        }
    }

    /// Boltzmann probability for a transition
    ///
    /// P(accept) = sigmoid((value - threshold) / temperature)
    pub fn probability(&self, value: f64, threshold: f64) -> f64 {
        let delta = value - threshold;
        Self::sigmoid(delta / self.temperature)
    }

    /// Sample from Boltzmann distribution
    ///
    /// Returns true with probability P(accept)
    pub fn sample(&mut self, value: f64, threshold: f64) -> bool {
        let p = self.probability(value, threshold);
        self.random_uniform() < p
    }

    /// Sample with energy-based acceptance (Metropolis-Hastings style)
    ///
    /// For minimization: always accept lower energy, probabilistically accept higher
    pub fn metropolis(&mut self, current_energy: f64, proposed_energy: f64) -> bool {
        let delta_e = proposed_energy - current_energy;

        if delta_e <= 0.0 {
            // Always accept lower energy
            true
        } else {
            // Accept with Boltzmann probability
            let acceptance = E.powf(-delta_e / self.temperature);
            self.random_uniform() < acceptance
        }
    }

    /// Annealing schedule: exponential cooling
    pub fn anneal(&mut self, cooling_rate: f64) {
        self.temperature *= 1.0 - cooling_rate;
        self.temperature = self.temperature.max(ZERO_TEMPERATURE);
    }

    /// Annealing schedule: linear cooling
    pub fn anneal_linear(&mut self, step: f64) {
        self.temperature -= step;
        self.temperature = self.temperature.max(ZERO_TEMPERATURE);
    }

    /// Reset to initial temperature
    pub fn reheat(&mut self, temperature: f64) {
        self.temperature = temperature.max(ZERO_TEMPERATURE);
    }

    /// Simple xorshift64 random number generator
    fn random_uniform(&mut self) -> f64 {
        self.rng_state ^= self.rng_state << 13;
        self.rng_state ^= self.rng_state >> 7;
        self.rng_state ^= self.rng_state << 17;

        // Convert to [0, 1)
        (self.rng_state as f64) / (u64::MAX as f64)
    }

    /// Seed the RNG
    pub fn seed(&mut self, seed: u64) {
        self.rng_state = if seed == 0 { 1 } else { seed };
    }
}

impl Default for BoltzmannSampler {
    fn default() -> Self {
        Self::new()
    }
}

/// Thermal predicate that uses noise for probabilistic evaluation
#[derive(Debug, Clone)]
pub struct ThermalPredicate {
    /// Base threshold
    pub threshold: f64,
    /// Noise sensitivity (how much thermal noise affects the decision)
    pub sensitivity: f64,
}

impl ThermalPredicate {
    pub fn new(threshold: f64, sensitivity: f64) -> Self {
        Self {
            threshold,
            sensitivity: sensitivity.max(0.0),
        }
    }

    /// Deterministic evaluation (T → 0)
    pub fn evaluate_deterministic(&self, value: f64) -> bool {
        value >= self.threshold
    }

    /// Probabilistic evaluation at given temperature
    pub fn evaluate_thermal(&self, value: f64, sampler: &mut BoltzmannSampler) -> bool {
        let effective_threshold = self.threshold;
        let effective_temp = sampler.temperature() * self.sensitivity;

        if effective_temp < ZERO_TEMPERATURE {
            // Deterministic
            value >= effective_threshold
        } else {
            sampler.sample(value, effective_threshold)
        }
    }

    /// Confidence in the evaluation (0.0 = uncertain, 1.0 = certain)
    pub fn confidence(&self, value: f64, temperature: f64) -> f64 {
        let delta = (value - self.threshold).abs();
        let effective_temp = temperature * self.sensitivity;

        if effective_temp < ZERO_TEMPERATURE {
            1.0
        } else {
            // Confidence increases with distance from threshold
            // and decreases with temperature
            let z = delta / effective_temp;
            1.0 - 2.0 * (0.5 - BoltzmannSampler::sigmoid(z).abs())
        }
    }
}

/// Simulated annealing optimizer for graph layout
#[derive(Debug)]
pub struct SimulatedAnnealing {
    sampler: BoltzmannSampler,
    initial_temperature: f64,
    cooling_rate: f64,
    iterations: usize,
}

impl SimulatedAnnealing {
    pub fn new(initial_temperature: f64, cooling_rate: f64, iterations: usize) -> Self {
        Self {
            sampler: BoltzmannSampler::with_temperature(initial_temperature),
            initial_temperature,
            cooling_rate,
            iterations,
        }
    }

    /// Standard configuration for layout optimization
    pub fn for_layout() -> Self {
        Self::new(10.0, 0.001, 10000)
    }

    /// Run optimization with energy function
    ///
    /// `neighbor`: Generate neighboring state from current
    /// `energy`: Compute energy of a state
    pub fn optimize<S, N, E>(&mut self, initial: S, mut neighbor: N, energy: E) -> (S, f64)
    where
        S: Clone,
        N: FnMut(&S, &mut BoltzmannSampler) -> S,
        E: Fn(&S) -> f64,
    {
        self.sampler.reheat(self.initial_temperature);

        let mut current = initial;
        let mut current_energy = energy(&current);

        let mut best = current.clone();
        let mut best_energy = current_energy;

        for _ in 0..self.iterations {
            let proposed = neighbor(&current, &mut self.sampler);
            let proposed_energy = energy(&proposed);

            if self.sampler.metropolis(current_energy, proposed_energy) {
                current = proposed;
                current_energy = proposed_energy;

                if current_energy < best_energy {
                    best = current.clone();
                    best_energy = current_energy;
                }
            }

            self.sampler.anneal(self.cooling_rate);
        }

        (best, best_energy)
    }

    /// Current temperature
    pub fn temperature(&self) -> f64 {
        self.sampler.temperature()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boltzmann_deterministic_at_zero_temp() {
        let mut sampler = BoltzmannSampler::with_temperature(ZERO_TEMPERATURE);

        // At zero temperature, should be deterministic
        for _ in 0..100 {
            assert!(sampler.sample(1.0, 0.0));   // 1 > 0 → always true
            assert!(!sampler.sample(-1.0, 0.0)); // -1 < 0 → always false
        }
    }

    #[test]
    fn test_boltzmann_probabilistic() {
        let mut sampler = BoltzmannSampler::with_temperature(1.0);
        sampler.seed(42);

        // At threshold (value = threshold), should be ~50/50
        let mut count = 0;
        for _ in 0..1000 {
            if sampler.sample(0.0, 0.0) {
                count += 1;
            }
        }

        // Should be roughly 500 ± 50
        assert!(count > 400 && count < 600);
    }

    #[test]
    fn test_annealing_cools_down() {
        let mut sampler = BoltzmannSampler::with_temperature(100.0);

        for _ in 0..100 {
            sampler.anneal(0.05);
        }

        assert!(sampler.temperature() < 10.0);
    }

    #[test]
    fn test_metropolis_always_accepts_lower() {
        let mut sampler = BoltzmannSampler::with_temperature(1.0);

        // Lower energy should always be accepted
        for _ in 0..100 {
            assert!(sampler.metropolis(10.0, 5.0));
        }
    }
}
