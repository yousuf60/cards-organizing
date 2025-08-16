pub type CardRow = [String;4];
pub type CardTable = [Vec<String>;4];

#[derive(Debug)]
pub struct Card{
	pub	name: String,
	pub	id: usize,
	pub	favorite_food: String,
	pub	expire_date: String
}



