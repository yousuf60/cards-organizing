
use duckdb::{params, Params, Result, Connection};
const DATAPATH: &str = "./data/data.dt";

pub use crate::common::Card;
pub use crate::common::CardRow;



fn connect()->Connection{
	Connection::open(DATAPATH).unwrap()
}

fn create()->Result<()>{

	connect().execute_batch(r"
		CREATE OR REPLACE TABLE cards(
			Name varchar,
			Id integer UNIQUE,
			FavoriteFood varchar,
			ExpireDate varchar
	);
	")?;
	Ok(())
}


pub fn insert(dt: CardRow)->Result<()>{
	connect().execute(
			"INSERT INTO cards VALUES (?,?,?,?)",
			params![dt[0], dt[1], dt[2], dt[3]]	//[dt.name, dt.id, dt.favorite_food, dt.expire_date]	
	)?;

	Ok(())
}


pub fn store(list: Vec<CardRow>){
	create().unwrap();
	for i in list.into_iter(){
		println!("{:#?}",i);
		if let Err(e)=insert(i){println!("{:#?}",e);};
		}
}

	
fn query_map(command: String ,params_:impl Params)->Vec<Card>{
	let conn = connect();
	let mut statement = conn.prepare(
			command.as_str()
		).unwrap();
		
	let output_iter= statement.query_map(
						params_,
						|row|{
								Ok(Card{
									name:row.get(0).unwrap(), 
									id:row.get(1).unwrap() , 
									favorite_food: row.get(2).unwrap(), 
									expire_date: row.get(3).unwrap()  
									}) }
					 ).unwrap();
		let mut output:Vec<Card> = vec![];
		for i in output_iter{
			output.push(i.unwrap());
		}
		output		
		
}

pub fn get(id: String)->Vec<Card>{
	let command =
		r"
			SELECT * FROM cards
				WHERE id=?
		".to_string();
	
	query_map(command, params![id])

}


pub fn read_all()->Vec<Card>{
	println!("readallllllll");
	let command = "SELECT name , id , favoritefood , expiredate FROM cards".to_string();
	query_map(command, params![])
			 
	
}

pub fn update(id: String, replacing: [String;2])->Result<bool>{
	println!("{} updating. ..",replacing[0]);
	
	connect().execute(
				&format!("UPDATE cards SET {}=? WHERE id=?",replacing[0]),
				params![replacing[1], id.to_string()]
		)?;
		Ok(true)
		
}
pub fn delete(id: String)->Result<bool>{
	connect().execute(
					r"DELETE FROM cards WHERE id = ?",
					params![id.to_string()]
			)?;
			Ok(true)
}










