use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use dotenv::dotenv;
use log::{info, error};


#[tokio::test]
async fn test_connection() {

    dotenv().ok();
    //env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
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
    
}

#[tokio::test]
async fn test_insert() {

    dotenv().ok();
    //env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
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

    let query  = "INSERT INTO gaiacatalog (id,ra,dec,x,y,z) VALUES ($1, $2, $3, $4, $5, $6)  RETURNING id";
    for i in 200..100000{
        let insert_query = sqlx::query(query);
    
        let insert_result = &insert_query.bind(i)
            .bind(123)
            .bind(123)
            .bind(123)
            .bind(123)
            .bind(123)
            .execute(&pool) // fetch_one
            .await;

        match insert_result {
            Ok(rslt) => {
                info!("✓Inserted: {:?}", rslt);
            }
            Err(e) => {
                error!("Error Insert: {}", e.to_string())
            }
        }    
    }
}


#[tokio::test]
async fn test_cartesian_query() -> Result<(),Box<dyn std::error::Error>>{
    dotenv().ok();
    //env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    let result = sqlx::query("SELECT sin(ra) as ra_sin, cos(dec) as dec_sin FROM gaiacatalog where id < 500")
        .fetch_all(&pool) // fetch_one, fetch_optional, fetch_all, fetch
        .await?;

    for row in result{
        let sin: f64 = row.get("ra_sin");
        println!("{}", sin);
    }
    
    Ok(())

}





#[tokio::test]
async fn test_bulk_insert() {

    dotenv().ok();
    //env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
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

    let query  = r"INSERT INTO gaiacatalog (id,ra,dec,x,y,z) 
        SELECT * FROM UNNEST ($1::BIGINT[]
            , $2::DOUBLE PRECISION[]
            , $3::DOUBLE PRECISION[]
            , $4::DOUBLE PRECISION[]
            , $5::DOUBLE PRECISION[]
            , $6::DOUBLE PRECISION[] )      
        RETURNING id";

    let ids: Vec<i32> = (1..1000).collect();
    let ras: Vec<f64> = (1..1000).map(|r| 0.2 * r as f64).collect();
    let decs: Vec<f64> = (1..1000).map(|r| 0.2 * r as f64).collect();
    let xs: Vec<f64> = (1..1000).map(|r| 0.2 * r as f64).collect();
    let ys: Vec<f64> =  (1..1000).map(|r| 0.2 * r as f64).collect();
    let zs: Vec<f64> = (1..1000).map(|r| 0.2 * r as f64).collect();
    
    let insert_query = sqlx::query(query);

    let insert_result = &insert_query.bind(ids)
        .bind(ras)
        .bind(decs)
        .bind(xs)
        .bind(ys)
        .bind(zs)
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

