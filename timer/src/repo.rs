use postgres::{NoTls};

pub struct Client {
    client: postgres::Client
}

pub type Result<T> = std::result::Result<T, postgres::Error>;

pub type Timestamp = i32;

impl Client{
    pub fn new(params: &str)->Result<Self> {
        let client = postgres::Client::connect(params, NoTls)?;
        Ok(Client{client})
    }

    pub fn migration(&mut self) -> Result<()>{
        self.client.batch_execute("
        CREATE TABLE IF NOT EXISTS timer (
            id      SERIAL PRIMARY KEY,
            timestamp INTEGER UNIQUE
         )")
    }

    pub fn insert(&mut self, time: Timestamp) -> Result<()> {
        self.client.execute("INSERT INTO timer (timestamp) VALUES ($1) ON CONFLICT (timestamp) DO NOTHING", &[&time])?;
        Ok(())
    }

    pub fn get_last(&mut self) -> Result<Timestamp> {
        let res = self.client.query_one("SELECT MAX(timestamp) FROM timer", &[])?;
        Ok(res.get(0))
    }
}