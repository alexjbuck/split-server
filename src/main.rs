#[macro_use] extern crate rocket;
use split_server::dataset::read_dataset;

#[get("/")]
fn index() -> String {
    match read_dataset() {
       Ok(dataset) => format!("{:?}",dataset).to_string(),
       Err(why) => format!("{:?}",why)
    }
}

#[get("/split/<name>")]
fn split(name: &str) -> String {
    println!("Requested the split of {}",name);
    match read_dataset() {
        Ok(dataset) => format!("{:?}",dataset.get_splits_for(name)),
        _ => "Failed to load datasets.".into()
    }
}

#[get("/list/<split>")]
fn list(split: &str) -> String {
    println!("Requested the names in {:?}",split);
    match read_dataset() {
        Ok(dataset) => format!("{:?}",dataset.get_names_for(split.into())),
        _ => "Failed to load datasets.".into()
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/",routes![index])
        .mount("/",routes![split])
        .mount("/",routes![list])
}
