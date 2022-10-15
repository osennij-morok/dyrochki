use actix_web::{HttpServer, App, HttpResponse, get, web, post, HttpRequest, http::header::{self, EntityTag}};
use handlebars::{Handlebars, JsonValue, Template};
use phf::phf_map;
use serde::Deserialize;
use serde_json::json;

use crate::{holes_counter::{self, HolesCountingResult}, i18n};

#[derive(Debug)]
struct ConnectionInfo {
    host: String,
    extern_port: u16,
    protocol: String
}

pub async fn run(host: &str, port: u16, secure: bool, with_reverse_proxy: bool) -> std::io::Result<()> {
    let hbars_ref: web::Data<Handlebars<'static>> = web::Data::new(configure_handlebars());
    let protocol: String = match secure {
        true => "https".to_owned(),
        false => "http".to_owned()
    };
    let connection_info_ref: web::Data<ConnectionInfo> = web::Data::new(ConnectionInfo { 
        host: host.to_owned(), 
        extern_port: extern_port(port, secure, with_reverse_proxy),
        protocol
    });
    HttpServer::new(move || {
        App::new()
            .app_data(hbars_ref.clone())
            .app_data(connection_info_ref.clone())
            .service(count_holes_endpoint)
            .service(index_endpoint)
            .service(get_file)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

#[get("/")]
async fn index_endpoint(hb: web::Data<Handlebars<'_>>, 
                        connection_info: web::Data<ConnectionInfo>) -> HttpResponse {
    HttpResponse::Ok()
        .body(render_index(hb, &connection_info))
}

#[derive(Debug, Deserialize)]
struct CountHolesForm {
    text: String,
}

#[post("/")]
async fn count_holes_endpoint(hb: web::Data<Handlebars<'_>>, 
                              connection_info: web::Data<ConnectionInfo>, 
                              form: web::Form<CountHolesForm>) -> HttpResponse {
    if form.text.trim().is_empty() {
        let msg: String =  i18n::holes::input_text_is_empty_msg();
        let body: String = render_index_with_result(hb, &connection_info,
            &msg, "", "");
        return HttpResponse::Ok().body(body);
    }

    let counting_result: HolesCountingResult = holes_counter::count(&form.text);

    let count_msg: String = i18n::holes::holes_found_msg(counting_result.holes_count);
    let uncounted_chars_msg: String = i18n::holes::uncounted_chars_msg(&counting_result.uncounted_chars);
    let body: String = render_index_with_result(hb, &connection_info, 
        &count_msg, &uncounted_chars_msg, &form.text);
    HttpResponse::Ok().body(body)
}

fn render_index(hb: web::Data<Handlebars<'_>>, connection_info: &ConnectionInfo) -> String {
    let data = json!({
        "host": &connection_info.host,
        "port": connection_info.extern_port,
        "protocol": &connection_info.protocol,
        "textareaContent": "",
        "counting": {
            "isPresent": false
        },
        "ui": ui_json_default_value()
    });
    hb.render("index", &data).unwrap()
}

fn render_index_with_result(hb: web::Data<Handlebars<'_>>, connection_info: &ConnectionInfo, 
                            count_msg: &str, uncounted_chars_msg: &str, prev_input_text: &str) -> String {
    let data = json!({
        "host": &connection_info.host,
        "port": connection_info.extern_port,
        "protocol": &connection_info.protocol,
        "textareaContent": prev_input_text,
        "counting": {
            "isPresent": true,
            "countMsg": count_msg,
            "uncountedCharsMsg": uncounted_chars_msg
        },
        "ui": ui_json_default_value()
    });
    hb.render("index", &data).unwrap()
}

fn ui_json_default_value() -> JsonValue {
    json!({
        "textLabel": i18n::holes::text_label_msg(),
        "title":     i18n::holes::index_title_msg(),
        "submitBtn": i18n::holes::count_holes_btn_msg()
    })
}

fn configure_handlebars() -> Handlebars<'static> {
    let mut hb = Handlebars::new();

    const INDEX_TEMPLATE_STR: &str = include_str!("../../static/templates/index.hbs");
    let index_template = Template::compile(INDEX_TEMPLATE_STR).unwrap();
    hb.register_template("index", index_template);

    hb
}

fn extern_port(intern_port: u16, secure: bool, with_reverse_proxy: bool) -> u16 {
    const EXTERN_PORT_DEFAULT_INSECURE: u16 = 80;
    const EXTERN_PORT_DEFAULT_SECURE: u16 = 443;
    if !with_reverse_proxy {
        return intern_port;
    }
    if secure {
        return EXTERN_PORT_DEFAULT_SECURE;
    }
    return EXTERN_PORT_DEFAULT_INSECURE;
}

#[get("/{filepath:.*}")]
async fn get_file(req: HttpRequest, file_path: web::Path<String>) -> HttpResponse {
    const FILES: phf::Map<&str, (&str, &str)> = phf_map!( // path => (content, etag)
        "css/bulma.min.css" => (
            include_str!("../../static/css/bulma.min.css"),
            env!("BULMA_HASH")
        )
    );
    let (file_content, stored_etag) = match FILES.get(&file_path) {
        Some(entry) => entry,
        None => return HttpResponse::NotFound().finish()
    };
    if let Some(req_etag) = req.headers()
        .get(header::IF_NONE_MATCH)
        .map(|header_value| header_value.to_str().unwrap().replace("\"", "")) 
    {
        if &req_etag == *stored_etag {
            return HttpResponse::NotModified().finish();
        }
    }
    return HttpResponse::Ok()
        .append_header(header::ETag(EntityTag::new_strong((*stored_etag).to_owned())))
        .body(*file_content);
}