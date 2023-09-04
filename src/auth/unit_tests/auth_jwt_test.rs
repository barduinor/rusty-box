use crate::config;

use super::*;

#[test]
fn test_jwt_assertion() {
    // should create a Box JWT assertion
    let config = config::Config::default();

    // print current dir
    println!("Current dir: {:?}", std::env::current_dir());

    let encrypted_private_key = std::fs::read_to_string("tests/keys/private.pem").unwrap();
    let public_key = std::fs::read_to_string("tests/keys/public.pem").unwrap();
    let passphrase = "ABCD".to_string();

    let jwt_auth = JWTAuth::new(
        config,
        "client_id".to_string(),
        "client_secret".to_string(),
        SubjectType::Enterprise,
        "box_subject_id".to_string(),
        "public_key_id".to_string(),
        encrypted_private_key,
        passphrase,
    );

    let jwt = jwt_assertion(jwt_auth).unwrap_or("".to_string());

    // have a non empty jwt
    assert!(jwt.len() > 0);

    //have 3 dots in assertion
    let count = jwt.chars().filter(|&c| c == '.').count();
    assert_eq!(count, 2);

    // have a valid signature
    // Verifing JWT
    let verifier = RS512.verifier_from_pem(&public_key).unwrap();
    let (payload, header) = jwt::decode_with_verifier(&jwt, &verifier).unwrap();

    // assert headers
    assert_eq!(header.token_type(), Some("JWT"));
    assert_eq!(header.key_id(), Some("public_key_id"));
    assert_eq!(header.algorithm(), Some("RS512"));

    //assert claims
    assert_eq!(payload.issuer(), Some("client_id"));
    assert_eq!(payload.subject(), Some("box_subject_id"));
    assert_eq!(payload.claim("box_sub_type"), Some(&json!("enterprise")));
    assert_eq!(
        payload.audience(),
        Some(vec!["https://api.box.com/oauth2/token"])
    );
    assert!(payload.jwt_id() != Some(""));
    assert!(payload.expires_at() > Some(std::time::SystemTime::now()));
}
