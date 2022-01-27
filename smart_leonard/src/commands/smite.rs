use std::collections::HashSet;

use indoc::indoc;
use lazy_static::lazy_static;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use titlecase::titlecase;

lazy_static! {
    static ref GODS: HashSet<&'static str> = hashset! {
        "Achilles",
        "Agni",
        "Ah Muzen Cab",
        "Ah Puch",
        "Amaterasu",
        "Anhur",
        "Anubis",
        "Ao Kuang",
        "Aphrodite",
        "Apollo",
        "Arachne",
        "Ares",
        "Artemis",
        "Artio",
        "Athena",
        "Atlas",
        "Awilix",
        "Baba Yaga",
        "Bacchus",
        "Bakasura",
        "Baron Samedi",
        "Bastet",
        "Bellona",
        "Cabrakan",
        "Camazotz",
        "Cerberus",
        "Cernunnos",
        "Chaac",
        "Chang'e",
        "Charybdis",
        "Chernobog",
        "Chiron",
        "Chronos",
        "Cliodhna",
        "Cthulhu",
        "Cu Chulainn",
        "Cupid",
        "Da Ji",
        "Danzaburou",
        "Discordia",
        "Erlang Shen",
        "Eset",
        "Fafnir",
        "Fenrir",
        "Freya",
        "Ganesha",
        "Geb",
        "Gilgamesh",
        "Guan Yu",
        "Hachiman",
        "Hades",
        "He Bo",
        "Heimdallr",
        "Hel",
        "Hera",
        "Hercules",
        "Horus",
        "Hou Yi",
        "Hun Batz",
        "Izanami",
        "Janus",
        "Jing Wei",
        "Jormungandr",
        "Kali",
        "Khepri",
        "King Arthur",
        "Kukulkan",
        "Kumbhakarna",
        "Kuzenbo",
        "Loki",
        "Medusa",
        "Mercury",
        "Merlin",
        "Morgan Le Fay",
        "Mulan",
        "Ne Zha",
        "Neith",
        "Nemesis",
        "Nike",
        "Nox",
        "Nu Wa",
        "Odin",
        "Olorun",
        "Osiris",
        "Pele",
        "Persephone",
        "Poseidon",
        "Ra",
        "Raijin",
        "Rama",
        "Ratatoskr",
        "Ravana",
        "Scylla",
        "Serqet",
        "Set",
        "Shiva",
        "Skadi",
        "Sobek",
        "Sol",
        "Sun Wukong",
        "Susano",
        "Sylvanus",
        "Terra",
        "Thanatos",
        "The Morrigan",
        "Thor",
        "Thoth",
        "Tiamat",
        "Tsukuyomi",
        "Tyr",
        "Ullr",
        "Vamana",
        "Vulcan",
        "Xbalanque",
        "Xing Tian",
        "Yemoja",
        "Ymir",
        "Zeus",
        "Zhong Kui"
    };
}

#[command]
pub async fn counter(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let god = titlecase(args.message().trim());
    let text = if GODS.contains(&god[..]) {
        format!(
            indoc! {"
            Check out
                - https://smite.fandom.com/wiki/{}#Abilities
                - https://www.google.com/search?q=SMITE+How+to+Counter+{},
            "},
            god.replace(" ", "_"),
            god.replace(" ", "_")
        )
    } else {
        format!("Couldn't find God named {} :(", god)
    };
    msg.reply(ctx, text).await?;
    Ok(())
}
