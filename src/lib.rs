
use warp::Filter;

pub async fn run(){
 let health_check = warp::path!("health_check")
        .map(warp::reply);

     let routes = warp::get().and(
        health_check,
    );
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
   