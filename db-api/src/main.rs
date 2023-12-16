use std::str::FromStr;

use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use futures::StreamExt;
use mongodb::{Client, bson::{doc, Document}, bson::oid::ObjectId, options::FindOptions, Database};

use shared::model::operator::Operator;
use shared::model::operator_display::OperatorDisplay;

#[get("/operator_displays/{side}")]
async fn get_operator_displays(db: web::Data<Database>, side: web::Path<u8>) -> impl Responder {
    let side = side.into_inner();
    let collection = db.collection("operators");
    let filter = if side == 1 { Some(doc! {"attacker": true}) } else if side == 2 { Some(doc! {"attacker": false}) } else { None };
    let options = FindOptions::builder().projection(doc! {"_id": 1, "name": 1, "icon_url": 1}).build();
    let query: Result<mongodb::Cursor<OperatorDisplay>, mongodb::error::Error> = collection.find(filter, options).await;
    match query {
        Ok(mut cursor) => {
            let mut result: Vec<OperatorDisplay> = Vec::new();
            while let Some(doc) = cursor.next().await {
                match doc {
                    Ok(operator_display) => result.push(operator_display),
                    Err(err) => return HttpResponse::InternalServerError().body(err.to_string())
                };
            }
            HttpResponse::Ok().json(result)
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

// set route
#[get("/operator_ids/{side}")]
async fn get_operator_ids(db: web::Data<Database>, side: web::Path<u8>) -> impl Responder {
    // get argument from path
    let side = side.into_inner();
    // access correct collection
    let collection = db.collection("operators");
    // set up filter to filter for attackers or defenders (or both - here None for no filter)
    let filter = if side == 1 { Some(doc! {"attacker": true}) } else if side == 2 { Some(doc! {"attacker": false}) } else { None };
    // set up options, so only the _id field is returned (other fields are set up as field_name: 0 implicitly by this)
    let options = FindOptions::builder().projection(doc! {"_id": 1}).build();
    // execute the query
    let query: Result<mongodb::Cursor<Document>, mongodb::error::Error> = collection.find(filter, options).await;
    match query {
        // if no error loop thru the returned cursor, write _id field into a vector (array), then return that as json
        Ok(mut cursor) => {
            let mut result: Vec<ObjectId> = Vec::new();
            while let Some(doc) = cursor.next().await {
                result.push(doc.unwrap().get_object_id("_id").unwrap());
            }
            HttpResponse::Ok().json(result)
        },
        // on error return an InternalServerError with reason
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[get["/operator/{id}"]]
async fn get_operator(db: web::Data<Database>, id:web::Path<String>) -> impl Responder {
    let id = id.into_inner();
    // parse id into ObjectId struct
    let obj_id = mongodb::bson::oid::ObjectId::from_str(&id);
    let id;
    match obj_id {
        Ok(obj_id) => id = obj_id,
        Err(err) => return HttpResponse::BadRequest().body(format!("Could not parse id, error: {}", err))
    };
    // set up typed collection
    let collection: mongodb::Collection<Operator> = db.collection("operators");
    // filter for id
    let filter = Some(doc! {"_id": id});
    // execute query
    let query = collection.find_one(filter, None).await;
    match query {
        Ok(operator) => match operator {
            Some(operator) => HttpResponse::Ok().json(operator),
            None => HttpResponse::NotFound().body(format!("Operator with id {} not found.", id))
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // get db uri from envvars or default
    let mongo_uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://192.168.0.146:27017".into());

    #[cfg(debug_assertions)]
    let debug = true;
    #[cfg(not(debug_assertions))]
    let debug = false;

    // set up server address for actix
    let actix_port = std::env::var("ACTIX_PORT").unwrap_or_else(|_| "6969".into());
    let server_addr = format!("{}:{}", if debug {"192.168.0.146"} else {"0.0.0.0"}, actix_port);

    // initialize the db driver
    let mongo_client = Client::with_uri_str(mongo_uri).await.expect("Failed to connect.");

    // set up the HttpServer
    HttpServer::new(move || {
        App::new()
            // give the handler functions some data - here an an object for db access
            .app_data(web::Data::new(mongo_client.database("ranbow").clone()))
            // register service functions
            .service(get_operator_displays)
            .service(get_operator_ids)
            .service(get_operator)
    // tell the HttpServer where to listen
    }).bind(server_addr)?
        .run()
        .await
}
