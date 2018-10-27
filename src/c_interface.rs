use paillier::EncryptionKey;
use std::ffi::{CStr,CString};
use serde::ser::Serialize;
use std::os::raw::c_char;
use protocols::two_party_ecdsa::lindell_2017::*;
use cryptography_utils::cryptographic_primitives::proofs::dlog_zk_protocol::DLogProof;
use cryptography_utils::{BigInt, GE};

#[no_mangle]
pub extern "C" fn p1_keygen1_public_share(msg: *const party_one::KeyGenFirstMsg) -> *mut GE {
    unsafe {
        let z = Box::new((&*msg).public_share.clone());
        Box::into_raw(z)
    }
}

#[no_mangle]
pub extern "C" fn p1_keygen1_pk_commitment(msg: *const party_one::KeyGenFirstMsg) -> *mut BigInt {
    unsafe {
        let z = Box::new((&*msg).pk_commitment.clone());
        Box::into_raw(z)
    }
}

#[no_mangle]
pub extern "C" fn p1_keygen1_zk_pok_commitment(msg: *const party_one::KeyGenFirstMsg) -> *mut BigInt {
    unsafe {
        let z = Box::new((&*msg).zk_pok_commitment.clone());
        Box::into_raw(z)
    }
}

#[no_mangle]
pub extern "C" fn bigint_delete(msg: *mut BigInt) {
    unsafe {
        Box::from_raw(msg);
    }
}

#[no_mangle]
pub extern "C" fn ge_delete(msg: *mut GE) {
    unsafe {
        Box::from_raw(msg);
    }
}

#[no_mangle]
pub extern "C" fn p1_keygen1_delete(msg : *mut party_one::KeyGenFirstMsg) {
    unsafe {
        Box::from_raw(msg);
    }
}

#[no_mangle]
pub extern "C" fn p1_keygen1_new_create_commitments() -> *mut party_one::KeyGenFirstMsg {
    let x = Box::new(party_one::KeyGenFirstMsg::create_commitments());
    Box::into_raw(x)
}

#[no_mangle]
pub extern "C" fn p2_keygen1_delete(msg : *mut party_two::KeyGenFirstMsg) {
    unsafe {
        Box::from_raw(msg);
    }
}

#[no_mangle]
pub extern "C" fn p2_keygen1_new_create() -> *mut party_two::KeyGenFirstMsg {
    let x = Box::new(party_two::KeyGenFirstMsg::create());
    Box::into_raw(x)
}

#[no_mangle]
pub extern "C" fn d_log_proof_delete(msg: *mut DLogProof) {
    unsafe {
        Box::from_raw(msg);
    }
}

#[no_mangle]
pub extern "C" fn d_log_proof_nullable_new_deserialize(msg: *mut c_char) -> *mut DLogProof {
    unsafe {
        let x = CStr::from_ptr(msg).to_string_lossy().into_owned();
        match serde_json::from_str(&x) {
            Err(e) => 0 as *mut DLogProof, // TODO: test this
            Ok(y) => {
                let z = Box::new(y);
                Box::into_raw(z)
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn d_log_proof_serialize(msg: *const DLogProof) -> *mut c_char {
    unsafe {
        let x = conv(&*msg);
        //let y = d_log_proof_new_deserialize(x);
        //d_log_proof_delete(y);
        x
    }
}

#[no_mangle]
pub extern "C" fn c_str_delete(msg: *mut c_char) {
    unsafe {
        CString::from_raw(msg);
    }
}

#[no_mangle]
pub extern "C" fn p2_keygen1_d_log_proof(msg: *const party_two::KeyGenFirstMsg) -> *mut DLogProof {
    unsafe {
        let z = Box::new((&*msg).d_log_proof.clone());
        Box::into_raw(z)
    }
}

#[no_mangle]
pub extern "C" fn p2_keygen1_public_share(msg: *const party_two::KeyGenFirstMsg) -> *mut GE {
    unsafe {
        let z = Box::new((&*msg).public_share.clone());
        Box::into_raw(z)
    }
}

#[no_mangle]
pub extern "C" fn p1_keygen2_delete(msg : *mut party_one::KeyGenSecondMsg) {
    unsafe {
        Box::from_raw(msg);
    }
}

#[no_mangle]
pub extern "C" fn p1_keygen2_nullable_new_verify_and_decommit(p1keygen1 : *const party_one::KeyGenFirstMsg, dlogproof : *const DLogProof) -> *mut party_one::KeyGenSecondMsg {
    unsafe {
        let x = party_one::KeyGenSecondMsg::verify_and_decommit(&*p1keygen1, &*dlogproof);
        match x {
            Err(err) => 0 as *mut party_one::KeyGenSecondMsg,
            Ok(y) => {
                let z = Box::new(y);
                Box::into_raw(z)
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn p1_keygen2_pk_commitment_blind_factor(msg: *const party_one::KeyGenSecondMsg) -> *mut BigInt {
    unsafe {
        let z = Box::new((&*msg).pk_commitment_blind_factor.clone());
        Box::into_raw(z)
    }
}

#[no_mangle]
pub extern "C" fn p1_keygen2_zk_pok_blind_factor(msg: *const party_one::KeyGenSecondMsg) -> *mut BigInt {
    unsafe {
        let z = Box::new((&*msg).zk_pok_blind_factor.clone());
        Box::into_raw(z)
    }
}

#[no_mangle]
pub extern "C" fn p1_keygen2_public_share(msg: *const party_one::KeyGenSecondMsg) -> *mut GE {
    unsafe {
        let z = Box::new((&*msg).public_share.clone());
        Box::into_raw(z)
    }
}

#[no_mangle]
pub extern "C" fn p1_keygen2_d_log_proof(msg: *const party_one::KeyGenSecondMsg) -> *mut DLogProof {
    unsafe {
        let x = serde_json::to_string(&*msg).unwrap();
        let y: party_one::KeyGenSecondMsg = serde_json::from_str(&x).unwrap();
        let z = Box::new(y.d_log_proof);
        Box::into_raw(z)
    }
}

fn conv<T: Serialize>(msg: &T) -> *mut c_char {
    CString::new(
        serde_json::to_string(msg)
            .unwrap_or("error".to_string()
        )
    )
    .unwrap() // safe because it doesn't contain \0
    .into_raw()
}

#[no_mangle]
pub extern "C" fn p2_keygen2_nullable_new_verify_commitments_and_dlog_proof(
    // all these are from party_one
    pk_com: *const BigInt,
    zk_pok: *const BigInt,
    zk_pok_blind: *const BigInt,
    public_share: *const GE,
    pk_com_blind_party: *const BigInt,
    d_log_proof: *const DLogProof,
) -> *mut party_two::KeyGenSecondMsg {
    unsafe {
        let x = party_two::KeyGenSecondMsg::verify_commitments_and_dlog_proof(
            &*pk_com,
            &*zk_pok,
            &*zk_pok_blind,
            &*public_share,
            &*pk_com_blind_party,
            &*d_log_proof,
        );
        match x {
            Ok(msg) => {
                let y = Box::new(msg);
                Box::into_raw(y)
            },
            Err(err) => 0 as *mut party_two::KeyGenSecondMsg
        }
    }
}

#[no_mangle]
pub extern "C" fn p2_keygen2_delete(msg : *mut party_two::KeyGenSecondMsg) {
    unsafe {
        Box::from_raw(msg);
    }
}

#[no_mangle]
pub extern "C" fn p1_paillier_pair_new_generate_keypair_and_encrypted_share(msg : *const party_one::KeyGenFirstMsg) -> *mut party_one::PaillierKeyPair {
    unsafe {
        let x = party_one::PaillierKeyPair::generate_keypair_and_encrypted_share(&*msg);
        let y = Box::new(x);
        Box::into_raw(y)
    }
}

#[no_mangle]
pub extern "C" fn p1_paillier_pair_ek(msg : *const party_one::PaillierKeyPair) -> *mut EncryptionKey {
    unsafe {
        let z = Box::new((&*msg).ek.clone());
        Box::into_raw(z)
    }
}

#[no_mangle]
pub extern "C" fn p1_paillier_pair_encrypted_share(msg : *const party_one::PaillierKeyPair) -> *mut BigInt {
    unsafe {
        let z = Box::new((&*msg).encrypted_share.clone());
        Box::into_raw(z)
    }
}

#[no_mangle]
pub extern "C" fn p1_paillier_pair_delete(msg : *mut party_one::PaillierKeyPair) {
    unsafe {
        Box::from_raw(msg);
    }
}

#[no_mangle]
pub extern "C" fn p2_paillier_public_new(ek_ptr : *const EncryptionKey, encrypted_secret_share_ptr : *const BigInt) -> *mut party_two::PaillierPublic {
    unsafe {
        let ek: EncryptionKey = (&*ek_ptr).clone();
        let encrypted_secret_share: BigInt = (&*encrypted_secret_share_ptr).clone();
        let y = party_two::PaillierPublic { ek, encrypted_secret_share };
        let z = Box::new(y);
        Box::into_raw(z)
    }
}

#[no_mangle]
pub extern "C" fn p2_paillier_public_delete(msg : *mut party_two::PaillierPublic) {
    unsafe {
        Box::from_raw(msg);
    }
}

#[no_mangle]
pub extern "C" fn encryption_key_delete(msg : *mut paillier::EncryptionKey) {
    unsafe {
        Box::from_raw(msg);
    }
}

pub struct ChalVeriPair {
    challenge: paillier::Challenge,
    verification_aid: paillier::VerificationAid,
}

#[no_mangle]
pub extern "C" fn chal_veri_pair_new(msg : *const party_two::PaillierPublic) -> *mut ChalVeriPair {
    unsafe {
        let (challenge, verification_aid) = party_two::PaillierPublic::generate_correct_key_challenge(&*msg);
        let x = ChalVeriPair {
            challenge,
            verification_aid,
        };
        let y = Box::new(x);
        Box::into_raw(y)
    }
}

#[no_mangle]
pub extern "C" fn chal_veri_pair_challenge(msg : *const ChalVeriPair) -> *mut paillier::Challenge {
    // copying with json since Challenge doesn't impl Clone
    unsafe {
        let z = serde_json::to_string(&(&*msg).challenge)
            .unwrap_or("error".to_string());
        match serde_json::from_str(&z) {
            Err(e) => 0 as *mut paillier::Challenge,
            Ok(y) => {
                let z = Box::new(y);
                Box::into_raw(z)
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn chal_veri_pair_verification_aid(msg: *const ChalVeriPair) -> *mut paillier::VerificationAid {
    // copying with json since VerificationAid doesn't impl Clone
    unsafe {
        let z = serde_json::to_string(&(&*msg).verification_aid)
            .unwrap_or("error".to_string());
        match serde_json::from_str(&z) {
            Err(e) => 0 as *mut paillier::VerificationAid,
            Ok(y) => {
                let z = Box::new(y);
                Box::into_raw(z)
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn chal_veri_pair_delete(msg : *mut ChalVeriPair) {
    unsafe {
        Box::from_raw(msg);
    }
}

#[no_mangle]
pub extern "C" fn challenge_delete(msg: *mut paillier::Challenge) {
    unsafe {
        Box::from_raw(msg);
    }
}

#[no_mangle]
pub extern "C" fn p1_paillier_pair_nullable_generate_proof_correct_key(p1pair: *const party_one::PaillierKeyPair, chal: *const paillier::Challenge) -> *mut paillier::CorrectKeyProof {
    unsafe {
        let x = party_one::PaillierKeyPair::generate_proof_correct_key(&*p1pair, &*chal);
        match x {
            Err(e) => 0 as *mut paillier::CorrectKeyProof,
            Ok(y) => {
                let z = Box::new(y);
                Box::into_raw(z)
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn correct_key_proof_delete(msg: *mut paillier::CorrectKeyProof) {
    unsafe {
        Box::from_raw(msg);
    }
}

#[no_mangle]
pub extern "C" fn p2_paillier_public_verify_correct_key(c_k_p: *const paillier::CorrectKeyProof, veri_aid : *const paillier::VerificationAid) -> bool {
    unsafe {
        match party_two::PaillierPublic::verify_correct_key(&*c_k_p, &*veri_aid) {
            Ok(res) => true,
            Err(e) => false,
        }
    }
}

#[no_mangle]
pub extern "C" fn verification_aid_delete(msg: *mut paillier::VerificationAid) {
    unsafe {
        Box::from_raw(msg);
    }
}

#[no_mangle]
pub extern "C" fn range_tuple_delete(msg: *mut RangeProofTriple) {
    unsafe {
        Box::from_raw(msg);
    }
}

#[no_mangle]
pub extern "C" fn encrypted_pairs_delete(msg: *mut paillier::EncryptedPairs) {
    unsafe {
        Box::from_raw(msg);
    }
}

#[no_mangle]
pub extern "C" fn proof_delete(msg: *mut paillier::Proof) {
    unsafe {
        Box::from_raw(msg);
    }
}

#[no_mangle]
pub extern "C" fn challenge_bits_delete(msg: *mut paillier::ChallengeBits) {
    unsafe {
        Box::from_raw(msg);
    }
}

#[no_mangle]
pub extern "C" fn range_tuple_1(msg: *const RangeProofTriple) -> *mut paillier::EncryptedPairs {
    unsafe {
        let z = serde_json::to_string(&(&*msg).encrypted_pairs)
            .unwrap_or("error".to_string());
        match serde_json::from_str(&z) {
            Err(e) => 0 as *mut paillier::EncryptedPairs,
            Ok(y) => {
                let z = Box::new(y);
                Box::into_raw(z)
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn range_tuple_2(msg: *const RangeProofTriple) -> *mut paillier::ChallengeBits {
    unsafe {
        let z = serde_json::to_string(&(&*msg).challenge)
            .unwrap_or("error".to_string());
        match serde_json::from_str(&z) {
            Err(e) => 0 as *mut paillier::ChallengeBits,
            Ok(y) => {
                let z = Box::new(y);
                Box::into_raw(z)
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn range_tuple_3(msg: *const RangeProofTriple) -> *mut paillier::Proof {
    unsafe {
        let z = serde_json::to_string(&(&*msg).proof)
            .unwrap_or("error".to_string());
        match serde_json::from_str(&z) {
            Err(e) => 0 as *mut paillier::Proof,
            Ok(y) => {
                let z = Box::new(y);
                Box::into_raw(z)
            }
        }
    }
}

pub struct RangeProofTriple {
    encrypted_pairs: paillier::EncryptedPairs,
    challenge: paillier::ChallengeBits,
    proof: paillier::Proof,
}

#[no_mangle]
pub extern "C" fn p1_paillier_pair_generate_range_proof(pkp: *const party_one::PaillierKeyPair, msg : *const party_one::KeyGenFirstMsg) -> *mut RangeProofTriple {
    unsafe {
        let (encrypted_pairs, challenge, proof) = party_one::PaillierKeyPair::generate_range_proof(&*pkp, &*msg);
        let x = RangeProofTriple {
            encrypted_pairs, challenge, proof
        };
        let y = Box::new(x);
        Box::into_raw(y)
    }
}

#[no_mangle]
pub extern "C" fn p2_paillier_public_verify_range_proof(p2pub: *const party_two::PaillierPublic, challenge_bits : *const paillier::ChallengeBits, encrypted_pairs: *const paillier::EncryptedPairs, proof: *const paillier::Proof) -> bool {
    unsafe {
        match party_two::PaillierPublic::verify_range_proof(&*p2pub, &*challenge_bits, &*encrypted_pairs, &*proof) {
            Ok(res) => true,
            Err(e) => false,
        }
    }
}

// todo: signing
