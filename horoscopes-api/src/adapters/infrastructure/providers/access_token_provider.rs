use chrono::prelude::Utc;
use hmac::{Hmac, NewMac};
use jwt::{
    Claims, Error, Header, RegisteredClaims, SignWithKey, Token,
    VerifyWithKey,
};
use serde_json::json;
use sha2::Sha256;
use std::collections::BTreeMap;

use crate::usecases::common::ports::providers::AccessTokenProvider;

pub struct AccessTokenProviderImpl {}

impl AccessTokenProviderImpl {
    pub fn new() -> Self {
        Self {}
    }
}

// TODO: 環境変数にしたい。
const VAR_KEY: &[u8] = b"horoscopes-secret";

impl AccessTokenProvider for AccessTokenProviderImpl {
    fn generate(
        &self,
        user_id: String,
        issued_at_timestamp: u64,
        expires_at_timestamp: u64,
    ) -> String {
        let key: Hmac<Sha256> = Hmac::new_varkey(VAR_KEY).unwrap();
        let header: Header = Default::default();

        let registered_claims = RegisteredClaims {
            expiration: Some(expires_at_timestamp),
            issued_at: Some(issued_at_timestamp),
            ..Default::default()
        };

        let mut private_claims = BTreeMap::new();
        private_claims.insert("user_id".to_string(), json!(user_id));

        let claims = Claims {
            registered: registered_claims,
            private: private_claims,
        };

        let access_token: Token<Header, Claims, _> =
            Token::new(header, claims);

        let signed_access_token = access_token
            .sign_with_key(&key)
            .expect("Failed to sign access token.");

        signed_access_token.into()
    }

    fn verify(&self, access_token: String) -> Result<String, String> {
        let key: Hmac<Sha256> = Hmac::new_varkey(VAR_KEY).unwrap();
        let _header: Header = Default::default();

        let res_claims: Result<Claims, Error> =
            access_token.verify_with_key(&key);

        match res_claims {
            Ok(claims) => {
                if let Some(exp) = claims.registered.expiration {
                    let now = Utc::now();
                    let now_ts = now.timestamp() as u64;
                    if now_ts > exp {
                        return Err(
                            "Access Token Expired".to_string()
                        );
                    }
                };

                match claims.private.get("user_id") {
                    Some(user_id) => Ok(user_id.to_string()),
                    None => panic!("Access Token Invalid"),
                }
            }
            Err(err) => {
                panic!(err)
            }
        }
    }
}
