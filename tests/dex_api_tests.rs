// custom structs from src/structs.rs
use arbuscular::structs::{Coin, Quantity, Recipient, SendCoin, ExchangeCoin, ExchangeReceipt};
// external crates
use httpmock::prelude::*;
use isahc::{prelude::*, Request};
use serde_json::{json, Value};

#[test]
fn exchange_coin_test() {
    let endpoint = "/exchange-coin";

    let server = MockServer::start();

    let m = server.mock(|when, then| {
        when.method(POST)
            .path(endpoint)
            .header("content-type", "application/json")
            .json_body_obj(&ExchangeCoin {
                exchange_this: Quantity {
                    coin: String::from("ETH"),
                    amount: f64::from(2.0),
                },
                get_this: Coin {
                    coin: String::from("SOL")
                }
            });
        then.status(200)
            .header("content-type", "application/json")
            .json_body_obj(&ExchangeReceipt {
                this_was_exchanged: Quantity {
                    coin: String::from("ETH"),
                    amount: f64::from(2.0),
                },
                got_item: Quantity {
                    coin: String::from("SOL"),
                    amount: f64::from(4.0)
                }
            });
    });

    // Act: Send the request and deserialize the response to JSON
    let mut response = Request::post(&format!("http://{0}{1}", server.address(), endpoint))
        .header("content-type", "application/json")
        .body(
            json!(&ExchangeCoin {
                exchange_this: Quantity {
                    coin: ("ETH").to_string(),
                    amount: 2.0 as f64
                },
                get_this: Coin {
                    coin: ("SOL").to_string()
                }
            })
            .to_string(),
        )
        .unwrap()
        .send()
        .unwrap();

    let receipt: ExchangeReceipt =
        serde_json::from_str(&response.text().unwrap()).expect("cannot deserialize JSON");

    // Assert
    m.assert();
    assert_eq!(response.status(), 200);
    assert_eq!(receipt.this_was_exchanged.coin, "ETH");
    assert_eq!(receipt.this_was_exchanged.amount, 2.0);
    assert_eq!(receipt.got_item.coin, "SOL");
    assert_eq!(receipt.got_item.amount, 4.0);
}

#[test]
fn send_coin_test() {
    let endpoint = "/send-coin";

    let server = MockServer::start();

    let m = server.mock(|when, then| {
        when.method(POST)
            .path(endpoint)
            .header("content-type", "application/json")
            .json_body_obj(&SendCoin {
                quantity: Quantity {
                    coin: String::from("SOL"),
                    amount: f64::from(11.0),
                },
                recipient: Recipient {
                    address: String::from("crYPt04ddr355")
                }
            });
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({ "send_success": true }));
    });

    // Act: Send the request and deserialize the response to JSON
    let mut response = Request::post(&format!("http://{0}{1}", server.address(), endpoint))
        .header("content-type", "application/json")
        .body(
            json!(&SendCoin {
                quantity: Quantity {
                    coin: ("SOL").to_string(),
                    amount: 11.0 as f64
                },
                recipient: Recipient {
                    address: ("crYPt04ddr355").to_string()
                }
            })
            .to_string(),
        )
        .unwrap()
        .send()
        .unwrap();

    let receipt: Value =
        serde_json::from_str(&response.text().unwrap()).expect("cannot deserialize JSON");

    // Assert
    m.assert();
    assert_eq!(response.status(), 200);
    assert_eq!(receipt.as_object().unwrap().get("send_success").unwrap(), true);
}

///////////////////////////////
// TODO: use structs for below
///////////////////////////////

#[test]
fn get_coin_price_test() {
    let endpoint = "/get-usdt-price";

    let server = MockServer::start();

    let m = server.mock(|when, then| {
        when.method(GET)
            .path(endpoint)
            .query_param("coin", "SOL");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({ "USDT": 9999 }));
    });

    // Act: Send the request and deserialize the response to JSON
    let url = &format!("http://{0}{1}?coin=SOL", server.address(), endpoint);
    let mut response = isahc::get(url).unwrap();

    let price: Value =
        serde_json::from_str(&response.text().unwrap()).expect("cannot deserialize JSON");

    // Assert
    m.assert();
    assert_eq!(response.status(), 200);
    assert_eq!(price.as_object().unwrap().get("USDT").unwrap(), 9999);
}

#[test]
fn get_full_wallet_balance_test() {
    let endpoint = "/get-balance";

    let server = MockServer::start();

    let m = server.mock(|when, then| {
        when.method(GET).path(endpoint);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "balance": {
                    "SOL": 55,
                    "ETH": 77,
                    "LINK": 99,
                }
            }));
    });

    // Act: Send the request and deserialize the response to JSON
    let url = &format!("http://{0}{1}", server.address(), endpoint);
    let mut response = isahc::get(url).unwrap();

    let balance: Value =
        serde_json::from_str(&response.text().unwrap()).expect("cannot deserialize JSON");

    // Assert
    m.assert();
    assert_eq!(response.status(), 200);
    assert_eq!(
        balance.as_object().unwrap().get("balance").unwrap().get("SOL").unwrap(),
        55
    );
    assert_eq!(
        balance.as_object().unwrap().get("balance").unwrap().get("ETH").unwrap(),
        77
    );
    assert_eq!(
        balance.as_object().unwrap().get("balance").unwrap().get("LINK").unwrap(),
        99
    );
}

#[test]
fn get_single_coin_balance_test() {
    let endpoint = "/get-balance";

    let server = MockServer::start();

    let m = server.mock(|when, then| {
        when.method(GET)
            .path(endpoint)
            .query_param("coin", "SOL");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({ "balance": {"SOL": 55} }));
    });

    // Act: Send the request and deserialize the response to JSON
    let url = &format!("http://{0}{1}?coin=SOL", server.address(), endpoint);
    let mut response = isahc::get(url).unwrap();

    let balance: Value =
        serde_json::from_str(&response.text().unwrap()).expect("cannot deserialize JSON");

    // Assert
    m.assert();
    assert_eq!(response.status(), 200);
    assert_eq!(
        balance.as_object().unwrap().get("balance").unwrap().get("SOL").unwrap(),
        55
    );
}