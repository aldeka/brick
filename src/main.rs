extern crate iron;
extern crate urlencoded;
extern crate router;
extern crate smtp;
#[macro_use]
extern crate log;

use std::io::Read;
use std::net::TcpStream;
// use std::collections::HashMap;
use iron::prelude::*;
use iron::status;
use urlencoded::UrlEncodedQuery;
use urlencoded::UrlEncodedBody;
// use urlencoded::QueryMap;
use router::Router;
use smtp::sender::{Sender, SenderBuilder};
use smtp::mailer::EmailBuilder;

fn main() {
    info!("Starting up!");
    println!("Hello");
    let mut router = Router::new();

    // ?first_name=Karen&last_name=Rustad&email=karen%40aerofs.com&phone=952-210-6598&org_name=&org_size=&title=&comment=Testing

    router.get("/", hello_world);
    router.post("/sales", post_email);

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let payload = "Hi!!!!";
        Ok(Response::with((status::Ok, payload)))
    }

    // Receive a message by POST and play it back.
    fn post_email(request: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        let mut email = "form".to_string();

        match request.get_ref::<UrlEncodedQuery>() {
            Ok(ref hashmap) => {
                for (name, value) in hashmap.iter() {
                    payload.push_str(name);
                    payload.push_str(": ");
                    payload.push_str(&value[0]);
                    payload.push_str("\n");
                }
                if hashmap.contains_key("email") {
                    email = hashmap["email"][0].clone();
                }
            },
            Err(ref e) => println!("{:?}", e)
        };

        match request.get_ref::<UrlEncodedBody>() {
            Ok(ref hashmap) => println!("Parsed POST request body:\n {:?}", hashmap),
            Err(ref e) => println!("{:?}", e)
        };

        let mut email_builder = EmailBuilder::new();
        email_builder = email_builder.to("karen@aerofs.com");
        let email = email_builder.from("inquiryform@aerofs.com")
                        .body(&payload)
                        .subject(&format!("Inquiry from {}", email))
                        .build();
        let sb = SenderBuilder::new(("devmail.aerofs.com",25)).hello_name("localhost")
                        .enable_connection_reuse(false);

        let mut sender: Sender = sb.build();
        let result = sender.send(email);
        sender.close();

        match result {
            Ok(..) => println!("Email sent successfully"),
            Err(error) => println!("Error: {:?}", error),
        }

        return Ok(Response::with((status::Ok, "")));
    }

    Iron::new(router).http("localhost:3000").unwrap();
}