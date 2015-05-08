extern crate iron;
extern crate router;
extern crate smtp;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::io::Read;
use std::net::TcpStream;
use iron::prelude::*;
use iron::status;
use router::Router;
use smtp::sender::{Sender, SenderBuilder};
use smtp::mailer::EmailBuilder;

fn main() {
    env_logger::init().unwrap();
    info!("Starting up!");
    println!("Hello");
    let mut router = Router::new();

    router.get("/", hello_world);
    router.post("/set", post_email);

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let payload = "Hi!!!!";
        Ok(Response::with((status::Ok, payload)))
    }

    // Receive a message by POST and play it back.
    fn post_email(request: &mut Request) -> IronResult<Response> {
        // let mut payload = String::new();
        // request.body.read_to_string(&mut payload).unwrap();
        // println!("Hi: {}", payload);

        let mut email_builder = EmailBuilder::new();
        email_builder = email_builder.to("karen@aerofs.com");
        let email = email_builder.from("inquiryform@aerofs.com")
                        .body("Hello!")
                        .subject("Bonjour!")
                        .build();
        let sb = SenderBuilder::new(("127.0.0.1",25)).hello_name("localhost")
                        .enable_connection_reuse(false);

        println!("sb data: {:?}", sb.server_addr);
        let mut sender: Sender = sb.build();

        //for _ in (1..5) {
        //    let _ = sender.send(email.clone());
        //}
        let result = sender.send(email);
        sender.close();

        match result {
            Ok(..) => println!("Email sent successfully"),
            Err(error) => println!("What? {:?}", error),
        }
        //result.unwrap();

        return Ok(Response::with((status::Ok, "")));
    }

    Iron::new(router).http("localhost:3000").unwrap();
}