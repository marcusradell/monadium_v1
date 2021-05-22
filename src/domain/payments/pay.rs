use crate::io::error::Error;
use actix_web::{HttpResponse, Result};
use reqwest::header::CONTENT_TYPE;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Amount {
    currency: String,
    value: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PaymentMethod {
    #[serde(rename(serialize = "type"))]
    type_: String,
    encrypted_card_number: String,
    encrypted_expiry_month: String,
    encrypted_expiry_year: String,
    encrypted_security_code: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Body {
    amount: Amount,
    reference: String,
    payment_method: PaymentMethod,
    return_url: String,
    merchant_account: String,
}

async fn handler(
    base_url: String,
    api_key: String,
    merchant_account: String,
    return_url: String,
) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let body = Body {
        amount: Amount {
            currency: "EUR".into(),
            value: 1000,
        },
        reference: "YOUR_ORDER_NUMBER".into(),
        payment_method: PaymentMethod {
            type_: "scheme".into(),
            encrypted_card_number: "5577000055770004".into(),
            encrypted_expiry_month: "03".into(),
            encrypted_expiry_year: "2030".into(),
            encrypted_security_code: "737".into(),
        },
        merchant_account,
        return_url,
    };

    let req = client
        .post(&base_url)
        .header("X-API-Key", api_key)
        .header(CONTENT_TYPE, "application/json")
        .header("Idempotency-Key", uuid::Uuid::new_v4().to_string())
        .json(&body);

    dbg!(&req);

    let res = req.send().await.map_err(|e| {
        dbg!(e);
        Error::InternalServerError
    })?;

    dbg!(res);

    Ok(())
}

pub async fn controller() -> Result<HttpResponse> {
    let api_key = std::env::var("PAYMENTS_API_KEY").expect("Missing PAYMENT_API_KEY.");
    let base_url = std::env::var("PAYMENTS_BASE_URL").expect("Missing PAYMENTS_BASE_URL.");
    let merchant_account =
        std::env::var("PAYMENTS_MERCHANT_ACCOUNT").expect("Missing PAYMENTS_MERCHANT_ACCOUNT.");
    let return_url = std::env::var("PAYMENTS_RETURN_URL").expect("Missing PAYMENTS_RETURN_URL");
    handler(base_url, api_key, merchant_account, return_url).await?;
    Ok(HttpResponse::Ok().finish())
}
