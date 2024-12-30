// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.
pub mod filters;
pub mod handlers;
pub mod models;

#[cfg(test)]
mod tests {
    use warp::http::StatusCode;
    use warp::test::request;
    use super::{filters, models};
    use std::collections::HashSet;

    #[tokio::test]
    async fn try_list() {
        use std::str;
        use serde_json;

        let simulation1 = models::Simulation {
            id: 1,
            name: String::from("Test 1")
        };
        let simulation2 = models::Simulation {
            id: 2,
            name: String::from("Test 2")
        };
        let db = models::new_db();
        db.lock().await.insert(simulation1.clone());
        db.lock().await.insert(simulation2.clone());

        let api = filters::list_sims(db);
        let response = request()
            .method("GET")
            .path("/holodeck")
            .reply(&api)
            .await;

        let result: Vec<u8> = response.into_body().into_iter().collect();
        let result = str::from_utf8(&result).unwrap();
        let result: HashSet<models::Simulation> = serde_json::from_str(result).unwrap();
        assert_eq!(models::get_simulation(&result, 1).unwrap(), &simulation1);
        assert_eq!(models::get_simulation(&result, 2).unwrap(), &simulation2);

        let response = request()
            .method("GET")
            .path("/holodeck/2")
            .reply(&api)
            .await;

        let result: Vec<u8> = response.into_body().into_iter().collect();
        let result = str::from_utf8(&result).unwrap();
        let result: HashSet<models::Simulation> = serde_json::from_str(result).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(models::get_simulation(&result, 2).unwrap(), &simulation2);
    }

    #[tokio::test]
    async fn try_create() {
        let db = models::new_db();
        let api = filters::post_sim(db);
        let response = request()
            .method("POST")
            .path("/holodeck")
            .json(&models::Simulation {
                id: 1,
                name: String::from("The Big Goodbye")
            })
            .reply(&api)
            .await;

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn try_create_duplicates() {
        let db = models::new_db();
        let api = filters::post_sim(db);
        let response = request()
            .method("POST")
            .path("/holodeck")
            .json(&models::Simulation {
                id: 1,
                name: String::from("The Big Goodbye")
            })
            .reply(&api)
            .await;
        assert_eq!(response.status(), StatusCode::CREATED);
        let response = request()
            .method("POST")
            .path("/holodeck")
            .json(&models::Simulation {
                id: 1,
                name: String::from("The Big Goodbye")
            })
            .reply(&api)
            .await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn try_update() {
        let db = models::new_db();
        let api = filters::update_sim(db);

        let response = request()
            .method("PUT")
            .path("/holodeck/1")
            .json(&models::Name { name: String::from("Name") })
            .reply(&api)
            .await;
        assert_eq!(response.status(), StatusCode::CREATED);

        let response = request()
            .method("PUT")
            .path("/holodeck/1")
            .json(&models::Name { name: String::from("New name") })
            .reply(&api)
            .await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn try_delete() {
        let simulation = models::Simulation{
            id: 1, 
            name: String::from("The Big Goodbye!"),
        };

        let db = models::new_db();
        db.lock().await.insert(simulation);

        let api = filters::delete_sim(db);

        let response = request()
            .method("DELETE")
            .path("/holodeck/1")
            .reply(&api)
            .await;

            assert_eq!(response.status(), StatusCode::OK);
    }
}
