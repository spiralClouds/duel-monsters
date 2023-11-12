use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use anyhow::Result;
use serde_json;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardDb {
    pub data: Vec<Card>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub frame_type: String,
    pub desc: String,
    pub race: String,
    pub archetype: Option<String>,
    #[serde(rename = "card_sets")]
    #[serde(default)]
    pub card_sets: Vec<CardSet>,
    #[serde(rename = "card_images")]
    pub card_images: Vec<CardImage>,
    #[serde(rename = "card_prices")]
    pub card_prices: Vec<CardPrice>,
    pub atk: Option<i64>,
    pub def: Option<i64>,
    pub level: Option<i64>,
    pub attribute: Option<String>,
    #[serde(rename = "pend_desc")]
    pub pend_desc: Option<String>,
    #[serde(rename = "monster_desc")]
    pub monster_desc: Option<String>,
    pub scale: Option<i64>,
    pub linkval: Option<i64>,
    #[serde(default)]
    pub linkmarkers: Vec<String>,
    #[serde(rename = "banlist_info")]
    pub banlist_info: Option<BanlistInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardSet {
    #[serde(rename = "set_name")]
    pub set_name: String,
    #[serde(rename = "set_code")]
    pub set_code: String,
    #[serde(rename = "set_rarity")]
    pub set_rarity: String,
    #[serde(rename = "set_rarity_code")]
    pub set_rarity_code: String,
    #[serde(rename = "set_price")]
    pub set_price: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardImage {
    pub id: i64,
    #[serde(rename = "image_url")]
    pub image_url: String,
    #[serde(rename = "image_url_small")]
    pub image_url_small: String,
    #[serde(rename = "image_url_cropped")]
    pub image_url_cropped: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardPrice {
    #[serde(rename = "cardmarket_price")]
    pub cardmarket_price: String,
    #[serde(rename = "tcgplayer_price")]
    pub tcgplayer_price: String,
    #[serde(rename = "ebay_price")]
    pub ebay_price: String,
    #[serde(rename = "amazon_price")]
    pub amazon_price: String,
    #[serde(rename = "coolstuffinc_price")]
    pub coolstuffinc_price: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BanlistInfo {
    #[serde(rename = "ban_tcg")]
    pub ban_tcg: Option<String>,
    #[serde(rename = "ban_ocg")]
    pub ban_ocg: Option<String>,
    #[serde(rename = "ban_goat")]
    pub ban_goat: Option<String>,
}

pub fn read_card_db_from_file<P: AsRef<Path>>(path: P) -> Result<CardDb> {
    // Read the file to a String, handle potential errors with `?`
    let data = fs::read_to_string(path)?;

    // Deserialize the data, handle potential errors with `?`
    let card_db = serde_json::from_str(&data)?;

    Ok(card_db)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_card_db_from_json() {
        let json_filepath = "resources/cards.json";
        let card_db = read_card_db_from_file(json_filepath);
        let total_cards = 12850; // as of Nov 10 2023
        assert_eq!(card_db.unwrap().data.len(), total_cards);
    }
}