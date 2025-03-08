use alloc::vec::Vec;

use p3_commit::Mmcs;
use p3_field::Field;
use p3_poly::Polynomial;
use serde::{Deserialize, Serialize};

/// A STIR proof that the committed polynomial satisfies the configured degree
/// bound.
#[derive(Serialize, Deserialize, Clone)]
#[serde(bound(
    serialize = "Witness: Serialize, RoundProof<F, M, Witness>: Serialize, Polynomial<F>: Serialize",
    deserialize = "Witness: Deserialize<'de>, RoundProof<F, M, Witness>: Deserialize<'de>, Polynomial<F>: Deserialize<'de>"
))]
pub struct StirProof<F: Field, M: Mmcs<F>, Witness> {
    // Round proofs for the full-rounds i = 1, ..., M
    pub(crate) round_proofs: Vec<RoundProof<F, M, Witness>>,

    // Final poylnomial `p = g_{M + 1}` in plain
    pub(crate) final_polynomial: Polynomial<F>,

    // Proof of work for the final round
    pub(crate) pow_witness: Witness,

    // Merkle proofs for the final-round openings (of g_M)
    pub(crate) final_round_queries: Vec<(Vec<F>, M::Proof)>,
}

// A proof for one of the M full rounds of the protocol
#[derive(Serialize, Deserialize, Clone)]
#[serde(bound(
    serialize = "Witness: Serialize, Polynomial<F>: Serialize",
    deserialize = "Witness: Deserialize<'de>, Polynomial<F>: Deserialize<'de>",
))]
pub(crate) struct RoundProof<F: Field, M: Mmcs<F>, Witness> {
    // Important note:
    // The indices are given in the following frame of reference: Self is
    // produced inside prove_round for round i (for i = 1, ..., M) and are
    // consistent with the article's notation. The final round, with index M +
    // 1, does not produce a RoundProof.

    // Commitment to the stacked evaluations of g_i over the domain L_i
    pub(crate) g_root: M::Commitment,

    // Replies beta_{i, j} to the out-of-domain queries to g_i
    pub(crate) betas: Vec<F>,

    // Polynomial interpolating the evaluations of g_i at the in-domain and
    // out-of-domain queried points, r_{i, j}^shift and r_{i, j}^ood, resp.
    pub(crate) ans_polynomial: Polynomial<F>,

    // Merkle proofs of the committed evaluations of g_{i - 1} necessary to
    // compute f_{i - 1} at the k_i-th roots of the in-domain queried points
    // r_{i, j}^shift
    pub(crate) query_proofs: Vec<(Vec<F>, M::Proof)>,

    // Auxiliary polynomial helping the verifier evaluate ans_polynomial at the
    // queried points
    pub(crate) shake_polynomial: Polynomial<F>,

    // Solution to the proof-of-work challenge in round i
    pub(crate) pow_witness: Witness,
}
