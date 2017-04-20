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

pub fn characteristic_modifier(num: u8) -> i8 {
    match num {
        x if x >= 0 && x <= 2 => -2,
        x if x >= 3 && x <= 5 => -1,
        x if x >= 6 && x <= 8 => 0,
        x if x >= 9 && x <= 11 => 1,
        x if x >= 12 && x <= 14 => 2,
        x if x >= 15 && x <= 17 => 3,
        x if x >= 18 && x <= 20 => 4,
        x if x >= 21 && x <= 23 => 5,
        x if x >= 24 && x <= 26 => 6,
        x if x >= 27 && x <= 29 => 7,
        x if x >= 30 && x <= 32 => 8,
        _ => 9
    }
}

impl fmt::Display for Characteristics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use pseudo_hex::num_to_pseudo_hex;
        let chars = vec![self.strength ,
                         self.dexterity,
                         self.endurance,
                         self.intelligence,
                         self.education,
                         self.social];

        let upp = chars.iter().map(|&n| num_to_pseudo_hex(n)).collect::<String>();
        write!(f, "{}", upp)
    }
}

pub enum Characteristic {
    Strength,
    Dexterity,
    Endurance,
    Intelligence,
    Education,
    Social,
}

impl fmt::Display for Characteristic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Characteristic::*;
        let printable = match *self {
            Strength => "Strength",
            Dexterity => "Dexterity",
            Endurance => "Endurance",
            Intelligence => "Intelligence",
            Education => "Education",
            Social => "Social",
        };
        write!(f, "{}", printable)
    }
}

impl Characteristics {
    pub fn insert(&self, conn: &Connection) {
        conn.execute("INSERT INTO characteristics (strength, dexterity, endurance, intelligence, education, social)
                  Characteristic (?1, ?2, ?3, ?4, ?5, ?6)",
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

#[test]
fn test_characteristics_display() {
    let char1 =  Characteristics {
        entity_id: 0,
        strength: 1,
        dexterity: 2,
        endurance: 3,
        intelligence: 4,
        education: 5,
        social: 6,
    };

    let char1Fmt = format!("{}", char1);
    assert_eq!(char1Fmt, "123456");

        let char2 =  Characteristics {
        entity_id: 0,
        strength: 10,
        dexterity: 11,
        endurance: 12,
        intelligence: 11,
        education: 10,
        social: 9,
    };

    let char2Fmt = format!("{}", char2);
    assert_eq!(char2Fmt, "ABCBA9");

}