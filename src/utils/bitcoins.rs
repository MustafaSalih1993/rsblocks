use crate::config::CONFIG;
use crate::types::ThreadsData;
use serde::Deserialize;

#[derive(Deserialize)]
struct Response {
    price_24h: f64,
}

pub async fn get_price() -> ThreadsData {
    let url = format!(
        "https://api.blockchain.com/v3/exchange/tickers/{}",
        CONFIG.bitcoins.symbol
    );
    let _err = String::from("Error");
    let res = match minreq::get(url).send() {
        Ok(resp) => match resp.json::<Response>() {
            Ok(data) => data.price_24h.to_string(),
            _ => _err,
        },
        Err(_) => _err,
    };

    let data = format!("  {}  {}  {}", CONFIG.bitcoins.icon, res, CONFIG.seperator);
    ThreadsData::BitCoins(data)
}
