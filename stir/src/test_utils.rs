use p3_baby_bear::{BabyBear, Poseidon2BabyBear};
use p3_challenger::DuplexChallenger;
use p3_commit::ExtensionMmcs;
use p3_field::extension::BinomialExtensionField;
use p3_field::Field;
use p3_goldilocks::{Goldilocks, Poseidon2Goldilocks};
use p3_merkle_tree::MerkleTreeMmcs;
use p3_symmetric::{PaddingFreeSponge, TruncatedPermutation};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

use crate::{SecurityAssumption, StirConfig, StirParameters};

// This configuration is insecure (the field is too small). Use for testing
// purposes only!
pub type BB = BabyBear;
pub type BBExt = BinomialExtensionField<BB, 5>;

type BBPerm = Poseidon2BabyBear<16>;
type BBHash = PaddingFreeSponge<BBPerm, 16, 8, 8>;
type BBCompress = TruncatedPermutation<BBPerm, 2, 8, 16>;
type BBPacking = <BB as Field>::Packing;

type BBMMCS = MerkleTreeMmcs<BBPacking, BBPacking, BBHash, BBCompress, 8>;
pub type BBExtMMCS = ExtensionMmcs<BB, BBExt, BBMMCS>;

pub type BBChallenger = DuplexChallenger<BB, BBPerm, 16, 8>;

pub type GL = Goldilocks;
pub type GLExt = BinomialExtensionField<GL, 2>;

type GLPerm = Poseidon2Goldilocks<8>;
type GLHash = PaddingFreeSponge<GLPerm, 8, 4, 4>;
type GLCompress = TruncatedPermutation<GLPerm, 2, 4, 8>;
type GLPacking = <GL as Field>::Packing;

type GLMMCS = MerkleTreeMmcs<GLPacking, GLPacking, GLHash, GLCompress, 4>;
pub type GLExtMMCS = ExtensionMmcs<GL, GLExt, GLMMCS>;

pub type GLChallenger = DuplexChallenger<GL, GLPerm, 8, 4>;

macro_rules! impl_test_mmcs_config {
    ($name:ident, $ext_mmcs:ty, $perm:ty, $hash:ty, $compress:ty, $mmcs:ty) => {
        pub fn $name() -> $ext_mmcs {
            let mut rng = ChaCha20Rng::seed_from_u64(0);
            let perm = <$perm>::new_from_rng_128(&mut rng);
            let hash = <$hash>::new(perm.clone());
            let compress = <$compress>::new(perm.clone());
            <$ext_mmcs>::new(<$mmcs>::new(hash, compress))
        }
    };
}

macro_rules! impl_test_challenger {
    ($name:ident, $challenger:ty, $perm:ty) => {
        pub fn $name() -> $challenger {
            let mut rng = ChaCha20Rng::seed_from_u64(0);
            let perm = <$perm>::new_from_rng_128(&mut rng);
            <$challenger>::new(perm)
        }
    };
}

macro_rules! impl_test_stir_config {
    ($name:ident, $ext:ty, $ext_mmcs:ty, $mmcs_config_fn:ident) => {
        pub fn $name(
            log_starting_degree: usize,
            log_starting_inv_rate: usize,
            log_folding_factor: usize,
            num_rounds: usize,
        ) -> StirConfig<$ext, $ext_mmcs> {
            let security_level = 128;
            let security_assumption = SecurityAssumption::CapacityBound;
            let pow_bits = 20;

            let parameters = StirParameters::fixed_domain_shift(
                log_starting_degree,
                log_starting_inv_rate,
                log_folding_factor,
                num_rounds,
                security_assumption,
                security_level,
                pow_bits,
                $mmcs_config_fn(),
            );

            StirConfig::new(parameters)
        }
    };
}

macro_rules! impl_test_stir_config_folding_factors {
    ($name:ident, $ext:ty, $ext_mmcs:ty, $mmcs_config_fn:ident) => {
        pub fn $name(
            log_starting_degree: usize,
            log_starting_inv_rate: usize,
            log_folding_factors: Vec<usize>,
        ) -> StirConfig<$ext, $ext_mmcs> {
            let security_level = 128;
            let security_assumption = SecurityAssumption::CapacityBound;
            let pow_bits = 20;

            // With each subsequent round, the size of the evaluation domain is
            // decreased by a factor of 2 whereas the degree bound (plus 1) of the
            // polynomial is decreased by a factor of 2^log_folding_factor. Thus,
            // the logarithm of the inverse of the rate increases by log_k - 1.
            let mut i_th_log_rate = log_starting_inv_rate;

            let log_inv_rates = log_folding_factors
                .iter()
                .map(|log_k| {
                    i_th_log_rate = i_th_log_rate + log_k - 1;
                    i_th_log_rate
                })
                .collect();

            let parameters = StirParameters {
                log_starting_degree,
                log_starting_inv_rate,
                log_folding_factors,
                log_inv_rates,
                security_assumption,
                security_level,
                pow_bits,
                mmcs_config: $mmcs_config_fn(),
            };

            StirConfig::new(parameters)
        }
    };
}

impl_test_mmcs_config!(
    test_bb_mmcs_config,
    BBExtMMCS,
    BBPerm,
    BBHash,
    BBCompress,
    BBMMCS
);

impl_test_mmcs_config!(
    test_gl_mmcs_config,
    GLExtMMCS,
    GLPerm,
    GLHash,
    GLCompress,
    GLMMCS
);

impl_test_challenger!(test_bb_challenger, BBChallenger, BBPerm);
impl_test_challenger!(test_gl_challenger, GLChallenger, GLPerm);

impl_test_stir_config!(test_bb_stir_config, BBExt, BBExtMMCS, test_bb_mmcs_config);
impl_test_stir_config!(test_gl_stir_config, GLExt, GLExtMMCS, test_gl_mmcs_config);

impl_test_stir_config_folding_factors!(
    test_bb_stir_config_folding_factors,
    BBExt,
    BBExtMMCS,
    test_bb_mmcs_config
);

impl_test_stir_config_folding_factors!(
    test_gl_stir_config_folding_factors,
    GLExt,
    GLExtMMCS,
    test_gl_mmcs_config
);
