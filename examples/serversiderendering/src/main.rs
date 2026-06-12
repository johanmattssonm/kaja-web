use actix_web::{get, http::header::ContentType, App, HttpResponse, HttpServer, Responder};
use kaja_html_macro::html;

struct Person {
    name: String,
    age: u32,
}

#[get("/")]
async fn index() -> impl Responder {
    let content = html! {{
        <main>
            <h1>Server Side Rendering Using actix-web and kaja-web</h1>
            <include table() />
        </main>
    }};

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(content)
}

fn table() -> String {
    let persons = vec![
        Person {
            name: "John".to_string(),
            age: 30,
        },
        Person {
            name: "Jane".to_string(),
            age: 25,
        },
        Person {
            name: "Julia".to_string(),
            age: 50,
        },
    ];

    let table = html! {{
        <table>
            <thead>
                <tr>
                    <th>ID</th>
                    <th>Name</th>
                    <th>Age</th>
                </tr>
            </thead>
            <tbody>
                <rust>
                    let mut id = 1;

                    for person in &persons {
                        <markup>
                            <tr>
                                <td>$(id)</td>
                                <td>$(person.name)</td>
                                <td>$(person.age)</td>
                            </tr>
                        </markup>

                        id += 1;
                    }
                </rust>
            </tbody>
        </table>
    }};

    table
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
