pub type CardItems = (String, usize, String, String);
pub type CardTable = (Vec<String>, Vec<usize>, Vec<String>, Vec<String>);

#[derive(Debug)]
pub struct Card{
	pub	name: String,
	pub	id: usize,
	pub	favorite_food: String,
	pub	expire_date: String
}

