use rocket::http::{Cookie, Cookies};

extern crate time;

use models::*;

pub fn get_id_from_session(cookies: &Cookies) -> Result<i32, String>{
    if let Some(cookie) =  cookies.find("session") {
        let mut updated_cookie = Cookie::new("session".to_string(), cookie.value.clone());
        updated_cookie.expires = Some(time::now() + time::Duration::minutes(10));
        cookies.add(updated_cookie);
        Ok(cookie.value.parse::<i32>().unwrap())
    }
    else {
        Err(format!("User is not loged in"))
    }
}

#[derive(Debug, Serialize)]
pub struct Context{ user: User, folders: Vec<Folder>, documents: Vec<Document>, folder_name: String }

impl Context {
    /*pub fn err(msg: &'a str) -> Context<'static, 'a> {
        Context{msg: Some(("error", msg)), tasks: Task::all()}
    }

    pub fn raw(msg: Option<(&'a str, &'b str)>) -> Context<'a, 'b> {
        Context{msg: msg, tasks: Task::all()}
    }*/

    pub fn folder_view(user_id: i32, mut folder_name: String) -> Context {
        let user: User = User::get(user_id);
        let folders: Vec<Folder> = (&user).get_folders();
        let folder = folders.clone().into_iter().filter(|folder| folder.name == folder_name).next().unwrap();
        let documents: Vec<Document> = folder.get_documents();
        Context{user: user, folders: folders, documents: documents, folder_name: folder_name}
    }
}