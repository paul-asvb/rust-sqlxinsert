#[derive(Default, Debug, std::cmp::PartialEq, sqlx::FromRow)]
struct Car {
    pub id: i32,
    pub name: String,
    pub color: Option<String>,
}
#[derive(Default, Debug, sqlx::FromRow, sqlxinsert::PqInsert)]
struct CreateCar {
    pub name: String,
    pub color: Option<String>,
}
impl CreateCar {
    pub fn new<T: Into<String>>(name: T) -> Self {
        CreateCar {
            name: name.into(),
            color: None,
        }
    }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let url = "postgres://postgres@localhost:5432/postgres";
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&url)
        .await
        .unwrap();
    let car_skoda = CreateCar::new("Skoda");
    car_skoda.insert::<Car>(&pool, "cars").await?;
    Ok(())
}
