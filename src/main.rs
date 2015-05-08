extern crate iron;
extern crate urlencoded;
extern crate router;
extern crate smtp;

use iron::prelude::*;
use iron::status;
use urlencoded::UrlEncodedQuery;
// use urlencoded::UrlEncodedBody;
use router::Router;
use smtp::sender::{Sender, SenderBuilder};
use smtp::mailer::EmailBuilder;

// const SEND_DOMAIN: &'static str = "@aerofs.com";

fn main() {
    println!("Hello");
    let mut router = Router::new();

    router.get("/", hello_world);
    router.post("/:address", post_email);

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let payload = "Hi!";
        Ok(Response::with((status::Ok, payload)))
    }

    // Receive a message by POST and play it back.
    fn post_email(request: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        let mut email = "form".to_string();
        // TODO: email address chosen via URL route
        //println!("{}", request.extensions.get::<Params>());

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

        // TODO: handle POST bodies, if form works that way instead
        // match request.get_ref::<UrlEncodedBody>() {
        //     Ok(ref hashmap) => println!("Parsed POST request body:\n {:?}", hashmap),
        //     Err(ref e) => println!("{:?}", e)
        // };

        // TODO: Make email address a function of a config var + routing param
        let mut recv_email = "karen@aerofs.com";
        let mut email_builder = EmailBuilder::new();
        email_builder = email_builder.to(recv_email);
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