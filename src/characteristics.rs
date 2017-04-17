use rusqlite::Connection;
use rusqlite::Error;
use std::fmt;

#[derive(Debug)]
pub struct Characteristics {
    pub entity_id: u32,
    pub strength: u8,
    pub dexterity: u8,
    pub endurance: u8,
    pub intelligence: u8,
    pub education: u8,
    pub social: u8,
}

pub enum Values {
    Strength,
    Dexterity,
    Endurance,
    Intelligence,
    Education,
    Social,
}

impl fmt::Display for Values {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Values::Strength => "Strength",
            Values::Dexterity => "Dexterity",
            Values::Endurance => "Endurance",
            Values::Intelligence => "Intelligence",
            Values::Education => "Education",
            Values::Social => "Social",
        };
        write!(f, "{}", printable)
    }
}

impl Characteristics {
    pub fn insert(&self, conn: &Connection) {
        conn.execute("INSERT INTO characteristics (strength, dexterity, endurance, intelligence, education, social)
                  VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                 &[&self.strength, &self.dexterity, &self.endurance, &self.intelligence, &self.education, &self.social]).unwrap();
    }

    pub fn create_tables(conn: &Connection) {
        conn.execute("CREATE TABLE characteristics (
                  entity_id       INTEGER PRIMARY KEY,
                  strength        INTEGER NOT NULL,
                  dexterity       INTEGER NOT NULL,
                  endurance       INTEGER NOT NULL,
                  intelligence    INTEGER NOT NULL,
                  education       INTEGER NOT NULL,
                  social          INTEGER NOT NULL
                  )",
                     &[])
            .unwrap();
    }


    pub fn get_all(conn: &Connection) -> Result<Vec<Characteristics>, Error> {
        let mut stmt = try!(conn.prepare("SELECT entity_id, strength, dexterity, endurance, intelligence, education, social FROM characteristics"));

        let rows = try!(stmt.query_map(&[], |row| {
            Characteristics {
                entity_id: row.get(0),
                strength: row.get(1),
                dexterity: row.get(2),
                endurance: row.get(3),
                intelligence: row.get(4),
                education: row.get(5),
                social: row.get(6),
            }
        }));

        let mut result = Vec::new();
        for ch in rows {
            result.push(try!(ch));
        }

        Ok(result)
    }
}
