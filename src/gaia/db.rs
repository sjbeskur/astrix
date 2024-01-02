use super::*;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
//use dotenv::dotenv;
//use log::{info, trace};


// id  $1::BIGINT[]
const INSERT_STARS_QUERY: &str  = r"INSERT INTO gaiacatalog (ra,dec,mag,x,y,z) 
SELECT * FROM UNNEST (
      $1::DOUBLE PRECISION[]
    , $2::DOUBLE PRECISION[]
    , $3::DOUBLE PRECISION[]
    , $4::DOUBLE PRECISION[]
    , $5::DOUBLE PRECISION[]
    , $6::DOUBLE PRECISION[] )      
RETURNING id";


/// read catalog file and insert into database
/// 
pub async fn insert_stars(database_url: String, catalog_filename: String){
    println!("Inserting stars into database");
	let reader = super::GaiaFileReader::new(catalog_filename);
	let stars = reader.read_csv().unwrap();
    println!("Inserting stars into database");
    bulk_insert(database_url, stars).await;
}


async fn bulk_insert(database_url: String, stars: Vec<Star>) {
    println!("Inserting stars into database");

    let pool = match sqlx::postgres::PgPool::connect(&database_url).await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };


    for (i, stars) in stars.chunks(1000).enumerate(){
        println!("size of chunk is {:?}", stars.len());
        let ras: Vec<f64> = stars.iter().map(|r| r.right_ascention_rads()).collect();
        let decs: Vec<f64> = stars.iter().map(|r| r.declination_rads()).collect();
        let mags: Vec<f32> = stars.iter().map(|r| r.apparent_magnitude()).collect();
        let xx: Vec<f64> = stars.iter().map(|r| r.to_cartesian().x()).collect();
        let yy: Vec<f64> = stars.iter().map(|r| r.to_cartesian().y()).collect();
        let zz: Vec<f64> = stars.iter().map(|r| r.to_cartesian().z()).collect();

        let insert_query = sqlx::query(INSERT_STARS_QUERY);

        let insert_result = &insert_query
            .bind(ras)
            .bind(decs)
            .bind(mags)
            .bind(xx)
            .bind(yy)
            .bind(zz)
            .execute(&pool) // fetch_one
            .await;

        match insert_result {
            Ok(rslt) => {
                println!("✓Inserted: {:?}", rslt);
            }
            Err(e) => {
                println!("Error Insert: {}", e.to_string())
            }
        }    
    }
}

