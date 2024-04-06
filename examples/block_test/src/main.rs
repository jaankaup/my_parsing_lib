use quick_xml::name::QName;
use json::misc::{
    load_from_file,
    seek_block,
    seek_xml_blocks
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Deck<'a> {
    #[serde(rename = "@version")]
    version: std::borrow::Cow<'a, str>,
    #[serde(rename = "meta")]
    meta: Meta<'a>,
    #[serde(rename = "superzone")]
    super_zone: Option<Vec<SuperZone<'a>>>,
}

#[derive(Deserialize, Debug)]
struct Meta<'a> {
    #[serde(rename = "game")]
    game: std::borrow::Cow<'a, str>,
}

#[derive(Deserialize, Debug)]
struct SuperZone<'a> {
    #[serde(rename = "@name")]
    super_name: std::borrow::Cow<'a, str>,
    #[serde(rename = "card")]
    cards: Option<Vec<Card<'a>>>,
}

#[derive(Deserialize, Debug)]
struct Card<'a> {
    #[serde(rename = "name")]
    name: CardName<'a>,
    #[serde(rename = "set")]
    set: std::borrow::Cow<'a, str>,
}

#[derive(Deserialize, Debug)]
struct CardName<'a> {
    #[serde(rename = "@id")]
    card_id: std::borrow::Cow<'a, str>,
    #[serde(rename = "$value")]
    card_name: std::borrow::Cow<'a, str>,
}

const test_str: &str = r#"
<deck version="0.8">
	<meta>
		<game>magic</game>
	</meta>
	<superzone name="Deck">
		<card><name id="4ed/298">Black Mana Battery</name><set>4e</set></card>
		<card><name id="4ed/319">Fellwar Stone</name><set>4e</set></card>
		<card><name id="4ed/331">Jayemdae Tome</name><set>4e</set></card>
		<card><name id="4ed/343">Red Mana Battery</name><set>4e</set></card>
		<card><name id="ice/335">Shield of the Ages</name><set>ia</set></card>
		<card><name id="ice/350">Zuran Orb</name><set>ia</set></card>
		<card><name id="4ed/302">Brass Man</name><set>4e</set></card>
		<card><name id="4ed/308">Colossus of Sardia</name><set>4e</set></card>
		<card><name id="4ed/188">Earth Elemental</name><set>4e</set></card>
		<card><name id="hml/73">Eron the Relentless</name><set>homelands</set></card>
		<card><name id="4ed/191">Fire Elemental</name><set>4e</set></card>
		<card><name id="4ed/191">Fire Elemental</name><set>4e</set></card>
		<card><name id="4ed/191">Fire Elemental</name><set>4e</set></card>
		<card><name id="4ed/339">Obsianus Golem</name><set>4e</set></card>
		<card><name id="ice/337">Snow Fortress</name><set>ia</set></card>
		<card><name id="ice/337">Snow Fortress</name><set>ia</set></card>
		<card><name id="4ed/228">Uthden Troll</name><set>4e</set></card>
		<card><name id="4ed/228">Uthden Troll</name><set>4e</set></card>
		<card><name id="ice/223">Wall of Lava</name><set>ia</set></card>
		<card><name id="4ed/231">Wall of Stone</name><set>4e</set></card>
		<card><name id="4ed/122">Blight</name><set>4e</set></card>
		<card><name id="4ed/122">Blight</name><set>4e</set></card>
		<card><name id="ice/180">Conquer</name><set>ia</set></card>
		<card><name id="4ed/127">Cursed Land</name><set>4e</set></card>
		<card><name id="4ed/127">Cursed Land</name><set>4e</set></card>
		<card><name id="4ed/127">Cursed Land</name><set>4e</set></card>
		<card><name id="ice/292">Ghostly Flame</name><set>ia</set></card>
		<card><name id="4ed/204">Immolation</name><set>4e</set></card>
		<card><name id="4ed/222">Smoke</name><set>4e</set></card>
		<card><name id="4ed/129">Dark Ritual</name><set>4e</set></card>
		<card><name id="ice/289">Fire Covenant</name><set>ia</set></card>
		<card><name id="ice/289">Fire Covenant</name><set>ia</set></card>
		<card><name id="4ed/194">Fissure</name><set>4e</set></card>
		<card><name id="ice/129">Gravebind</name><set>ia</set></card>
		<card><name id="ice/194">Incinerate</name><set>ia</set></card>
		<card><name id="4ed/205">Inferno</name><set>4e</set></card>
		<card><name id="ice/213">Pyroblast</name><set>ia</set></card>
		<card><name id="4ed/219">Shatter</name><set>4e</set></card>
		<card><name id="4ed/219">Shatter</name><set>4e</set></card>
		<card><name id="4ed/219">Shatter</name><set>4e</set></card>
		<card><name id="ice/224">Word of Blasting</name><set>ia</set></card>
		<card><name id="4ed/373">Mountain</name><set>4e</set></card>
		<card><name id="4ed/373">Mountain</name><set>4e</set></card>
		<card><name id="4ed/373">Mountain</name><set>4e</set></card>
		<card><name id="4ed/373">Mountain</name><set>4e</set></card>
		<card><name id="4ed/373">Mountain</name><set>4e</set></card>
		<card><name id="4ed/373">Mountain</name><set>4e</set></card>
		<card><name id="4ed/373">Mountain</name><set>4e</set></card>
		<card><name id="4ed/373">Mountain</name><set>4e</set></card>
		<card><name id="4ed/373">Mountain</name><set>4e</set></card>
		<card><name id="4ed/373">Mountain</name><set>4e</set></card>
		<card><name id="4ed/373">Mountain</name><set>4e</set></card>
		<card><name id="4ed/373">Mountain</name><set>4e</set></card>
		<card><name id="4ed/373">Mountain</name><set>4e</set></card>
		<card><name id="4ed/373">Mountain</name><set>4e</set></card>
		<card><name id="4ed/373">Mountain</name><set>4e</set></card>
		<card><name id="4ed/363">Strip Mine</name><set>4e</set></card>
		<card><name id="4ed/363">Strip Mine</name><set>4e</set></card>
		<card><name id="4ed/363">Strip Mine</name><set>4e</set></card>
		<card><name id="4ed/370">Swamp</name><set>4e</set></card>
		<card><name id="4ed/370">Swamp</name><set>4e</set></card>
		<card><name id="4ed/370">Swamp</name><set>4e</set></card>
		<card><name id="4ed/370">Swamp</name><set>4e</set></card>
		<card><name id="4ed/370">Swamp</name><set>4e</set></card>
		<card><name id="4ed/370">Swamp</name><set>4e</set></card>
		<card><name id="4ed/370">Swamp</name><set>4e</set></card>
		<card><name id="4ed/370">Swamp</name><set>4e</set></card>
		<card><name id="4ed/370">Swamp</name><set>4e</set></card>
		<card><name id="4ed/370">Swamp</name><set>4e</set></card>
		<card><name id="4ed/370">Swamp</name><set>4e</set></card>
		<card><name id="4ed/370">Swamp</name><set>4e</set></card>
		<card><name id="4ed/370">Swamp</name><set>4e</set></card>
		<card><name id="4ed/370">Swamp</name><set>4e</set></card>
		<card><name id="4ed/119">Ashes to Ashes</name><set>4e</set></card>
		<card><name id="4ed/185">Disintegrate</name><set>4e</set></card>
		<card><name id="4ed/189">Earthquake</name><set>4e</set></card>
		<card><name id="4ed/192">Fireball</name><set>4e</set></card>
		<card><name id="ice/291">Fumarole</name><set>ia</set></card>
		<card><name id="ice/291">Fumarole</name><set>ia</set></card>
		<card><name id="ice/134">Icequake</name><set>ia</set></card>
		<card><name id="ice/134">Icequake</name><set>ia</set></card>
		<card><name id="ice/134">Icequake</name><set>ia</set></card>
		<card><name id="ice/195">Jokulhaups</name><set>ia</set></card>
		<card><name id="ice/198">Lava Burst</name><set>ia</set></card>
		<card><name id="ice/214">Pyroclasm</name><set>ia</set></card>
		<card><name id="ice/214">Pyroclasm</name><set>ia</set></card>
		<card><name id="4ed/217">Pyrotechnics</name><set>4e</set></card>
		<card><name id="ice/161">Soul Burn</name><set>ia</set></card>
		<card><name id="ice/161">Soul Burn</name><set>ia</set></card>
	</superzone>
	<superzone name="Command">
	</superzone>
</deck>
"#;

pub fn main() {
    // let source = load_from_file(String::from("blah.txt")).unwrap(); 
    // let indices = seek_block(&source, '{', '}');
    // println!("indices :: {:?}", indices);

    // let source_xml = load_from_file(String::from("temp.dek")).unwrap(); 
    // let result: Vec<Deck> = seek_xml_blocks(&source_xml, &QName(b"deck"), None);

    let result: Vec<Deck> = seek_xml_blocks(&test_str.to_string(), &QName(b"deck"), None);
    println!("{:?}", result);

    println!("***************************************");

    let result: Vec<Card> = seek_xml_blocks(&test_str.to_string(), &QName(b"card"), None);
    println!("{:?}", result);

    println!("***************************************");

    let result: Vec<SuperZone> = seek_xml_blocks(&test_str.to_string(), &QName(b"superzone"), None);
    println!("{:?}", result);

}
