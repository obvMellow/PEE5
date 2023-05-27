use rusqlite::{params, Connection};
use serenity::model::prelude::Message;

pub struct User {
    pub id: u64,
    pub xp: usize,
    pub level: usize,
}

pub fn run(msg: &Message) {
    let xp_gain = msg.content.split(" ").count() * 10;

    let conn = Connection::open("pee5.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS xp (
            id    INTEGER PRIMARY KEY,
            xp    INTEGER NOT NULL,
            level INTEGER NOT NULL
        )",
        [],
    )
    .unwrap();

    let mut user = _get_user(&conn, msg.author.id.0);

    if let Some(mut user) = user {
        user.xp += xp_gain;
        user.level = user.xp / 100;

        update_user(&conn, user);
    } else {
        user = Some(User {
            id: msg.author.id.0,
            xp: xp_gain,
            level: 0,
        });

        insert_user(&conn, user.unwrap());
    }
}

pub fn get_user(id: u64) -> Option<User> {
    let conn = Connection::open("pee5.db").unwrap();

    _get_user(&conn, id)
}

fn _get_user(conn: &Connection, id: u64) -> Option<User> {
    let mut stmt = conn
        .prepare("SELECT id, xp, level FROM xp WHERE id = ?1")
        .unwrap();

    let user_iter = stmt
        .query_map(params![id], |row| {
            Ok(User {
                id: row.get(0)?,
                xp: row.get(1)?,
                level: row.get(2)?,
            })
        })
        .unwrap();

    for user in user_iter {
        if let Ok(user) = user {
            return Some(user);
        }
    }

    None
}

fn update_user(conn: &Connection, user: User) {
    conn.execute(
        "UPDATE xp
        SET xp = ?1, level = ?2
        WHERE id = ?3",
        params![user.xp, user.level, user.id],
    )
    .unwrap();
}

fn insert_user(conn: &Connection, user: User) {
    conn.execute(
        "INSERT INTO xp (id, xp, level)
        VALUES (?1, ?2, ?3)",
        params![user.id, user.xp, user.level],
    )
    .unwrap();
}
