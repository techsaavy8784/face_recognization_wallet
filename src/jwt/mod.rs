use dotenvy::dotenv;
use hyper::StatusCode;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    wallet_pubkey: String,
    uid: i64,
    exp: usize,
    nbf: usize,
}

pub fn generate_token(wallet_pubkey: String, uid: i64) -> Result<String, StatusCode> {
    dotenv().ok();

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let in_sec = since_the_epoch.as_secs() as usize;

    let exp_time_str = env::var("JWT_EXPIRATION_TIME").expect("JWT_EXPIRATION_TIME must be set");
    let exp_time = exp_time_str
        .parse::<usize>()
        .expect("Failed to parse JWT_EXPIRATION_TIME");

    let not_before_str = env::var("JWT_NOT_BEFORE").expect("JWT_NOT_BEFORE must be set");
    let not_before = not_before_str
        .parse::<usize>()
        .expect("Failed to parse JWT_NOT_BEFORE");

    let claims = Claims {
        wallet_pubkey: wallet_pubkey.to_string(),
        uid,
        exp: in_sec + exp_time, // + 1 hour
        nbf: in_sec - not_before,
    };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key = EncodingKey::from_secret(secret.as_bytes());
    encode(&Header::default(), &claims, &key).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn is_valid(token: &str) -> Result<(bool, i64, String), StatusCode> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key = DecodingKey::from_secret(secret.as_bytes());
    let token_data =
        decode::<Claims>(token, &key, &Validation::new(Algorithm::HS256)).map_err(|error| {
            match error.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => StatusCode::UNAUTHORIZED,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;
    Ok((true, token_data.claims.uid, token_data.claims.wallet_pubkey))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_valid() {
        let token = generate_token("abc".to_string(), 1).unwrap();

        let (is_valid, uid, wallet_pubkey) = is_valid(&token).unwrap();
        assert_eq!(is_valid, true);
        assert_eq!(uid, 1);
        assert_eq!(wallet_pubkey, "abc".to_string());
    }
}
