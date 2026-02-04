use leansig::{serialization::Serializable, signature::SignatureScheme};
use rand::{SeedableRng, rngs::StdRng};


#[unsafe(no_mangle)]
pub extern "C" fn noop() {}

#[unsafe(no_mangle)]
pub extern "C" fn gen_keypair(
    seed: u64,
    activation_epoch: u32
) {

    type LeanSignatureScheme = leansig::signature::generalized_xmss::instantiations_poseidon_top_level::lifetime_2_to_the_32::hashing_optimized::SIGTopLevelTargetSumLifetime32Dim64Base8;
    
    let mut rng = StdRng::seed_from_u64(seed);
    let lifetime = 1 << 3;

    let (pk, sk) = LeanSignatureScheme::key_gen(&mut rng, activation_epoch as usize, lifetime);

    let pk_byte = pk.to_bytes();
    let sk_bytes = sk.to_bytes();
    
    let _ = pk_byte;
    let _ = sk_bytes;
}


#[unsafe(no_mangle)]
pub extern "C" fn message_signing(
    seed: u64,
    activation_epoch: u32,
    signing_epoch: u32
) {

    type LeanSignatureScheme = leansig::signature::generalized_xmss::instantiations_poseidon_top_level::lifetime_2_to_the_32::hashing_optimized::SIGTopLevelTargetSumLifetime32Dim64Base8;

    let mut rng = StdRng::seed_from_u64(seed);
    let lifetime = 1 << 3;
    let message: [u8; 32] = [0; 32];

    let (_, sk) = LeanSignatureScheme::key_gen(&mut rng, activation_epoch as usize, lifetime);

    let _ = LeanSignatureScheme::sign(&sk, signing_epoch, &message).unwrap();
    
}

#[unsafe(no_mangle)]
pub extern "C" fn sig_verification(
    seed: u64,
    activation_epoch: u32,
    signing_epoch: u32
) {

    type LeanSignatureScheme = leansig::signature::generalized_xmss::instantiations_poseidon_top_level::lifetime_2_to_the_32::hashing_optimized::SIGTopLevelTargetSumLifetime32Dim64Base8;
    

    let mut rng = StdRng::seed_from_u64(seed);
    let lifetime = 1 << 3;
    let message: [u8; 32] = [0; 32];

    let (pk, sk) = LeanSignatureScheme::key_gen(&mut rng, activation_epoch as usize, lifetime);

    let sig = LeanSignatureScheme::sign(&sk, signing_epoch, &message).unwrap();
    
    // signature verification
    let _ = LeanSignatureScheme::verify(&pk, signing_epoch, &message, &sig);

    let pk_byte = pk.to_bytes();
    let sk_bytes = sk.to_bytes();
    let sig_bytes = sig.to_bytes();
    
    let _ = pk_byte;
    let _ = sk_bytes;
    let _ = sig_bytes;
}
