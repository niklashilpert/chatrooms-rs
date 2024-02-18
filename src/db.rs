use std::fs;

use mysql::{prelude::Queryable, *};
use serde::{Deserialize, Serialize};

pub struct DbConn {
    conn: PooledConn,
}

#[derive(Default, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
}
#[derive(Default, Serialize, Deserialize)]
pub struct Room {
    pub room_id: i32,
    pub room_name: String,
}

pub struct Message {
    pub message_id: i32,
    pub content: String,
    pub timestamp: String,
    pub room: Room,
    pub user: User,
}


pub fn get_db_credentials() -> (String, String) {
    let mut lines: Vec<String> = fs::read_to_string("db.txt").unwrap().lines().map(|s| s.trim().to_string()).collect();
    (std::mem::take(&mut lines[0]) ,std::mem::take(&mut lines[1]))
}

impl DbConn {
    pub fn new() -> Self {
        let (username, password) = get_db_credentials();
        let url = format!("mysql://{}:{}@localhost:3306/chatroomrs", username, password);
        let pool = Pool::new(url).unwrap();
        let mut conn = pool.get_conn().unwrap();
        DbConn { conn }
    }


    pub fn get_user(&mut self, id: i32) -> Option<User> {
        let query = format!("SELECT user_id, username, first_name, last_name FROM users WHERE user_id = {}", id);
        let mut users = self.conn.query_map(query, 
            |(user_id, username, firstname, lastname)| 
            (User {
                id: user_id,
                username,
                firstname,
                lastname,
            })
        ).ok()?;
    
        return if users.len() >= 1 { Some(std::mem::take(&mut users[0])) } else { None }
    }

    pub fn get_room(&mut self, id: i32) -> Option<Room> {
        let query = format!("SELECT * FROM rooms WHERE room_id = {}", id);
        let mut rooms = self.conn.query_map(query, 
            |(room_id, room_name)| Room { room_id, room_name }
        ).ok()?;
        
        return if rooms.len() >= 1 { Some(std::mem::take(&mut rooms[0])) } else { None }
    }

    pub fn get_rooms(&mut self) -> Vec<Room> {
        let query = String::from("SELECT * FROM rooms");
        return self.conn.query_map(query, 
            |(room_id, room_name)| Room { room_id, room_name }
        ).unwrap_or(Vec::new());
    }

    pub fn get_messages_by_room(&mut self, id: i32) -> Option<Vec<Message>> {
        let query = format!("SELECT * FROM messages WHERE room_id = {}", id);
        
        let vec: Vec<(i32, String, String, i32, i32)> = self.conn.query(query).unwrap();

        let mut message_vec = Vec::new();
        for row in vec {
            message_vec.push(Message {
                message_id: row.0,
                content: row.1,
                timestamp: row.2,
                user: self.get_user(row.3).unwrap(),
                room: self.get_room(row.4).unwrap(),
            })
            
        }

        Some(message_vec)
    }

    pub fn get_user_by_credentials(&mut self, username: String, password: String) -> Option<User> {
        let query = format!("SELECT user_id FROM users WHERE username = '{}' AND password = '{}'", username, password);
        let id = self.conn.query_first(query).ok()?;
        return self.get_user(id.unwrap());
    }

}
