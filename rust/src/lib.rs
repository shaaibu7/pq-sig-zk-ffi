use leansig::{serialization::Serializable, signature::SignatureScheme};
use rand::{SeedableRng, rngs::StdRng};



#[unsafe(no_mangle)]
pub extern "C" fn gen_keypair(
    seed: u64,
    activation_epoch: u32
) {

    type LeanSignatureScheme = leansig::signature::generalized_xmss::instantiations_poseidon_top_level::lifetime_2_to_the_32::hashing_optimized::SIGTopLevelTargetSumLifetime32Dim64Base8;
    /// The public key type from the leansig library.
    type LeanSigPublicKey = <LeanSignatureScheme as SignatureScheme>::PublicKey;
    
    let mut rng = StdRng::seed_from_u64(seed);
    let lifetime = 1 << 3;
    

    let (pk, sk) = LeanSignatureScheme::key_gen(&mut rng, activation_epoch as usize, lifetime);

    let pk_byte = pk.to_bytes();
    let sk_bytes = sk.to_bytes();
    
    let _ = pk_byte;
    let _ = sk_bytes;
  
    let pub_key = LeanSigPublicKey::from_bytes(&pk_byte).unwrap();
    // let secret_key = LeanSigSecretKey::from_bytes(&sk_bytes).unwrap();

    println!("============================key generation completed==================================");
    println!("The public key data is: root {:?} parameter {:?}", pub_key.root(), pub_key.parameter())
    
    
}


#[unsafe(no_mangle)]
pub extern "C" fn message_signing(
    seed: u64,
    activation_epoch: u32,
    signing_epoch: u32
) {

    type LeanSignatureScheme = leansig::signature::generalized_xmss::instantiations_poseidon_top_level::lifetime_2_to_the_32::hashing_optimized::SIGTopLevelTargetSumLifetime32Dim64Base8;
    
    /// The signature type from the leansig library.
    type LeanSigSignature = <LeanSignatureScheme as SignatureScheme>::Signature;

    let mut rng = StdRng::seed_from_u64(seed);
    let lifetime = 1 << 3;
    let message: [u8; 32] = [0; 32];

    let (_, sk) = LeanSignatureScheme::key_gen(&mut rng, activation_epoch as usize, lifetime);

    let sig = LeanSignatureScheme::sign(&sk, signing_epoch, &message).unwrap();
    

    let sig_bytes = sig.to_bytes();
    
    
    let signature = LeanSigSignature::from_bytes(&sig_bytes).unwrap();

    println!("============================message signing completed==================================");
    println!("The byte representation of the data is: {:?}", sig_bytes);
    
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
    let is_valid_sig = LeanSignatureScheme::verify(&pk, signing_epoch, &message, &sig);

    let pk_byte = pk.to_bytes();
    let sk_bytes = sk.to_bytes();
    let sig_bytes = sig.to_bytes();
    
    let _ = pk_byte;
    let _ = sk_bytes;
    let _ = sig_bytes;

    println!("============================signature verification completed==================================");
    println!("The signature validity status is: {}", is_valid_sig);
}
