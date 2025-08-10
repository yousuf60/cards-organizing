use pyo3::prelude::*;

//use pyo3::types::PyDate;

// 
// #[pyclass]
// struct CardsHolder{
	// cards: Cards
// }

// name , id , favorite food , expired date
pub use common::CardItems;


#[pyclass]
#[derive(Debug, Clone)]
pub struct Cards{

	
	pub name: Vec<String>,

	
	pub id: Vec<usize>,
	
	pub favorite_food: Vec<String>,

	pub expire_date: Vec<String>
}


impl Cards{
	pub fn empty()->Cards{
		
		 Cards{
				name: vec![],
				id: vec![],
				favorite_food: vec![],
				expire_date: vec![]
			}
						
	}

}

#[pymethods]
impl Cards{
	#[new]	
	pub fn new(list: Vec<CardItems> )->Self
		{
		duckduck::store(list);
		Self::empty()
	}


	
	pub fn read(&mut self)->common::CardTable{
		
		let list = duckduck::read();
		for i in list.into_iter(){
			println!("{}",i.name);	
			self.name.push(i.name);
		
			self.id.push(i.id);
			self.favorite_food.push(i.favorite_food);
			self.expire_date.push(i.expire_date);
			}
		
		(self.name.clone(),self.id.clone(),self.favorite_food.clone(),self.expire_date.clone())
	}




}

