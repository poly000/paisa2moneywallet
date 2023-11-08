use anyhow::Result;
use std::{collections::BTreeMap, path::Path};

use paisa2moneywallet::{money_wallet, paisa::PaisaJson};

fn main() -> Result<()> {
    let input = std::env::args().nth(1).unwrap();
    let paisa_data = read_paisa(input)?;
    let output = std::env::args().nth(2).unwrap();

    let mut csv_writer = csv::WriterBuilder::new()
        .quote_style(csv::QuoteStyle::Always)
        .from_path(output)?;

    let account_map = BTreeMap::from_iter(
        paisa_data
            .accounts
            .iter()
            .map(|acc| (acc.super_id, &acc.bank_name)),
    );

    for transaction in paisa_data.expenses {
        let wallet = format!("{}", account_map[&transaction.account_id]);

        let category = format!("{}", paisa_data.categories[transaction.category_id].name);

        let datetime = format!(
            "{}",
            transaction
                .time
                .split_once('.')
                .unwrap()
                .0
                .replace("T", " ")
        );

        let sign = if &transaction.type_field == "expense" {
            "-"
        } else {
            ""
        };
        let money = format!("{sign}{}", transaction.currency);

        let description = format!("{}", transaction.description.unwrap_or_default());

        let money_wallet = money_wallet::MoneyWallet {
            wallet,
            currency: String::from("CNY"),
            category,
            datetime,
            money,
            description,
        };

        csv_writer.serialize(money_wallet)?;
    }

    Ok(())
}

fn read_paisa(path: impl AsRef<Path>) -> Result<PaisaJson> {
    let paisa_json = std::fs::read_to_string(path)?;

    Ok(serde_json::from_str(&paisa_json)?)
}
