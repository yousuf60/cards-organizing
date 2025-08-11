use pyo3::prelude::*;

//use pyo3::types::PyDate;

// 
// #[pyclass]
// struct CardsHolder{
	// cards: Cards
// }

// name , id , favorite food , expired date
pub use common::CardRow;


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

	// clears items and returns the cardtable to user
	fn empty_and_clone(&mut self)-> common::CardTable{
		let cln = (self.name.clone(), self.id.clone(), self.favorite_food.clone(), self.expire_date.clone());
		*self = Self::empty();
		cln
	}


	fn convert_read(&mut self, list: Vec<common::Card>)->common::CardTable{
			for i in list.into_iter(){
				println!("{}",i.name);	
				self.name.push(i.name);
			
				self.id.push(i.id);
				self.favorite_food.push(i.favorite_food);
				self.expire_date.push(i.expire_date);
				}
			self.empty_and_clone()
		}

		

}

#[pymethods]
impl Cards{
	#[new]	
	pub fn write(list: Vec<CardRow>)->Self
		{
		duckduck::store(list);
		Self::empty()
	}

	pub fn read(&mut self)->common::CardTable{
		
		
		self.convert_read(
			duckduck::read_all()
			)
	}
	
	pub fn get(&mut self, id: usize)->common::CardTable{
		
		self.convert_read(duckduck::get(id))
	}
	
	pub fn update(&self, id: usize, replacing: [String;2])->bool{
		duckduck::update(id, replacing).unwrap()
	}
	pub fn delete(&self, id: usize)->bool{
		duckduck::delete(id).unwrap()
	}
	

}

