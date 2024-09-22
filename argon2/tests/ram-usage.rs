use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, Result as Rez},
    Argon2
};


const HASHED_AAAA: &str = "$argon2id$v=19$m=19456,t=2,p=1$JBGJ6MV6WMfRxt3SrRHsHg$XemLlBUVStBYVJKj/9qPYQxOSuvszcFh17NK6+t4tA8";
// const HASHED_BBBB: &str = "$argon2id$v=19$m=19456,t=2,p=1$q70hfZhYsxP3JFsOMVUlLw$r8U0ns/rNTLxLfteXmngqacmdu2DebicscNucgJsXw8";
// const HASHED_CCCC: &str = "$argon2id$v=19$m=19456,t=2,p=1$ecj8ig+YnGODLACQlv+xOQ$/zOr7QL6EKsQOxYbqIkQ3+jNhRJ4lwOFIT5RJlT0dao";


use memory_stats::memory_stats;
fn print_memory() {
    let stats = memory_stats().unwrap();
    println!("physical memory: {:.2} M", stats.physical_mem as f32 / 1_000_000.0);
}

/*
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};

fn create(pwd: &str) {
    let password = pwd.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2.hash_password(password, &salt).unwrap().to_string();

    println!("{}", password_hash);
}
*/

#[test]
fn ram_usage() {
    for _i in 1..10 {
        check(HASHED_AAAA, "AAAA");
    }

    // check(HASHED_AAAA, "AABA");
}

fn check(hashed: &str, pass: &str) {
    print_memory();
    let ph = PasswordHash::new(hashed).unwrap();

    Argon2::default().verify_password(pass.as_bytes(), &ph).unwrap();

    // Argon2::default().verify_password(...) expands as:
    // if let (Some(salt), Some(expected_output)) = (&ph.salt, &ph.hash) {
    //     let imp = Argon2::default();
    //     let res = imp.hash_password_customized(
    //         pass.as_bytes(),
    //         Some(ph.algorithm),
    //         ph.version,
    //         argon2::Params::try_from(&ph).unwrap(),
    //         *salt,
    //     ).unwrap();
    // }

    print_memory();
}
