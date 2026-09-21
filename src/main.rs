use warp;
use warp::Filter;

#[derive(Eq, Hash, PartialEq)]
enum Routes {
    Home,
    About,
}

fn get_route(route: Routes) -> String {

    match route {
        Routes::Home => {
            "home".to_string()
        },

        Routes::About => {
            "about".to_string()
        }
    }
}

mod hashes {
    use crate::{Routes, get_route};

    pub fn _get() -> String{
        let hash: String = get_route(Routes::Home);
        hash
    }

    pub fn _get_1() -> String {
        let hash: String = get_route(Routes::About);
        hash
    }
}

fn start() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let my_page = warp::path!().map(|| "Home Page");
    let page = warp::path!("hello").map(|| hashes::_get());
    let page1 = warp::path!("about").map(|| hashes::_get_1());

    my_page.or(page).or(page1)
}

#[tokio::main]
async fn main() {
    warp::serve(start()).run(([127, 0, 0, 1], 3030)).await;

    std::process::abort();
}