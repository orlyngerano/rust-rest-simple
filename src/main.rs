use actix_web::{server, App, HttpRequest, Result, Json, Path};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use lazy_static::lazy_static;
use std::sync::Mutex;

// should replace database for storing users
lazy_static! {
    static ref users: Mutex<Vec<Users>> = Mutex::new({
        let mut us:Vec<Users> = Vec::new();
        us
    });
}


#[derive(Serialize, Clone)]
struct Users{
    id: u32,
    name: String
}

#[derive(Serialize)]
struct UsersOut{
    message: String,
    users: Vec<Users>
}

#[derive(Serialize)]
struct UserOut{
    message: String,
    user: Users
}

fn user_handler(_req: &HttpRequest) -> Result<Json<UsersOut>>{
    let request_method =  _req.method().as_str();
    let users_out = match request_method{
        "GET" => {
            UsersOut{
                message: String::from("Ok"),
                users: users.lock().unwrap().to_vec()
            }
        },
        "POST" => {

            //tbd add

            UsersOut{
                message: String::from("Ok"),
                users: users.lock().unwrap().to_vec()
            }
        },        
        _ => {
            UsersOut{
                message: String::from("Ok"),
                users: vec![]
            }
        }
    };
    Ok(Json(users_out))
}

fn user_handler_id(_req: &HttpRequest) -> Result<Json<UserOut>>{
    let id:u32 = _req.match_info().query("id")?;
    let request_method =  _req.method().as_str();
    let mut uv = users.lock().unwrap();
    let pos = uv.iter().position(|x| x.id == id);


    let user_out = match request_method{
        "GET" => {
            
            UserOut{
                message: String::from("Ok"),
                user: uv[pos.unwrap() as usize].clone()
            }
        },
        "UPDATE" =>  {
           //tbd update
           UserOut{
                message: String::from("Ok"),
                user: uv[id as usize].clone()
            }
        },
        "DELETE" =>  {
           uv.remove(pos.unwrap() as usize);
           UserOut{
                message: String::from("Ok"),
                user: Users{id: 0, name: String::from("")}
            }
        },
        _ =>  {
           UserOut{
                message: String::from("Ok"),
                user: Users{id: 0, name: String::from("")}
            }
        }
    };

    Ok(Json(user_out))
}

fn main() {

    users.lock().unwrap().push(Users{id: 1, name: String::from("Orlyn 1")});
    users.lock().unwrap().push(Users{id: 2, name: String::from("Orlyn 2")});

    server::new(|| App::new()
    .resource("/user/{id}", |r| r.f(user_handler_id))
    .resource("/user", |r| r.f(user_handler)))
    .bind("127.0.0.1:8088")
    .unwrap()
    .run();
}
