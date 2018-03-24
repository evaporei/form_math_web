extern crate iron;
#[macro_use] extern crate mime;
extern crate router;
extern crate urlencoded;

extern crate math_utils;

use math_utils::gcd;
use iron::prelude::*;
use iron::status;

use router::Router;

use urlencoded::UrlEncodedBody;

use std::fs::File;
use std::io::prelude::*;

use std::str::FromStr;

fn main() {
    const PORT: u16 = 3000;
    let url = format!("localhost:{}", PORT);

    let mut router = Router::new();

    router.get("/", get_index, "root_get");
    router.get("/gcd", get_gcd, "gcd_get");
    router.post("/gcd", post_gcd, "gcd_post");

    println!("Serving on url {}...", url);
    Iron::new(router).http(url)
        .expect("failed to start iron server");
}

fn get_index(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let mut index_html = String::new();

    let mut file = File::open("./pages/index.html")
        .expect("html to exist on pages folder");

    file.read_to_string(&mut index_html).unwrap();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(index_html);

    Ok(response)
}

fn get_gcd(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let mut gcd_html = String::new();

    let mut file = File::open("./pages/gcd/form.html")
        .expect("html to exist on pages folder");

    file.read_to_string(&mut gcd_html).unwrap();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(gcd_html);

    Ok(response)
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        },
        Ok(map) => map,
    };

    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' parameter\n"));
            return Ok(response);
        },
        Some(nums) => nums,
    };

    let mut numbers = Vec::new();
    let mut numbers_str = String::from("");
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(
                    format!("Value for 'n' parameter not a number: {:?}\n",
                            unparsed));
                return Ok(response);
            },
            Ok(n) => {
                numbers.push(n);
                numbers_str.push_str(unparsed);
                numbers_str.push(' ');
            },
        }
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    let mut gcd_template = String::new();

    let mut file = File::open("./pages/gcd/result.template.html")
        .expect("html to exist on pages folder");

    file.read_to_string(&mut gcd_template).unwrap();

    let gcd_template = gcd_template.replace("{:?}", &numbers_str[..]);
    let gcd_template = gcd_template.replace("{}", &d.to_string()[..]);

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(gcd_template);

    Ok(response)
}
