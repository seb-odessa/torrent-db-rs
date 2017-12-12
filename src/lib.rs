/*
CREATE TABLE pieces (
	"id"      INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	"hash"    TEXT NOT NULL UNIQUE
    "length"  INTEGER NOT NULL,
)

CREATE TABLE blocks (
    "id"        INTEGER NOT NULL,
    "piece_id"  INTEGER NOT NULL,
    "index"     INTEGER NOT NULL,
    "data"      BLOB
)
*/

#[derive(PartialEq, Debug, Clone)]
pub struct Piece {
    hash: String,
    length: u64,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Block {
    hash: String,
    index: u32,
    data: Vec<u8>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Message {
    Piece,
    // Block,
    GetBlock(String, u32),
    Block(Vec<u8>),
    None(),
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
