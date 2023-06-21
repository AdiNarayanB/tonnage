use futures::future::FutureExt;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use serde::Deserialize;

//Add to weight lifted duration/incrementTime => So, if duration is 31, and incrementTime is 7, i add to weight lifted 4 times, with incrementValue as 5, so weight lifted moves from 10 => 15 -> 20 -> 25, and the tonnage also moves into those thoe values over time. 
//The goal is to  

fn calculateTonnage(sets: i32, reps: i32,repRange:i32, weightLifted:i32,duration:i32) -> i32 {
	println!("{sets}");
   	let tonnageTotal = 0;
   	sets*reps*weightLifted
   		
   	
}
//http://localhost:8080/sum/{num1}/{num2} => 	http://localhost:8080/TonnageQuery/4/8/12/125/31




#[get("/calculateTonnage")]
async fn calculateTotalTonnage(query: web::Query<TonnageQuery>) -> impl Responder {
    let num_sets = query.sets;
    let num_reps = query.reps;
    let rep_range = query.repRange;
    let weight_lifted = query.weightLifted;
    let duration = query.duration;

    println!("Parameters: sets={}, reps={}, repRange={}, weightLifted={}, duration={}", num_sets, num_reps, rep_range, weight_lifted, duration);

    let result = calculateTonnage(query.sets, query.reps, query.repRange, query.weightLifted,query.duration);
    HttpResponse::Ok().json(totalTonnageResponse { totalTonnage: result })
}

#[derive(serde::Deserialize)]
struct TonnageQuery {
    sets: i32,
    reps: i32,
    repRange: i32,
    weightLifted: i32,
    duration: i32
}

#[derive(serde::Serialize)]
struct totalTonnageResponse {
    totalTonnage: i32,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(calculateTotalTonnage)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
