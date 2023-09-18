use data_encoding::HEXLOWER;
use orion::hazardous::hash::sha2::sha256::Sha256;

pub fn is_valid_proof_of_work(challenge: &String, solution: &String, difficulty: u64) -> bool {
    let decoded_challenge = if let Ok(challenge) = HEXLOWER.decode(challenge.as_bytes()) {
        challenge
    } else {
        return false;
    };
    let decoded_solution = if let Ok(solution) = HEXLOWER.decode(solution.as_bytes()) {
        solution
    } else {
        return false;
    };

    let mut hasher = Sha256::new();
    hasher
        .update(&decoded_challenge)
        .expect("Should be able to hash bytes");
    hasher
        .update(&decoded_solution)
        .expect("Should be able to hash bytes");

    let hash = hasher.finalize().expect("Should be able to hash bytes");

    // sketchy
    let hash_bytes = HEXLOWER.encode(hash.as_ref());

    // TODO: rewrite with bit mask
    for byte in hash_bytes
        .as_bytes()
        .iter()
        .take(difficulty as usize)
        .cloned()
    {
        if byte != b'0' {
            return false;
        }
    }

    true
}
