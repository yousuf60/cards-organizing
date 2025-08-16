use pyo3::prelude::*;

// name , id , favorite food , expired date

pub use crate::common::Card;
pub use crate::common::CardRow;
pub use crate::common::CardTable;
use  crate::duckduck;



#[pyclass]
#[derive(Debug, Clone)]
pub struct Cards{

	
	pub name: Vec<String>,

	
	pub id: Vec<String>,
	
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

	// clears items and returns the cardtable to user
	fn empty_and_clone(&mut self)-> CardTable{
		let cln = [self.name.clone(), self.id.clone(), self.favorite_food.clone(), self.expire_date.clone()];
		*self = Self::empty();
		cln
	}


	fn convert_read(&mut self, list: Vec<Card>)->CardTable{
			for i in list.into_iter(){
				self.name.push(i.name);			
				self.id.push(i.id.to_string());
				self.favorite_food.push(i.favorite_food);
				self.expire_date.push(i.expire_date);
				}
			self.empty_and_clone()
		}

		

}

#[pymethods]
impl Cards{
	#[new]	
	pub fn new(list: Vec<CardRow>)->Self
		{
		duckduck::create().unwrap();
		duckduck::store(list);
		Self::empty()
	}

	pub fn read(&mut self)->CardTable{
		
		
		self.convert_read(
			duckduck::read_all()
			)
	}
	
	pub fn get(&mut self, id: String)->CardTable{
		
		self.convert_read(duckduck::get(id))
	}
	
	pub fn update(&self, id: String, replacing: [String;2])->bool{
		println!("got {}",replacing[0]);
		duckduck::update(id, replacing).unwrap()
	}
	pub fn delete(&self, id: String)->bool{
		duckduck::delete(id).unwrap()
	}
	pub fn insert(&self, dt: Vec<CardRow>)->Vec<bool>{
		duckduck::store(dt)
	}
	
	

}

