use jsonwebtoken::{DecodingKey, decode, Validation, Algorithm};
use serde::Deserialize;
use serde_json;
use dotenv;

// This is the structure that Next.js generates
#[derive(Deserialize, Debug)]
struct Claims {
    name: String,
    email: String,
    picture: String,
    sub: String,
    iat: i32,
    exp: i32
}

#[derive(Deserialize, Debug)]
struct Key {
    kty: String,
    kid: String,
    alg: String,
    k: String
}

/// Verify request token
fn verify_jwt_signature(token:&str) {
    // generate the key by running:
    // jose newkey -s 512 -t oct -a HS512
    // Paste the output into the jwt.signingKey field in the next-auth config in pages/api/auth/[...nextauth].js
    dotenv::from_path("../../.env").ok();

    let key_json = std::env::var("SIGNING_KEY").unwrap();
    let key: Key = serde_json::from_str(&key_json).unwrap();
    let key = DecodingKey::from_base64_secret(&key.k).unwrap();

    let decoded = decode::<Claims>(
        token,
        &key,
        &Validation::new(Algorithm::HS512),
    ).unwrap();

    println!("decoded JWT: {:?}", decoded.claims);
}

fn main() {
    // This is the value of the next-auth.session-token cookie, which can be extracted using the Application > Cookies view in chrome debugger
    let token = "eyJhbGciOiJIUzUxMiJ9.eyJuYW1lIjoiTWFyayBCcm9jYXRvIiwiZW1haWwiOiJtYXJrLmJyb2NhdG9AbGF5ZXIwLmNvIiwicGljdHVyZSI6Imh0dHBzOi8vbGgzLmdvb2dsZXVzZXJjb250ZW50LmNvbS9hLS9BT2gxNEdoRDFEbW9BNFlodTFpUlh2N3FGdnNKNXd2T3A2RmVLNjJidVRLMlV1Mk14ZXl0YVc0ZDhsZEVvZWNWVlNzREdMb3NiRTgzaU5oVVg5cFN0d2NLMzZlb0FhSmZWV3dyQ3lKLTR3Z29mR2hqYVJSX005c0N5TGVHX1ZzVjJCNHFyUXlQNHNfMU5xUmM4OS01OVIxdzkyU1J4Z1FjUTBkRHdrakFzaDBpbFdxUlRBQVAxU3BhRFlCT3lKY3IwczZtVmx0dU9QclJ2QXpaY3JmcWk2VEUtUzhlajl4c3dtZnlNWm9oMV9kYU9HMUh6a08zeGFueElGOElONmJsU1hFRVQzMTR1YndyU1ktS2FvblpjUUJ3NjdPUW96TGNQNXpkclFNdzZyNlRjN1VMNU84aXlSblZ1dWFFSENEeTUzUUZTajk1eXNIWHhKMUs5MzdLNHhBbmhLM3ZrWHpYbnh3eVpxc3Y3ekhJSlk4YXFwdEppRWthTXBSS1Uyd21zekFZbUNBSmNlYlQzZXNHWmdHd2U4aTFoVk53WFlOZ3BaNU1yNXZOTTZ4dV9xc1BuZ013RVFkdjF3T25Zd092NFVYbXpBcmFQVU5zV3RLYXh0QmRRak1pS2hETXFldHJjQm16bE5yY2RKM2g0X01sem04bGxmQ0tnQkRtV3NmdngzSk5DeGVIQXBsNzVQenhqbUFsSl9JVzRQS1FEX2o5TExYVHppWHdvVWJJa2tkWEJjb0MyQ1Q0ZmM3dTB5SlZtZ1JwNWN3TFRRV3FmU0tNYkdvTDZrbTQ5UWxubWdOS2Y3NWdXOVYxcWZGOVZOUmpacU94V3JXbFBwWld4WHlPWWx5N1JvLXMxOFF3YnFUOEVMd2lSX0xNUTMtbTNEMTEwMWZKcXBZc2ZhVU1BbVVyTGRKQWJGYmpTclhtZ1JSVlo4Vlduc0hpNTZIemhfZFhKckVxZVR4S0kzWDZqbmhITGZJQ3hJczM3VzNWNkEwTC1jLVNkd1M1elJMazgtb1ZlQXNOcTVCdDNRPXM5Ni1jIiwic3ViIjoiMSIsImlhdCI6MTYzNDAyOTcxNywiZXhwIjoxNjM2NjIxNzE3fQ.fyOSefx2YJvaLNP08oivw3QeLVoF0JVUqazpzEZ1ed__Qn8iAxt3Fm-hS84huNpVbSAgss1T95NYcDcipkPYYg";
    
    verify_jwt_signature(token);
}
