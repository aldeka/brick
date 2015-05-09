# Brick

This package is a Rust implementation of a webserver that takes a POST from an arbitrary webform and, based on the URL parameters, forwards it as an email to an email address or alias.

It is named as such because this service is, more or less intentionally, dumb as a brick.

There are a *lot* of TODOs left, including parameters that only work on the AeroFS VPN and dev environment. See src/main.rs. Someday this package may be reusable and useful. Till then, this is my first time writing anything in Rust, or any systems language for that matter. <3

![I have no idea what I'm &mut ing](https://i.imgur.com/wGjc785.jpg)

## To run

Modify `MY_DOMAIN` in main.rs to whatever your mail domain name is.

```$ cargo run```

Check the server is running by visiting `localhost:3000` in your web browser, or via curl.

Run the below, only swapping `/contact` for whatever local part mailbox you want the mail to go to. So if your domain was awesome.horse, CURLing `/sales` will email `sales@awesome.horse`.

```$ curl -X POST -d '' "http://localhost:3000/sales?first_name=Jane&last_name=Sagan&email=jane%40testing.com&org_name=&org_size=&title=&comment=This%20is%20a%20test"```

You should see a "Email sent successfully" message in the window running brick, and an email should soon arrive at sales@awesome.horse.
