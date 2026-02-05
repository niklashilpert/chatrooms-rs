use std::{fs, time::SystemTime};

use chrono::Utc;
use mysql::{prelude::Queryable, *};
use serde::{Deserialize, Serialize};

pub struct DbConn {
    conn: PooledConn,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Room {
    pub room_id: i32,
    pub room_name: String,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Message {
    pub message_id: i32,
    pub content: String,
    pub timestamp: String,
    pub room_id: i32,
    pub user_id: i32,
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
        let conn = pool.get_conn().unwrap();
        DbConn { conn }
    }


    pub fn get_user(&mut self, id: i32) -> Option<User> {
        let query = format!("SELECT user_id, username, first_name, last_name FROM users WHERE user_id = {}", id);
        let mut users = self.conn.query_map(query, 
            |(user_id, username, firstname, lastname)| 
            (User {
                user_id,
                username,
                first_name: firstname,
                last_name: lastname,
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

    pub fn get_messages_by_room(&mut self, id: i32) -> Vec<Message> {
        let query = format!("SELECT * FROM messages WHERE room_id = {}", id);
        
        let vec: Vec<(i32, String, String, i32, i32)> = self.conn.query(query).unwrap();

        let mut message_vec = Vec::new();
        for row in vec {
            message_vec.push(Message {
                message_id: row.0,
                content: row.1,
                timestamp: row.2,
                user_id: row.3,
                room_id: row.4,
            })
            
        }

        message_vec
    }

    pub fn get_user_by_credentials(&mut self, username: &str, password: &str) -> Option<User> {
        let query = format!("SELECT user_id FROM users WHERE username = '{}' AND password = '{}'", username, password);
        let id = self.conn.query_first(query).ok()?;
        return self.get_user(id.unwrap());
    }

    pub fn insert_message(&mut self, content: &str, room: &Room, user: &User) {
        let query = format!(
            "INSERT INTO messages (content, timestamp, user_id, room_id) VALUES (:c, :t, :u, :r)", 
        );

        _ = self.conn.exec_drop(query, params! {
            "c" => content,
            "t" => format!("{}", Utc::now().to_rfc3339().replace("+00:00", "")),
            "u" => user.user_id,
            "r" => room.room_id,
        }).unwrap();
    }

    pub fn get_newest_message(&mut self, room_id: i32) -> Option<Message> {
        let query = format!(
            "SELECT message_id, content, timestamp, u.user_id, r.room_id FROM messages m INNER JOIN rooms r ON r.room_id = m.room_id INNER JOIN users u ON u.user_id = m.user_id WHERE r.room_id = {} ORDER BY timestamp DESC LIMIT 1", 
            room_id);

        let mut messages: Vec<Message> = self.conn.query_map(query, |(message_id, content, timestamp, user_id, room_id)| {
            Message {
                message_id,
                content,
                timestamp,
                room_id,
                user_id
            }
        }).ok()?;
        
        if messages.len() > 0 {
            Some(std::mem::take( &mut messages[0]))
        } else {
            None
        }
        
    }

}
