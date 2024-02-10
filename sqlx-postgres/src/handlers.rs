use crate::{
    model::Movies,
    schema::{CreateMovieSchema, FilterOptions, UpdateMovieSchema},
    AppState,
};
use actix_web::{
    delete, get, patch, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use serde_json::json;

#[get("/ping")]
async fn pingpong() -> impl Responder {
    HttpResponse::Ok().json(json!({"message":"pong", "success":true}))
}

// function to get all movies list
#[allow(non_snake_case)]
#[get("/movies")]
async fn GetAllMovies(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(11);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;
    let query_result = sqlx::query_as!(
        Movies,
        "SELECT * FROM movies ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let message = "Couldn't retrieve all movies";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let movies = query_result.unwrap();
    let res = serde_json::json!({
        "succes":true,
        "results":movies.len(),
        "movies":movies
    });
    return HttpResponse::Ok().json(res);
}

// function to add a new movie to database
#[allow(non_snake_case)]
#[post("/movies")]
async fn AddNewMovie(body: web::Json<CreateMovieSchema>, data: Data<AppState>) -> impl Responder {
    let insert_result = sqlx::query!(
        "INSERT INTO movies (title, description, genre, actors) VALUES ($1, $2, $3, $4) RETURNING *",
        body.title.clone() as String,
        body.description.clone() as String,
        body.genre.clone() as Vec<String>,
        body.actors.clone() as Vec<String>
    )
    .fetch_one(&data.db)
    .await;

    match insert_result {
        Ok(_movie) => {
            let res = serde_json::json!({"success":"true"});
            return HttpResponse::Ok().json(res);
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "fail","message": "Movie with that title already exists"}));
            }

            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

// function to update a movie detail
#[allow(non_snake_case)]
#[patch("/movies/{id}")]
async fn UpdateMovie(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateMovieSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let movie_id = path.into_inner();
    let query_result = sqlx::query_as!(Movies, "SELECT * FROM movies WHERE id = $1", movie_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let message = format!("Movie with ID: {} not found", movie_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail","message": message}));
    }

    let movie = query_result.unwrap();

    let query_result = sqlx::query_as!(
        Movies,
        "UPDATE movies SET title = $1, description = $2, genre = $3, actors = $4 RETURNING *",
        body.title.to_owned().unwrap_or(movie.title) as String,
        body.description.to_owned().unwrap_or(movie.description) as String,
        body.genre.to_owned().unwrap_or(movie.genre.unwrap()) as Vec<String>,
        body.actors.clone().unwrap_or(movie.actors.unwrap()) as Vec<String>
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(_movie) => {
            let res = serde_json::json!({"status": "success"});
            return HttpResponse::Ok().json(res);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": message}));
        }
    }
}

// function to delete a movie
#[allow(non_snake_case)]
#[delete("/movies/{id}")]
async fn DeleteMovie(path: web::Path<uuid::Uuid>, data: web::Data<AppState>) -> impl Responder {
    let movieId = path.into_inner();
    let rows_affected = sqlx::query!("DELETE FROM movies  WHERE id = $1", movieId)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let message = format!("Movie with ID: {} not found", movieId);
        return HttpResponse::NotFound().json(json!({"status": "fail","message": message}));
    }

    HttpResponse::Ok().json(json!({"success":true}))
}

#[allow(non_snake_case)]
pub fn Routerconfig(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(GetAllMovies)
        .service(AddNewMovie)
        .service(UpdateMovie)
        .service(DeleteMovie);
    conf.service(scope);
}
