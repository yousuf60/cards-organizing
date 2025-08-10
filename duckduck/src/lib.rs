

use duckdb::{params, Result, Connection};
const DATAPATH: &str = "./data/data.dt";

pub use common::Card;


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


pub fn insert(dt: common::CardItems)->Result<()>{
	let x = connect().execute(
			"INSERT INTO cards VALUES (?,?,?,?)",
			params![dt.0,dt.1,dt.2,dt.3]	
	)?;
	println!("{:#?}", x);
	Ok(())
}


pub fn store(list: Vec<common::CardItems>){
	let _ = create();
	for i in list.into_iter(){
		println!("{:#?}",i);
		insert(i);
		}
}

pub fn read()->Vec<Card>{
	let conn = connect();
	let mut statment = conn.prepare("SELECT name , id , favoritefood , expiredate FROM cards").unwrap();
	let output_iter= statment.query_map(
			[],|row|{
			Ok(Card{
				name:row.get(0).unwrap(), 
				id:row.get(1).unwrap() , 
				favorite_food: row.get(2).unwrap(), 
				expire_date: row.get(3).unwrap()  
				})
					}
		 ).unwrap();
		let mut output:Vec<Card> = vec![];
		for i in output_iter{
			output.push(i.unwrap());
		}
			 
	output
}
