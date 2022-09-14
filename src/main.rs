#[macro_use] //Para usar las macro creada
extern crate diesel; //Extendible de diesel
extern crate tera;


pub mod schema; //Usando los datos del squema 
pub mod models; //Usando los datos de los modelos   

use std::env;
use dotenv::dotenv; // Import the dotenv library

use tera::{Tera, Template}; // Import the tera library Tera
use diesel::prelude::*; // Import the Diesel prelude
use diesel::pg::PgConnection; // Import the Diesel postgres connection

use diesel::r2d2::{self, ConnectionManager}; //Import the r2d2 connection manager
use diesel::r2d2::Pool; //Import the r2d2 pool multi conexiones para la data base


use actix_web::{get,post,web, App, HttpServer,HttpResponse, Responder};//Usando Actix web

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>; //Definir el tipo de la conexion


use self::models::{
    LayoutProfile,NewLayoutprofile,
    LayoutPost,NewLayoutpost,Pprofile,NewParrafoProfile,
    Skilss,NewSkill,
    LayoutService,NewLayoutService,
    Pservice,NewParrafoService,
    Pindex,NewPindex,
    Symbol,NewSymbol,
    Carousel,NewCarousel,
    Msg,NewMsg,PContact,NewPcontact,
    Social,NewSocial,
    Service,NewService,
    Profile,NewProfile,
    Post,NewPost,NewPostHandler};

use self::schema::posts;
use self::schema::posts::dsl::*;

use self::schema::profiles;
use self::schema::profiles::dsl::*;

use self::schema::services;
use self::schema::services::dsl::*;

use self::schema::socials;
use self::schema::socials::dsl::*;

use self::schema::msg;
use self::schema::msg::dsl::*;

use self::schema::carousel;
use self::schema::carousel::dsl::*;

use self::schema::symbol;
use self::schema::symbol::dsl::*;

use self::schema::pindex;
use self::schema::pindex::dsl::*;

use self::schema::pservice;
use self::schema::pservice::dsl::*;

use self::schema::layoutservice;
use self::schema::layoutservice::dsl::*;

use self::schema::layoutprofile;
use self::schema::layoutprofile::dsl::*;

use self::schema::pprofile;
use self::schema::pprofile::dsl::*;

use self::schema::skills;
use self::schema::skills::dsl::*;

use self::schema::layoutpost;
use self::schema::layoutpost::dsl::*;

use self::schema::p_contact;
use self::schema::p_contact::dsl::*;

//********************************************* Connect data *********************************************
//Start Funcion conectar a la base de datos
fn conectar_bd() -> PgConnection {
    dotenv().ok(); //Load the .env file
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set"); //Get the DATABASE_URL environment variable
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
//Final Funcion conectar a la base de datos
//********************************************* end data base*********************************************


//*********************************************Consultas Select Template Index*********************************************
//Start Funcion Seleccionar todos los Carousel
fn query_select_all_carousel(){
    println!("Query Select All Initial Carousel");
    let conn =  conectar_bd(); //Conectamos a la base de datos
    //Select * from posts
    let carousel_result = carousel.order(id_carousel).load::<Carousel>(&conn).expect("Error al ejecutar la query");
    for _carousel in carousel_result {
        println!("
        id_carousel:{} 
        title_carousel_1:{}
        title_carousel_2:{}
        title_carousel_3:{}
        p_carousel_1:{}
        p_carousel_2:{} 
        p_carousel_3:{}  
        img_carousel_1:{}
        img_carousel_2:{}
        img_carousel_3:{}
        href_carousel_1:{}
        href_carousel_2:{} 
        href_carousel_3:{}  
        ",
        _carousel.id_carousel, 
        _carousel.title_carousel_1,
        _carousel.title_carousel_2,
        _carousel.title_carousel_3,
        _carousel.p_carousel_1,
        _carousel.p_carousel_2, 
        _carousel.p_carousel_3,  
        _carousel.img_carousel_1,
        _carousel.img_carousel_2,
        _carousel.img_carousel_3,
        _carousel.href_carousel_1,
        _carousel.href_carousel_2,
        _carousel.href_carousel_3,
        );
    }
}
//Final Funcion Seleccionar todos los Carousel

//Start Funcion Seleccionar todos los Symbol
fn query_select_all_symbol(){
    println!("Query Select All Initial Symbol");
    let conn =  conectar_bd(); //Conectamos a la base de datos
    //Select * from posts
    let symbol_result = symbol.order(id_symbol).load::<Symbol>(&conn).expect("Error al ejecutar la query");
    for _symbol in symbol_result {
        println!("
        id_symbol:{} 
        name_symbol:{}
        description_symbol:{}
        img_symbol:{}
        href_symbol:{}
        ",
        _symbol.id_symbol, 
        _symbol.title_symbol,
        _symbol.p_symbol,
        _symbol.img_symbol,
        _symbol.href_symbol,
        );
    }
}
//Final Funcion Seleccionar todos los Symbol

//Start Funcion Seleccionar todos los Parrafo index
fn query_select_all_p_index(){
    println!("Query Select All Initial P Index");
    let conn =  conectar_bd(); //Conectamos a la base de datos
    //Select * from posts
    let p_index_result = pindex.order(id_p_index).load::<Pindex>(&conn).expect("Error al ejecutar la query");
    for _p_index in p_index_result {
        println!("
        id_p_index:{} 
        title_p_index:{}
        sub_title_p_index:{}
        p_p_index_1:{}
        p_p_index_2:{}
        p_p_index_3:{}
        img_p_index:{}
        ",
        _p_index.id_p_index, 
        _p_index.title_p_index,
        _p_index.sub_title_p_index,
        _p_index.p_p_index_1,
        _p_index.p_p_index_2,
        _p_index.p_p_index_3,
        _p_index.img_p_index,
        );
    }
}
//Final Funcion Seleccionar todos los Parrafo index
//*********************************************End Consultas Select Template Index*********************************************



//*********************************************Consultas Select Template Service*********************************************
//Start Funcion Seleccionar todos los Service
fn query_select_all_service(){
    println!("Query Select All Initial Service");
    let conn =  conectar_bd(); //Conectamos a la base de datos

    //Select * from posts
    let service_result = services.order(id_service).load::<Service>(&conn).expect("Error al ejecutar la query");
    for _service in service_result {
        println!("
        Id:{} 
        Name:{}
        Description:{}
        Img Service 1:{}
        Img Service 2:{}
        Img Service 3:{}
        Published:{}",
        _service.id_service, 
        _service.name,
        _service.description,
        _service.img_service_1,
        _service.img_service_2,
        _service.img_service_3,
        _service.published_service);
    } 
}
//Final Funcion Seleccionar todos los Service

//Start Funcion Seleccionar todos los Parrafo Service
fn query_select_all_p_service(){
    println!("Query Select All Initial P Service");
    let conn =  conectar_bd(); //Conectamos a la base de datos

    //Select * from posts
    let p_service_result = pservice.order(id_p_service).load::<Pservice>(&conn).expect("Error al ejecutar la query");
    for _p_service in p_service_result {
        println!("
        Id:{} 
        title_p_service:{}
        sub_title_p_service:{}
        p_p_service_1 :{}
        p_p_service_2 :{}
        p_p_service_3 :{}
        img_p_service:{}",
        _p_service.id_p_service, 
        _p_service.title_p_service,
        _p_service.sub_title_p_service,
        _p_service.p_p_service_1,
        _p_service.p_p_service_2,
        _p_service.p_p_service_3,
        _p_service.img_p_service);
    } 
}
//Final Funcion Seleccionar todos los Parrafo Service

//Start Funcion Seleccionar todos los Layout Service
fn query_select_all_layout_service(){
    println!("Query Select All Initial Layout Service");
    let conn =  conectar_bd(); //Conectamos a la base de datos

    //Select * from posts
    let layout_service_result = layoutservice.order(id_layout_service).load::<LayoutService>(&conn).expect("Error al ejecutar la query");
    for _layout_service in layout_service_result {
        println!("
        Id:{} 
        title_layout_service:{}
        p_p_layout_service:{}
        img_layout_service :{}
        title_button_layout_service_1 :{}
        title_button_layout_service_2 :{}
        href_layout_service_1:{}
        href_layout_service_2:{}",
        _layout_service.id_layout_service, 
        _layout_service.title_layout_service,
        _layout_service.p_p_layout_service,
        _layout_service.img_layout_service,
        _layout_service.title_button_layout_service_1,
        _layout_service.title_button_layout_service_2,
        _layout_service.href_layout_service_1,
        _layout_service.href_layout_service_2);
    } 
}
//Final Funcion Seleccionar todos los Layout Service
//*********************************************End Consultas Select Template Service*********************************************


//*********************************************Consultas Select Template Profile*********************************************
//Start Funcion Seleccionar todos los Layout Profile
fn query_select_all_layout_profile(){
    println!("Query Select All Initial Layout Service");
    let conn =  conectar_bd(); //Conectamos a la base de datos

    //Select * from posts
    let _layout_profile_result = layoutprofile.order(id_layout_profile).load::<LayoutProfile>(&conn).expect("Error al ejecutar la query");
    for _layout_profile in _layout_profile_result {
        println!("
        Id:{} 
        title_layout_service:{}
        p_p_layout_service:{}
        img_layout_service :{}
        title_button_layout_service_1 :{}
        title_button_layout_service_2 :{}
        href_layout_service_1:{}
        href_layout_service_2:{}",
        _layout_profile.id_layout_profile, 
        _layout_profile.title_layout_profile,
        _layout_profile.p_p_layout_profile,
        _layout_profile.img_layout_profile,
        _layout_profile.title_button_layout_profile_1,
        _layout_profile.title_button_layout_profile_2,
        _layout_profile.href_layout_profile_1,
        _layout_profile.href_layout_profile_2);
    } 
}
//Final Funcion Seleccionar todos los Layout Profile



fn query_select_all_p_profile(){
    println!("Query Select All Initial P Profile");
    let conn =  conectar_bd(); //Conectamos a la base de datos

    let p_profile_result = pprofile.order(id_p_profile).load::<Pprofile>(&conn).expect("Error al ejecutar la query");
    for _p_profile in p_profile_result {
        println!("
        Id:{} 
        title_p_profile:{}
        sub_title_p_profile:{}
        p_p_profile_1 :{}
        p_p_profile_2 :{}
        p_p_profile_3 :{}
        img_p_profile:{}",
        _p_profile.id_p_profile, 
        _p_profile.title_p_profile,
        _p_profile.sub_title_p_profile,
        _p_profile.p_p_profile_1,
        _p_profile.p_p_profile_2,
        _p_profile.p_p_profile_3,
        _p_profile.img_p_profile);
    }
}

fn query_select_all_skills(){
    println!("Query Select All Initial Skills");
    let conn =  conectar_bd(); //Conectamos a la base de datos

    let skills_result = skills.order(id_skill).load::<Skilss>(&conn).expect("Error al ejecutar la query");
    for _skills in skills_result {
        println!("
        Id:{} 
        title_skill:{}
        name_skill: {}
        level_skill:{}
        type_skill :{}
        img_skill :{}
        img_perfil_skill:{}
        ",
        _skills.id_skill,
        _skills.title_skill, 
        _skills.name_skill,
        _skills.level_skill,
        _skills.type_skill,
        _skills.img_skill,
        _skills.img_perfil_skill,
    );
    }
}


//Start Funcion Seleccionar todos los Profile
fn query_select_all_profile(){
    println!("Query Select All Initial Profile");
    let conn =  conectar_bd(); //Conectamos a la base de datos

    //Select * from posts
    let profile_result = profiles.order(id_profile).load::<Profile>(&conn).expect("Error al ejecutar la query");
    for _profile in profile_result {
        println!("
        Id:{} 
        Name:{}
        Last Name:{} 
        Email:{} 
        Git Hub:{} 
        Phone:{} 
        Address:{} 
        City:{} 
        Country:{} 
        Text:{} 
        Published:{}",
        _profile.id_profile, 
        _profile.name,
        _profile.last_name, 
        _profile.email,
        _profile.git_hub,
        _profile.phone,
        _profile.address,
        _profile.city,
        _profile.country,
        _profile.text,
        _profile.published_profile);
    }
}
//Final Funcion Seleccionar todos los Profile
//*********************************************End Consultas Select Template Profile*********************************************


//********************************************* Consultas Select Template Post*********************************************
//Start Funcion Seleccionar layout Post
fn query_select_all_layout_post(){
    println!("Query Select All Initial Layout Post");
    let conn =  conectar_bd(); //Conectamos a la base de datos

    //Select * from posts
    let layout_post_result = layoutpost.order(id_layout_post).load::<LayoutPost>(&conn).expect("Error al ejecutar la query");
    for _layout_post in layout_post_result {
        println!("
        Id:{} 
        title_layout_post:{}
        p_p_layout_post:{}
        img_layout_post_1 :{}
        img_layout_post_2 :{}
        title_button_layout_post_1:{}
        href_layout_post_1:{}",
        _layout_post.id_layout_post, 
        _layout_post.title_layout_post,
        _layout_post.p_p_layout_post,
        _layout_post.img_layout_post_1,
        _layout_post.img_layout_post_2,
        _layout_post.title_button_layout_post_1,
        _layout_post.href_layout_post_1);
    } 
}



//Start Funcion Seleccionar todos los post
fn query_select_all_post(){
    println!("Query Select All Initial Post");
    let conn =  conectar_bd(); //Conectamos a la base de datos

    //Select * from posts
    let posts_result = posts.order(id).load::<Post>(&conn).expect("Error al ejecutar la query");
    for _post in posts_result {
        println!("
        Id:{} 
        Title:{}
        Slug:{} 
        Bode:{}
        Img:{} 
        DAte: {}
        Published:{}",
        _post.id, 
        _post.title,
        _post.slug, 
        _post.body,
        _post.img,
        _post.date,
        _post.published);
    }
}
//Final Funcion Seleccionar todos los post

//Start Funcion Seleccionar todos los Post por Titulo
fn query_select_title(_title:&str){
    println!("Query Select Title Initial");
    let conn =  conectar_bd(); //Conectamos a la base de datos

    //Select * from posts where title = "First Post"
    let post_result = posts.filter(title.eq(_title)).load::<Post>(&conn).expect("Error al ejecutar la query");
    for _post in post_result {
        println!("
        Id:{} 
        Title:{}
        Slug:{} 
        Bode:{} 
        Published:{}",
        _post.id, 
        _post.title,
        _post.slug, 
        _post.body,
        _post.published);
    }
}
//Final Funcion Seleccionar todos los Post por Titulo
//*********************************************End Consultas Select Template Post*********************************************



//*********************************************Consultas Select Template Contact*********************************************
//Start Funcion Seleccionar los parrafos Contact
fn query_selct_all_p_contact(){
    println!("Query Select All Initial P Contact");
    let conn =  conectar_bd(); //Conectamos a la base de datos

    //Select * from posts
    let p_contact_result = p_contact.order(id_p_contact).load::<PContact>(&conn).expect("Error al ejecutar la query");
    for _p_contact in p_contact_result {
        println!("
        Id:{} 
        title_p_contact:{}
        p_p_contact_1 :{}
        p_p_contact_2 :{}
        p_p_contact_3 :{}
        title_button_contact_1 :{}
        title_button_contact_2 :{}
        title_button_contact_3 :{}
        href_contact_1 :{}
        href_contact_2 :{}
        href_contact_3:{}",
        _p_contact.id_p_contact, 
        _p_contact.title_p_contact,
        _p_contact.p_p_contact_1,
        _p_contact.p_p_contact_2,
        _p_contact.p_p_contact_3,
        _p_contact.title_button_contact_1,
        _p_contact.title_button_contact_2,
        _p_contact.title_button_contact_3,
        _p_contact.href_contact_1,
        _p_contact.href_contact_2,
        _p_contact.href_contact_3);
    }
}


//Start Funcion Seleccionar todos los Social
fn query_select_all_social(){
    println!("Query Select All Initial Social");
    let conn =  conectar_bd(); //Conectamos a la base de datos
    //Select * from posts
    let social_result = socials.order(id_social).load::<Social>(&conn).expect("Error al ejecutar la query");
    for _social in social_result {
        println!("
        Id:{} 
        Name:{}
        Telegram:{}
        Linkedin:{}
        Github:{}
        Instagram:{}
        Whatsapp:{}
        Twitter:{}
        Published:{}",
        _social.id_socials, 
        _social.name,
        _social.telegram,
        _social.linkedin,
        _social.github,
        _social.instagram,
        _social.whatsapp,
        _social.twitter,
        _social.published_social);
    }
}
//Final Funcion Seleccionar todos los Social

//Start Funcion Seleccionar todos los Msg
fn query_selct_all_msg(){
    println!("Query Select All Initial Msg");
    let conn =  conectar_bd(); //Conectamos a la base de datos
    //Select * from posts
    let msg_result = msg.order(first_name).load::<Msg>(&conn).expect("Error al ejecutar la query");
    for _msg in msg_result {
        println!("Contact Post 
        
        firstName:{},
        lastName:{},
        email:{},
        country: {},
        message:{},
    ",  
        _msg.first_name,
        _msg.last_name,
        _msg.email,
        _msg.country,
        _msg.message,
    );
    }
}
//Final Funcion Seleccionar todos los Msg
//*********************************************End Consultas Select Template Contact*********************************************


//????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????

//********************************************* Consultas Insert Template Index*********************************************

//Start Funcion insertar un nuevo Carousel
fn query_insert_carousel(
    _title_carousel_1:&str,
    _title_carousel_2:&str,
    _title_carousel_3:&str,
    _p_carousel_1:&str,
    _p_carousel_2:&str,
    _p_carousel_3:&str,
    _img_carousel_1:&str,
    _img_carousel_2:&str,
    _img_carousel_3:&str,
    _href_carousel_1:&str,
    _href_carousel_2:&str,
    _href_carousel_3:&str, ){
    println!("Query Insert Initial Carousel");
    let conn = conectar_bd();

    let new_carousel = NewCarousel {
        title_carousel_1 :_title_carousel_1,
        title_carousel_2 :_title_carousel_2,
        title_carousel_3 :_title_carousel_3,
        p_carousel_1:_p_carousel_1,
        p_carousel_2 :_p_carousel_2,
        p_carousel_3 :_p_carousel_3,
        img_carousel_1 :_img_carousel_1,
        img_carousel_2 :_img_carousel_2,
        img_carousel_3 :_img_carousel_3,
        href_carousel_1 :_href_carousel_1,
        href_carousel_2 :_href_carousel_2,
        href_carousel_3 :_href_carousel_3,
    };

    let _carousel: Carousel = diesel::insert_into(carousel::table)
        .values(&new_carousel)
        .get_result(&conn)
        .expect("Error al insertar el carousel");
        
    let carousel_result =  carousel.limit(1)
        .load::<Carousel>(&conn)
        .expect("Error al ejecutar la query"); 
    for _carousel in carousel_result {
        println!("
        id_carousel:{} 
        title_carousel_1:{}
        title_carousel_2:{}
        title_carousel_3:{}
        p_carousel_1:{}
        p_carousel_2:{} 
        p_carousel_3:{}  
        img_carousel_1:{}
        img_carousel_2:{}
        img_carousel_3:{}
        href_carousel_1:{}
        href_carousel_2:{} 
        href_carousel_3:{}  
        ",
        _carousel.id_carousel, 
        _carousel.title_carousel_1,
        _carousel.title_carousel_2,
        _carousel.title_carousel_3,
        _carousel.p_carousel_1,
        _carousel.p_carousel_2, 
        _carousel.p_carousel_3,  
        _carousel.img_carousel_1,
        _carousel.img_carousel_2,
        _carousel.img_carousel_3,
        _carousel.href_carousel_1,
        _carousel.href_carousel_2,
        _carousel.href_carousel_3,
        );
    }    
}
//Final Funcion insertar un nuevo Carousel

//Start Funcion insertar un nuevo Symbol
fn query_insert_symbol(_title_symbol:&str,_p_symbol:&str,_img_symbol:&str,_href_symbol:&str){
    println!("Query insert symbol");
    let conn = conectar_bd();

    let new_symbol = NewSymbol {
        title_symbol: _title_symbol,
        p_symbol: _p_symbol,
        img_symbol: _img_symbol,
        href_symbol: _href_symbol
    };

    let _symbol:Symbol = diesel::insert_into(symbol::table)
    .values(new_symbol)
    .get_result(&conn)
    .expect("No se ha podido establecer los datos.");

    let symbol_result =  symbol.limit(1)
        .load::<Symbol>(&conn)
        .expect("Error al ejecutar la query"); 

    for _symbol in symbol_result {
        println!("
        title_symbol :{} 
        p_symbol :{}   
        img_symbol :{}   
        href_symbol :{}      
        
        ",
        _symbol.title_symbol,
        _symbol.p_symbol,
        _symbol.img_symbol,
        _symbol.href_symbol);
    }    
}
//Final Funcion insertar un nuevo Symbol

//Start Funcion insertar un nuevo Parrafo index
fn query_insert_p_index(_title_p_index:&str,_sub_title_p_index:&str,_p_p_index_1:&str,_p_p_index_2:&str,_p_p_index_3:&str,_img_p_index:&str){
    println!("Query Insert Initial P Index");
    let conn = conectar_bd();

    let new_p_index = NewPindex {
        title_p_index:_title_p_index,
        sub_title_p_index:_sub_title_p_index,
        p_p_index_1:_p_p_index_1,
        p_p_index_2:_p_p_index_2,
        p_p_index_3:_p_p_index_3,
        img_p_index:_img_p_index,
    };

    let _p_index:Pindex = diesel::insert_into(pindex::table)
    .values(new_p_index)
    .get_result(&conn)
    .expect("No se ha podido establecer los datos.");

    let p_index_result =  pindex.limit(1)
        .load::<Pindex>(&conn)
        .expect("Error al ejecutar la query"); 

    for _p_index in p_index_result {    
        println!("
        title_p_index:{}
        sub_title_p_index:{}
        p_p_index_1:{}
        p_p_index_2:{}
        p_p_index_3:{}
        img_p_index:{}
        ",
        _p_index.title_p_index,
        _p_index.sub_title_p_index,
        _p_index.p_p_index_1,
        _p_index.p_p_index_2,
        _p_index.p_p_index_3,
        _p_index.img_p_index,
        );
    }

}
//Final Funcion insertar un nuevo Parrafo index
//********************************************* End Consultas Insert Template Index*********************************************







//*********************************************Consultas Insert Template Service*********************************************
//Start Funcion insertar un nuevo Service
fn query_insert_service(_name:&str,_description:&str,_img_service_1:&str,_img_service_2:&str,_img_service_3:&str,_published_service:bool){
    println!("Query Insert Initial Service");
    let conn = conectar_bd();

    let new_service = NewService {
        name: _name,
        description: _description,
        img_service_1: _img_service_1,
        img_service_2: _img_service_2,
        img_service_3: _img_service_3,
        published_service :_published_service,
    };

    let _service: Service = diesel::insert_into(services::table)
        .values(&new_service)
        .get_result(&conn)
        .expect("Error al insertar el service");

    let service_result =  services.limit(1)
        .load::<Service>(&conn)
        .expect("Error al ejecutar la query"); 

    for _service in service_result {
        println!("
        Id:{} 
        Name:{}
        Description:{}
        Img Service 1:{}
        Img Service 2:{}
        Img Service 3:{}
        Published:{}",
        _service.id_service, 
        _service.name,
        _service.description,
        _service.img_service_1,
        _service.img_service_2,
        _service.img_service_3,
        _service.published_service);
    }    
}
//Final Funcion insertar un nuevo Service

//Start Funcion insertar un nuevo Parrafo Service
fn query_insert_p_service(_title_p_service:&str,_sub_title_p_service:&str,_p_p_service_1:&str,_p_p_service_2:&str,_p_p_service_3:&str,_img_p_service:&str,){
    println!("Query Insert Initial Parrafo Service");
    let conn = conectar_bd();

    let new_p_service = NewParrafoService {
        title_p_service: _title_p_service,
        sub_title_p_service :_sub_title_p_service,
        p_p_service_1:_p_p_service_1,
        p_p_service_2:_p_p_service_2,
        p_p_service_3:_p_p_service_3,
        img_p_service:_img_p_service,
    };

    let _p_service: Pservice = diesel::insert_into(pservice::table)
        .values(&new_p_service)
        .get_result(&conn)
        .expect("Error al insertar el parrafo service");

    let p_service_result =  pservice.limit(1)
        .load::<Pservice>(&conn)
        .expect("Error al ejecutar la query"); 

        for _p_service in p_service_result {
            println!("
            Id:{} 
            title_p_service:{}
            sub_title_p_service:{}
            p_p_service_1 :{}
            p_p_service_2 :{}
            p_p_service_3 :{}
            img_p_service:{}",
            _p_service.id_p_service, 
            _p_service.title_p_service,
            _p_service.sub_title_p_service,
            _p_service.p_p_service_1,
            _p_service.p_p_service_2,
            _p_service.p_p_service_3,
            _p_service.img_p_service);
        } 
}
//Final Funcion insertar un nuevo Parrafo Service

//Start Funcion insertar un nuevo Layout Service
fn query_insert_layout_service(_title_layout_service:&str,_p_p_layout_service:&str,_img_layout_service:&str,_title_button_layout_service_1:&str,_title_button_layout_service_2:&str,_href_layout_service_1:&str,_href_layout_service_2:&str,){
    println!("Query Insert Initial Layout Service");
    let conn = conectar_bd();

    let new_layout_service = NewLayoutService {
        title_layout_service: _title_layout_service,
        p_p_layout_service :_p_p_layout_service,
        img_layout_service:_img_layout_service,
        title_button_layout_service_1:_title_button_layout_service_1,
        title_button_layout_service_2:_title_button_layout_service_2,
        href_layout_service_1:_href_layout_service_1,
        href_layout_service_2:_href_layout_service_2,
    };
    
    let _layout_service: LayoutService = diesel::insert_into(layoutservice::table)
        .values(&new_layout_service)
        .get_result(&conn)
        .expect("Error al insertar el layout service");
    
    let layout_service_result =  layoutservice.limit(1)
        .load::<LayoutService>(&conn)
        .expect("Error al ejecutar la query");

        for _layout_service in layout_service_result {
            println!("
            Id:{} 
            title_layout_service:{}
            p_p_layout_service:{}
            img_layout_service :{}
            title_button_layout_service_1 :{}
            title_button_layout_service_2 :{}
            href_layout_service_1:{}
            href_layout_service_2:{}",
            _layout_service.id_layout_service, 
            _layout_service.title_layout_service,
            _layout_service.p_p_layout_service,
            _layout_service.img_layout_service,
            _layout_service.title_button_layout_service_1,
            _layout_service.title_button_layout_service_2,
            _layout_service.href_layout_service_1,
            _layout_service.href_layout_service_2);
        }             
}
//Final Funcion insertar un nuevo Layout Service
//*********************************************End Consultas Insert Template Service*********************************************



//*********************************************Consultas Insert Template Profile*********************************************
fn query_insert_layout_profile(_title_layout_profile:&str,_p_p_layout_profile:&str,_img_layout_profile:&str,_title_button_layout_profile_1:&str,_title_button_layout_profile_2:&str,_href_layout_profile_1:&str,_href_layout_profile_2:&str,){
    println!("Query Insert Initial Layout Profile");
    let conn = conectar_bd();

    let new_layout_profile = NewLayoutprofile {
        title_layout_profile: _title_layout_profile,
        p_p_layout_profile :_p_p_layout_profile,
        img_layout_profile:_img_layout_profile,
        title_button_layout_profile_1:_title_button_layout_profile_1,
        title_button_layout_profile_2:_title_button_layout_profile_2,
        href_layout_profile_1:_href_layout_profile_1,
        href_layout_profile_2:_href_layout_profile_2,
    };
    
    let _layout_profile: LayoutProfile = diesel::insert_into(layoutprofile::table)
        .values(&new_layout_profile)
        .get_result(&conn)
        .expect("Error al insertar el layout profile");
    
    let layout_profile_result =  layoutprofile.limit(1)
        .load::<LayoutProfile>(&conn)
        .expect("Error al ejecutar la query");

        for _layout_profile in layout_profile_result {
            println!("
            Id:{} 
            title_layout_profile:{}
            p_p_layout_profile:{}
            img_layout_profile :{}
            title_button_layout_profile_1 :{}
            title_button_layout_profile_2 :{}
            href_layout_profile_1:{}
            href_layout_profile_2:{}",
            _layout_profile.id_layout_profile, 
            _layout_profile.title_layout_profile,
            _layout_profile.p_p_layout_profile,
            _layout_profile.img_layout_profile,
            _layout_profile.title_button_layout_profile_1,
            _layout_profile.title_button_layout_profile_2,
            _layout_profile.href_layout_profile_1,
            _layout_profile.href_layout_profile_2);
        }             
}

fn query_insert_p_profile(_title_p_profile:&str,_sub_title_p_profile:&str,_p_p_profile_1:&str,_p_p_profile_2:&str,_p_p_profile_3:&str,_img_p_profile:&str,){
    println!("Query Insert Initial Parrafo Profile");
    let conn = conectar_bd();

    let new_p_profile = NewParrafoProfile {
        title_p_profile: _title_p_profile,
        sub_title_p_profile :_sub_title_p_profile,
        p_p_profile_1:_p_p_profile_1,
        p_p_profile_2:_p_p_profile_2,
        p_p_profile_3:_p_p_profile_3,
        img_p_profile:_img_p_profile,
    };

    let _p_profile: Pprofile = diesel::insert_into(pprofile::table)
        .values(&new_p_profile)
        .get_result(&conn)
        .expect("Error al insertar el parrafo profile");

    let p_profile_result =  pprofile.limit(1)
        .load::<Pprofile>(&conn)
        .expect("Error al ejecutar la query");
    
    for _p_profile in p_profile_result {
        println!("
        Id:{} 
        title_p_profile:{}
        sub_title_p_profile:{}
        p_p_profile_1 :{}
        p_p_profile_2 :{}
        p_p_profile_3 :{}
        img_p_profile:{}",
        _p_profile.id_p_profile, 
        _p_profile.title_p_profile,
        _p_profile.sub_title_p_profile,
        _p_profile.p_p_profile_1,
        _p_profile.p_p_profile_2,
        _p_profile.p_p_profile_3,
        _p_profile.img_p_profile);
    }    
}



//Start Funcion insertar un nuevo profile
fn query_insert_profile( _name:&str,
    _last_name:&str,
    _email:&str,
    _git_hub:&str,
    _phone:&str,
    _address:&str,
    _city:&str,
    _country:&str,
    _text:&str,
    _published_profile:bool){
    println!("Query Insert Initial Profile");
    let conn = conectar_bd();
 
    let new_profile = NewProfile {
        name: _name,
        last_name: _last_name,
        email: _email,
        git_hub: _git_hub,
        phone: _phone,
        address: _address,
        city: _city,
        country: _country,
        text: _text,
        published_profile: &_published_profile,
    };

    let _profile: Profile = diesel::insert_into(profiles::table)
        .values(&new_profile)
        .get_result(&conn)
        .expect("Error al insertar el profile");

    let  profile_result =  profiles.limit(1)
        .load::<Profile>(&conn)
        .expect("Error al ejecutar la query");    

    for _profile in profile_result {
        println!("
        Id:{} 
        Name:{}
        Last Name:{} 
        Email:{} 
        Git Hub:{} 
        Phone:{} 
        Address:{} 
        City:{} 
        Country:{} 
        Text:{} 
        Published:{}",
        _profile.id_profile, 
        _profile.name,
        _profile.last_name, 
        _profile.email,
        _profile.git_hub,
        _profile.phone,
        _profile.address,
        _profile.city,
        _profile.country,
        _profile.text,
        _profile.published_profile);
    }
}
//Final Funcion insertar un nuevo profile

fn query_insert_skills(_name_skill:&str,_level_skill:&str,_type_skill:&str,_img_skill:&str){
    println!("Query Insert Initial Skills");
    let conn = conectar_bd();
    let tittle:&str= "Skills";
    let img_profile_white_hacker="https://img2.freepng.es/20180402/lhe/kisspng-white-hat-security-hacker-anonymous-clip-art-anonymous-5ac1b1ffbc2056.6923529915226434557706.jpg";

    let new_skill = NewSkill {
        title_skill: tittle,
        name_skill: _name_skill,
        level_skill: _level_skill,
        type_skill: _type_skill,
        img_skill: _img_skill,
        img_perfil_skill: img_profile_white_hacker,
    };

    let _skill: Skilss = diesel::insert_into(skills::table)
        .values(&new_skill)
        .get_result(&conn)
        .expect("Error al insertar el skill");
    
    let skill_result =  skills.limit(1)
        .load::<Skilss>(&conn)
        .expect("Error al ejecutar la query");

        for _skill in skill_result {
            println!("
            Id:{} 
            title_skill:{}
            name_skill:{}
            level_skill :{}
            type_skill :{}
            img_skill :{}
            img_perfil_skill:{}",
            _skill.id_skill, 
            _skill.title_skill,
            _skill.name_skill,
            _skill.level_skill,
            _skill.type_skill,
            _skill.img_skill,
            _skill.img_perfil_skill);
        }
}

//*********************************************End Consultas Insert Template Profile*********************************************


//********************************************* Consultas Insert Template post*********************************************
//Start Funcion insertar un layout post
fn query_insert_layout_post(_title_layout_post:&str,_p_p_layout_post:&str,_img_layout_post_1:&str,_img_layout_post_2:&str,_title_button_layout_post_1:&str,_href_layout_post_1:&str,){
    println!("Query Insert Initial Layout Post");
    let conn = conectar_bd();

    let new_layout_post = NewLayoutpost {
        title_layout_post: _title_layout_post,
        p_p_layout_post: _p_p_layout_post,
        img_layout_post_1: _img_layout_post_1,
        img_layout_post_2: _img_layout_post_2,
        title_button_layout_post_1: _title_button_layout_post_1,
        href_layout_post_1: _href_layout_post_1,
    };

    let _layout_post: LayoutPost = diesel::insert_into(layoutpost::table)
        .values(&new_layout_post)
        .get_result(&conn)
        .expect("Error al insertar el layout post");

    let layout_post_result =  layoutpost.limit(1)
        .load::<LayoutPost>(&conn)
        .expect("Error al ejecutar la query");
    
    for _layout_post in layout_post_result {
        println!("
        Id:{} 
        title_layout_post:{}
        p_p_layout_post:{}
        img_layout_post_1 :{}
        img_layout_post_2 :{}
        title_button_layout_post_1 :{}
        href_layout_post_1:{}",
        _layout_post.id_layout_post, 
        _layout_post.title_layout_post,
        _layout_post.p_p_layout_post,
        _layout_post.img_layout_post_1,
        _layout_post.img_layout_post_2,
        _layout_post.title_button_layout_post_1,
        _layout_post.href_layout_post_1);
    }    
}


//Start Funcion insertar un nuevo post
fn query_insert_post(_title :&str, _body:&str, _slug:&str,_img:&str,_published:bool){
    //Time
    use chrono::Local;
    use chrono::DateTime;
    let local_date: DateTime<Local> = Local::now();
    let string_date = local_date.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("Local Date: {}", string_date);

    println!("Query Insert Initial");
    let conn = conectar_bd();
 
    let new_post = NewPost {
        title: _title,
        slug: _slug,
        body: _body,
        img:_img,
        date: &string_date,
        published: &_published,
    };

    let _post: Post = diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(&conn)
        .expect("Error al insertar el post");

    let  post_result =  posts.limit(1)
        .load::<Post>(&conn)
        .expect("Error al ejecutar la query");

        
    for _post in post_result {
        println!("
        Id:{} 
        Title:{}
        Slug:{} 
        Bode:{} 
        Img:{}
        Date:{}
        Published:{}",
        _post.id, 
        _post.title,
        _post.slug, 
        _post.body,
        _post.img,
        _post.date,
        _post.published);
    }
    
}
//Final Funcion insertar un nuevo post


//Start Funcion update un Post
fn query_update_or_edit(_id:i32,_title:&str,_slug:&str, _body:&str ,_published:bool){
    println!("Query Update or Edit Initial");
    let conn = conectar_bd();

    let  _post:Post = diesel::update(posts.filter(id.eq(_id)))
        .set((title.eq(_title),slug.eq_all(_slug),body.eq(_body),published.eq(_published)))
        .get_result(&conn)
        .expect("Error al ejecutar la query");
    
        println!("
        Id:{} 
        Title:{}
        Slug:{} 
        Bode:{} 
        Published:{}",
        _post.id, 
        _post.title,
        _post.slug, 
        _post.body,
        _post.published);
}
//Final Funcion update un Post

//Start Funcion delete un Post
fn query_delete(_id:i32){
    println!("Query Delete Initial");
    let conn = conectar_bd();

    let _post: Post = diesel::delete(posts.filter(id.eq(_id)))
        .get_result(&conn)
        .expect("Error al ejecutar la query");

    println!("Post Delete: {}", _post.id);
}
//Final Funcion delete un Post
//********************************************* END Consultas Insert Template post*********************************************




//********************************************* Consultas Insert Template Contact*********************************************
//Start Funcion insertar un parrafo contacto
fn query_insert_p_contact(
    _title_p_contact:&str,
    _p_p_contact_1:&str,
    _p_p_contact_2:&str,
    _p_p_contact_3:&str,
    _title_button_contact_1:&str,
    _title_button_contact_2:&str,
    _title_button_contact_3:&str,
    _href_contact_1:&str,
    _href_contact_2:&str,
    _href_contact_3:&str,
    ){
    println!("Query Insert Initial P Contact");
    let conn = conectar_bd();

    let new_p_contact = NewPcontact {
        title_p_contact: _title_p_contact,
        p_p_contact_1: _p_p_contact_1,
        p_p_contact_2: _p_p_contact_2,
        p_p_contact_3: _p_p_contact_3,
        title_button_contact_1: _title_button_contact_1,
        title_button_contact_2: _title_button_contact_2,
        title_button_contact_3: _title_button_contact_3,
        href_contact_1: _href_contact_1,
        href_contact_2: _href_contact_2,
        href_contact_3: _href_contact_3,
    };

    let _p_contact: PContact = diesel::insert_into(p_contact::table)
        .values(&new_p_contact)
        .get_result(&conn)
        .expect("Error al insertar el p contact");

    let p_contact_result =  p_contact.limit(1)
        .load::<PContact>(&conn)
        .expect("Error al ejecutar la query");
    
    for _p_contact in p_contact_result {
        println!("
        Id:{} 
        title_p_contact:{}
        p_p_contact_1:{}
        p_p_contact_2:{}
        p_p_contact_3:{}
        title_button_contact_1:{}
        title_button_contact_2:{}
        title_button_contact_3:{}
        href_contact_1:{}
        href_contact_2:{}
        href_contact_3:{}
        ",
        _p_contact.id_p_contact, 
        _p_contact.title_p_contact, 
        _p_contact.p_p_contact_1, 
        _p_contact.p_p_contact_2, 
        _p_contact.p_p_contact_3, 
        _p_contact.title_button_contact_1, 
        _p_contact.title_button_contact_2, 
        _p_contact.title_button_contact_3, 
        _p_contact.href_contact_1, 
        _p_contact.href_contact_2, 
        _p_contact.href_contact_3, 
    );
    }    
}

//Start Funcion insertar un nuevo Redes sociales
fn query_insert_social(_name:&str,_telegram:&str,_linkedin:&str,_github:&str,_instagram:&str,_whatsapp:&str,_twitter:&str,_published_social:bool){
    println!("Query Insert Initial Social");
    let conn = conectar_bd();

    let new_social = NewSocial {
        name: _name,
        telegram: _telegram,
        linkedin: _linkedin,
        github: _github,
        instagram: _instagram,
        whatsapp: _whatsapp,
        twitter: _twitter,
        published_social :_published_social,
    };

    let _social: Social = diesel::insert_into(socials::table)
        .values(&new_social)
        .get_result(&conn)
        .expect("Error al insertar el social");
        
    let social_result =  socials.limit(1)
        .load::<Social>(&conn)
        .expect("Error al ejecutar la query"); 
    for _social in social_result {
        println!("
        Id:{} 
        Name:{}
        Telegram:{}
        Linkedin:{}
        Github:{}
        Instagram:{}
        Whatsapp:{}
        Twitter:{}
        Published:{}",
        _social.id_socials, 
        _social.name,
        _social.telegram,
        _social.linkedin,
        _social.github,
        _social.instagram,
        _social.whatsapp,
        _social.twitter,
        _social.published_social);
    }    
}
//Final Funcion insertar un nuevo Redes sociales

//Start Funcion insertar un nuevo Msg
fn query_insert_msg(_first_name:&str,_last_name:&str,_email:&str,_country:&str,_message:&str){
    println!("Query Insert Initial Msg");
    let conn = conectar_bd();

    let new_msg = NewMsg {
        first_name: _first_name,
        last_name: _last_name,
        email: _email,
        country: _country,
        message: _message,
    };

    let _msg: Msg = diesel::insert_into(msg::table)
        .values(&new_msg)
        .get_result(&conn)
        .expect("Error al insertar el msg");
        
    let msg_result =  msg.limit(1)
        .load::<Msg>(&conn)
        .expect("Error al ejecutar la query"); 
    for _msg in msg_result {
        println!("
        First Name:{}
        Last Name:{}
        Email:{}
        Country:{}
        Message:{}
        ",
        
        _msg.first_name,
        _msg.last_name,
        _msg.email,
        _msg.country,
        _msg.message,
        );
    }    
}
//Final Funcion insertar un nuevo Msg
//*********************************************End Consultas Insert Template Contact*********************************************



//********************************************* ACTIX WEB IN USE FOR TEMPLATES*********************************************
/*
#[get("/")]
async fn index(pool: web::Data<DbPool>, template_manager: web::Data<tera::Tera>) -> impl Responder {
    let conn = pool.get().expect("Problemas al traer la base de datos");

    match web::block(move || {carousel.load::<Carousel>(&conn)}).await {
        Ok(data) => {
            let data = data.unwrap();

            let mut ctx = tera::Context::new();
            ctx.insert("carousel", &data);
            ctx.insert("symbol", &data);
          
            HttpResponse::Ok().content_type("text/html").body(
                
                template_manager.render("index.html", &ctx).unwrap()
            )
            // return HttpResponse::Ok().body(format!("{:?}", data));
        },
        Err(_err) => HttpResponse::Ok().body("Error al recibir la data"),

    }
}*/

#[get("/")]
async fn index(pool: web::Data<DbPool>,template_manager: web::Data<tera::Tera>) -> impl Responder {
        let conn = pool.get().expect("Problemas al traer la base de datos");
        let diesel_capture_dates_1 = carousel.load::<Carousel>(&conn).expect("Error al ejecutar la query");
        let diesel_capture_dates_2 = symbol.load::<Symbol>(&conn).expect("Error al ejecutar la query");
        let diesel_capture_dates_3 = pindex.load::<Pindex>(&conn).expect("Error al ejecutar la query");

        let mut ctx = tera::Context::new();
            ctx.insert("carousel", &diesel_capture_dates_1);
            ctx.insert("symbol", &diesel_capture_dates_2);
            ctx.insert("pindex", &diesel_capture_dates_3);

        HttpResponse::Ok().content_type("text/html").body(
            template_manager.render("index.html", &ctx).unwrap()
        )

    }

#[get("/service")]
async fn service(pool:web::Data<DbPool>,template_manager:web::Data<tera::Tera>) -> impl Responder {
        let conn = pool.get().expect("Problemas al traer la base de datos");
        let diesel_capture_dates_1 = layoutservice.load::<LayoutService>(&conn).expect("Error al ejecutar la query");
        let diesel_capture_dates_2 = pservice.load::<Pservice>(&conn).expect("Error al ejecutar la query");
        let diesel_capture_dates_3 = services.load::<Service>(&conn).expect("Error al ejecutar la query");
        
        let mut ctx = tera::Context::new();
            ctx.insert("layoutservice", &diesel_capture_dates_1);
            ctx.insert("pservice", &diesel_capture_dates_2);
            ctx.insert("services", &diesel_capture_dates_3);

            HttpResponse::Ok().content_type("text/html").body(
                template_manager.render("service.html", &ctx).unwrap()
            )
   
}



#[get("/profile")]
async fn profile(pool:web::Data<DbPool>,template_manager:web::Data<tera::Tera>) -> impl Responder {
    let conn = pool.get().expect("Problemas al traer la base de datos");
    let diesel_capture_dates_1 = layoutprofile.load::<LayoutProfile>(&conn).expect("Error al ejecutar la query");
    let diesel_capture_dates_2 = pprofile.load::<Pprofile>(&conn).expect("Error al ejecutar la query");
    let diesel_capture_dates_3 = profiles.load::<Profile>(&conn).expect("Error al ejecutar la query");
    let diesel_capture_dates_4 = skills.load::<Skilss>(&conn).expect("Error al ejecutar la query");

    let mut ctx = tera::Context::new();
        ctx.insert("layoutprofile",&diesel_capture_dates_1);
        ctx.insert("pprofile", &diesel_capture_dates_2);
        ctx.insert("profiles", &diesel_capture_dates_3);
        ctx.insert("skills", &diesel_capture_dates_4);

        HttpResponse::Ok().content_type("text/html").body(
            template_manager.render("profile.html", &ctx).unwrap()
        )
  
}


#[get("/post")]
async fn post(pool: web::Data<DbPool>, template_manager: web::Data<tera::Tera>) -> impl Responder {
    let conn = pool.get().expect("Problemas al traer la base de datos");
    let diesel_capture_dates_1 = posts.load::<Post>(&conn).expect("Error al ejecutar la query");
    let diesel_capture_dates_2 = layoutpost.load::<LayoutPost>(&conn).expect("Error al ejecutar la query");

    let mut ctx = tera::Context::new();
    ctx.insert("posts", &diesel_capture_dates_1);
    ctx.insert("layoutpost", &diesel_capture_dates_2);

    HttpResponse::Ok().content_type("text/html").body(
        template_manager.render("post.html", &ctx).unwrap()
    )

}

#[get("/contact")]
async fn contact(pool: web::Data<DbPool>, template_manager: web::Data<tera::Tera>) -> impl Responder {
    let conn = pool.get().expect("Problemas al traer la base de datos");
    let diesel_capture_dates_1 = p_contact.load::<PContact>(&conn).expect("Error al ejecutar la query");
    let diesel_capture_dates_2 = socials.load::<Social>(&conn).expect("Error al ejecutar la query");
    
    let mut ctx = tera::Context::new();
    ctx.insert("p_contact", &diesel_capture_dates_1);
    ctx.insert("socials", &diesel_capture_dates_2);
    ctx.insert("Msg","Submit");

    HttpResponse::Ok().content_type("text/html").body(
        template_manager.render("contact.html", &ctx).unwrap()
    )

}




#[post("/contact")]
async fn contact_post(data: web::Form<Msg>) -> impl Responder {
    println!("Contact Post  
        firstName:{},
        lastName:{},
        email:{},
        country: {},
        message:{} 
    ",

        data.first_name,
        data.last_name,
        data.email,
        data.country,
        data.message);


    query_insert_msg(
        &data.first_name,
        &data.last_name,
        &data.email,
        &data.country,
        &data.message);
        
    HttpResponse::Ok().body(format!("Contact Post Tank You Contact Test:
        firstName:{},
        lastName:{},
        email:{},
        country: {},
        message:{} 
    ",

        data.first_name,
        data.last_name,
        data.email,
        data.country,
        data.message))
    
}


#[get("/blog/{query_index_get}")]
async fn query_index_get(
    pool: web::Data<DbPool>, 
    template_manager: web::Data<tera::Tera>, 
    blog_slug: web::Path<String>
) -> impl Responder  {
    let conn = pool.get().expect("Problemas al traer la base de datos");

    let url_slug = blog_slug.into_inner();

    match web::block(move || {posts.filter(slug.eq(url_slug)).load::<Post>(&conn)}).await {
        Ok(data) => {
            let data = data.unwrap();

            if data.len() == 0 {
                return HttpResponse::NotFound().finish();
            }

            let data = &data[0];

            let mut ctx = tera::Context::new();
                    ctx.insert("post", data);

            HttpResponse::Ok().content_type("text/html").body(
                template_manager.render("posts.html", &ctx).unwrap()
            )

            // return HttpResponse::Ok().body(format!("{:?}", data));
        },
        Err(_err) => HttpResponse::Ok().body("Error al recibir la data")
    }
}



#[post("/query_index_post")]
async fn query_index_post(pool: web::Data<DbPool>,item:web::Json<NewPostHandler>) -> impl Responder {

    println!("{:?}", item);
    let conn = pool.get().expect("Problemas al traer la base de datos");

    match web::block(move || {Post::create_post(&conn, &item)}).await {
        Ok(data) => {
            return HttpResponse::Ok().body(format!("{:?}", data));
        },
        Err(_err) => HttpResponse::Ok().body("Error al recibir la data")
    }
}

//********************************************* END ACTIX WEB IN USE FOR TEMPLATES*********************************************


//********************************************* PRINCIPAL FUNCTION MAIN*********************************************
#[actix_web::main]
//Funcion  principal
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("db url variable no encontrada");
    let connection = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::builder().build(connection).expect("No se pudo construir la Pool");
/*
     //Funciones para insertar en la base de datos 
    //index
    //Insertar carousel #1
    query_insert_carousel(
        "Full Stack Developer",
        "Where I study?",
        "I increase my skills in platzi",
        "I am autonomous in my personal development as a systems engineer",
        "I am a student of systems engineering at the University of San Buenaventura de Cali and I am studying 5 semester",
        "With platzi I acquire new soft skills in order to improve my profile",
        "https://cdn.pixabay.com/photo/2022/04/06/04/47/ai-7114792__340.jpg",
        "https://cdn.discordapp.com/attachments/410558873951404053/1019345002398228600/carousel_u.jpg",
        "https://cdn.discordapp.com/attachments/410558873951404053/1019346398161940530/garfield-programmer.jpg",
        "/contact",
        "https://www.usbcali.edu.co/",
        "https://platzi.com/p/thenowrock/");

    //Insertar symbols #3
    //#1
    query_insert_symbol(
        "Udemy",
        "udemy is another of the platforms on which I keep updating to improve my skills as a developer.",
        "https://cdn.discordapp.com/attachments/410558873951404053/1016491415792001165/udemy-img-x200.png",
        "https://www.udemy.com/user/david-jimenez-190/");
    //#2    
    query_insert_symbol(
        "IBM",
        "ibm is the pioneer company in computers and offers us a lot of knowledge for the creation and development of software.",
        "https://cdn.discordapp.com/attachments/410558873951404053/1016492815217332285/ibm2.png",
        "https://www.ibm.com/co-es"); 
    //#3    
    query_insert_symbol(
        "Rust",
        "this website has been developed thanks to the implementation of the rust language for web development or web assembly",
        "https://cdn.discordapp.com/attachments/410558873951404053/1016491415582294076/rust-img-x200.png",
        "https://www.rust-lang.org/");      

    //Insertar parrafos en index #3
    //#1
    query_insert_p_index(
        "GROWING TECHNOLOGIES.",
        "New Era 3.0",
        " Since my professional growth and in the ICT area I have seen a great development in the use of new technologies that improve the quality of software development through the following tools that are changing the technological world as we know it: RUST, BOOTSTRAP 5, APIS, DATA STRUCTURES, DATABASES, WEB APPS.",
        "The arrival of the new era of the web, what we mean by this is that we see new development alternatives such as web 3.0, which consists of decentralized technology developments, cryptocurrencies, digital wallets, crypto exchange networks, metaverses, nfts.",
        "That is why I seek to prepare myself and satisfy the new technological needs that the world acclaims that you learn.",
        "https://cdn.discordapp.com/attachments/410558873951404053/1016505600496119878/web1.png",
    );
    //#2   
    query_insert_p_index(
        "How we work? ",
        "Agile methodologies",
        "Scrum is a management process that reduces complexity in product development to meet customer needs. Management and Scrum teams work together around requirements and technologies to deliver incrementally working products using empiricism.",
        "Scrum is a simple framework that promotes team collaboration to achieve complex product development.",
        "",
        "https://cdn.discordapp.com/attachments/410558873951404053/1016498990138798122/scrum1.png",
    );
    //#3   
    query_insert_p_index(
        "Goals when working.",
        "Productivity",
        " In order to generate quality software, we write code and create projects based on designs, standards and agile methodologies to meet the expectations of our clients, in addition to guaranteeing their quality.",
        "",
        "",
        "https://cdn.discordapp.com/attachments/410558873951404053/1016501971827576903/productivdad1.png",
    );   
    //Services
    //Insertar layout service #1
    //#1
    query_insert_layout_service(
        "Services Full Stack",
        "We are interested in creating high quality software products with security standards, optimization, testing, agile methodologies, tdd, scrum, ux.
        We work remotely from the country of Colombia",
        "http://webeys.com/wp-content/uploads/2022/08/fullstack-development-services-1585821765-5355051.png",
        "Development",
        "Contact",
        "/",
        "/contact",
    );
    //Insertar parrafo service #1
    //#1
    query_insert_p_service(
        "My Services.",
        "Profesional Services",
        "My service is based on productivity and delivery of results with agile practices for high-reach results.",
        "I have experience in software development as a systems engineering student, I have high knowledge to develop applications, desktop programs, web apps, APIs, interfaces, tests and mobile services.",
        "I have experience in managing and creating relational databases with postgreSQL, MySQL, such as managing queries and triggers with SQL. As non-relational with MongoDB I also handle FireBase.",
        "https://www.ngeeks.com/wp-content/uploads/2021/09/cursos-programacion.jpg",
    );
    //Insertar service #3
    //#1
    query_insert_service(
        "FRONT-END",
        "Html5,CSS,Bootstrap,JavaScript,React,Angular,Actix-Web",
        "https://www.kampuskod.com/wp-content/uploads/2020/12/frontend-teknolojileri.png",
        "https://i.ytimg.com/vi/NWRU6P6NFsc/maxresdefault.jpg",
        "https://actix.rs/img/jumbotron.jpg",
        true,
    );
    //#2
    query_insert_service(
        "BACK-END",
        "C++,Rust,Python,NodeJS,Java,Git,GitHub",
        "https://luyenkimmau.com.vn/backend-developer-la-gi/imager_9023.jpg",
        "https://blog.facialix.com/wp-content/uploads/2022/02/5fd3903b41d20bd2244ec3fd_programminglanguagesstickers.jpg",
        "https://bs-uploads.toptal.io/blackfish-uploads/components/blog_post_page/content/cover_image_file/cover_image/907884/retina_1708x683_cover-0816-C__Mistakes-Waldek_Newsletter-8ca5fb6eacb673aaad1fe4bfaf4ce205-2f26062f759e8698edd8c5d77b82b992.png",
        true,
    );
    //#3
    query_insert_service(
        "DATA-BASES",
        "MongoDB,PostgreSQL,MySQL,FireBase",
        "https://vietnix.vn/wp-content/uploads/2022/07/postgresql-la-gi-1024x536.webp",
        "https://tecnoticias.net/wp-content/uploads/2021/02/mongodb-atlas-google-cloud-partnership-nosql-databases-integrations-2.jpg",
        "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcStOSAYpLf1bA4JobhA6raFhSG-yHV70U7wAbVsp3jw2FTaVuvMVDVGTrfp8qHFHK-Uasw&usqp=CAU",
        true,
    );

        //Profile
    //Insertar layout profile #1
    query_insert_layout_profile(
        "Development Jobs",
        "Quickly design and customize responsive mobile-first sites with Bootstrap, the world’s most popular front-end open source toolkit, featuring Sass variables and mixins, responsive grid system, extensive prebuilt components, and powerful JavaScript plugins.",
        "https://images.pexels.com/photos/340152/pexels-photo-340152.jpeg?auto=compress&cs=tinysrgb&w=1600",
        "Post",
        "Contact",
        "/post",
        "/contact",
    );
    //Insertar parrafo profile #1
    query_insert_p_profile(
        "My Profile.",
        "Studen",
        "I am currently a systems engineering student at the University of San Buenaventura. I have experience in managing both front-end and back-end technologies.",
        " I don't have previous experience in a June full stack developer job, but my goal is to get that experience with a company that gives me an opportunity.",
        "",
        "https://media.discordapp.net/attachments/410558873951404053/1016533189990301767/apps2.png ",
    );
    //Insertar  profile #1
    query_insert_profile(
        "Juan David",
        "Jimenez",
        "thenowrock@gmail.com",
        "https://github.com/thenowrrock",
        "+57 3178178423",
        "Santa Elena - Cali",
        "Cali",
        "Colombia",
        "hello how am i the developer juan david nice to be able to connect with you",
        true);

    //Insertar skilss profile #1
    //Rust
    query_insert_skills(
        "Rust",
        "Mid",
        "Hybrid",
        "https://jagonzalez.org/wp-content/uploads/2022/08/Diesel-vs-SQLx-interacting-databases-Rust.png",
    );

    //C++
    query_insert_skills(
        "C++",
        "Mid",
        "Hybrid",
        "https://cdn.discordapp.com/attachments/410558873951404053/1019359130210799697/c.jpeg",
    );

    //Python
    query_insert_skills(
        "Python",
        "Mid",
        "Full",
        "https://cdn.discordapp.com/attachments/410558873951404053/1019359219683704902/python.jpg",
    );
    
    //JavaScript
    query_insert_skills(
        "JavaScript",
        "Mid",
        "Front-Back",
        "https://cdn.discordapp.com/attachments/410558873951404053/1019359238813929493/javascript.jpg",
    );

    //Java
    query_insert_skills(
        "Java",
        "Mid",
        "Full",
        "https://cdn.discordapp.com/attachments/410558873951404053/1019359271663706203/java.jpg",
    );

    //GitHub
    query_insert_skills(
        "GitHub",
        "Mid",
        "Repositoris",
        "https://cdn.discordapp.com/attachments/410558873951404053/1019359326453899264/gihub.png",
    );

    //NodeJs
    query_insert_skills(
        "NodeJs",
        "Nov",
        "Hybrid",
        "https://cdn.discordapp.com/attachments/410558873951404053/1019359370661867622/nodejs.png",
    );

    //Angular
    query_insert_skills(
        "Angular",
        "Mid",
        "Framework",
        "https://cdn.discordapp.com/attachments/410558873951404053/1019359395257266257/angular.png",
    );


    //Raeact
    query_insert_skills(
        "Raeact",
        "Mid",
        "Framework",
        "https://cdn.discordapp.com/attachments/410558873951404053/1019359444083155084/react.png",
    );

    //Html5
    query_insert_skills(
        "Html5",
        "Full",
        "Front",
        "https://cdn.discordapp.com/attachments/410558873951404053/1019359485841657876/html.png",
    );    


    //Css3
    query_insert_skills(
        "Css3",
        "Mid",
        "Front",
        "https://cdn.discordapp.com/attachments/410558873951404053/1019359516418134156/css.jpg",
    );


    //Bootstrap
    query_insert_skills(
        "Bootstrap",
        "Mid",
        "Framework",
        "https://cdn.discordapp.com/attachments/410558873951404053/1019359561792102451/bootstrap.png",
    );

    //Actix-Web
    query_insert_skills(
        "Actix-Web",
        "Mid",
        "Framework",
        "https://cdn.discordapp.com/attachments/410558873951404053/1019359579257192538/Actix-Web.png",
    );

   //SQL
    query_insert_skills(
        "SQL",
        "Mid",
        "Back",
        "https://cdn.discordapp.com/attachments/410558873951404053/1019359597775040522/sql.png",
    );

   //PostgreSQL
    query_insert_skills(
        "PostgreSQL",
        "Mid",
        "Database",
        "https://cdn.discordapp.com/attachments/410558873951404053/1019359623813279764/postgresql.jpg",
    );
    
    //FireBase
    query_insert_skills(
        "FireBase",
        "Nov",
        "Database",
        "https://cdn.discordapp.com/attachments/410558873951404053/1019360919056285787/firebase.jpg",
    );

    //MongoDB
    query_insert_skills(
        "MongoDB",
        "Nov",
        "Database",
        "https://cdn.discordapp.com/attachments/410558873951404053/1019359885953089566/mongodb.png",
    );



    //Insert layout post #1
       query_insert_layout_post(
        "My post",
        "This is my publication space for some personal works and projects in which I have worked and applied what I have learned.",
        "https://www.locurainformaticadigital.com/wp-content/uploads/2017/03/stack-overflow.png",
        "https://www.hispatecno.net/wp-content/uploads/2020/11/1-qtazbwhg6xwe-dhwfiqfma.png",
        "Coming soon",
        "/",
    );

    //Insertar post #1
    query_insert_post(
        "Game role",
        "link de game role",
        "Game role in rust",
        "https://cdn.discordapp.com/attachments/410558873951404053/1019004439106965514/icon.png",
        true,            
     );

         //Insertar post #2
    query_insert_post(
        "Create Simple Calculate JS",
        "https://github.com/thenowrrock/calculadora_JS",
        "Calculadora Js",
        "https://i.ytimg.com/vi/f18O80XRe9Q/maxresdefault.jpg",
        true,            
     );

   //Insertar post #3
    query_insert_post(
        "Create Simple Calculate Rust",
        "https://github.com/thenowrrock/calculadora-cientifica-rust",
        "Calculadora Rust",
        "https://play-lh.googleusercontent.com/r1XrRuJ30EPw4sOM0JjP0lV9yGNqnYejlRf_C6mNLu15lTcidSAEV4XB3XBhejpJQw",
        true,            
     );

  //Insertar post #4
    query_insert_post(
        "Market place react-v1",
        "https://github.com/thenowrrock/market-place-react-v1",
        "Market Place react",
        "https://github.com/thenowrrock/market-place-react-v1/blob/main/Screenshot%202022-08-16%20161138.png?raw=true",
        true,            
     );

    //Insertar post #5
    query_insert_post(
        "Sitioweb angular ",
        "https://github.com/thenowrrock/sitioweb-angular",
        "angular Place ",
        "https://github.com/thenowrrock/sitioweb-angular/blob/main/login.png?raw=true",
        true,            
     );

    //Insertar post #6
    query_insert_post(
        "Game-role Rust ",
        "https://github.com/thenowrrock/game-role-rust",
        "Game role rs ",
        "https://miro.medium.com/max/709/1*DicTpPZN_m4qMeGCToPW6g.png",
        true,            
     );


    //Insertar parrafos contact #1
        query_insert_p_contact(
        "Can you find me ",
        "I am a daily user of the internet since it allows me to communicate how to learn from the comfort of my home, as well as with the profiles of some social networks so that you can contact me and see my personal development as a full stack.",
        "Bmx fresstyle",
        "Colombia is a beautiful country in which we are full of entrepreneurs and technology enthusiasts",
        "Msg",
        "Bmx notice",
        "Colombia notice",
        "/",
        "https://www.instagram.com/reel/ChvIho8pZkw/?utm_source=ig_web_copy_link",
        "https://colombiareports.com/colombia-recommends-stricter-covid-19-measures-for-air-travelers/",
        );
        
        query_insert_social(
         "Social Juan",
         "https://t.me/+FjUS1UH8AoRkMzAx",
         "https://www.linkedin.com/in/jd-giz-7768a2222/",
         "https://github.com/thenowrrock",
         "https://www.instagram.com/poca_lith/",
         "https://chat.whatsapp.com/JDxfAdCrhusGnUfQVLnbsS",
         "https://twitter.com/TheNowRockHD",
         true);  
    

   //Time 
   use chrono::Local;
   use chrono::DateTime;
   let local_date: DateTime<Local> = Local::now();
   println!("Local Date: {}", local_date);

   use chrono::Local;
   use chrono::DateTime;
   let local_date: DateTime<Local> = Local::now();
   let string_date = local_date.format("%Y-%m-%d %H:%M:%S").to_string();
   println!("Local Date: {}", string_date);
   */

         

           
    /*
    //Funciones para consultar en la base de datos
    //index
    query_select_all_carousel();
    query_select_all_symbol();
    query_select_all_p_index();
    //Services
    query_select_all_layout_service();
    query_select_all_p_service();
    query_select_all_service();
    //Profile
    query_select_all_layout_profile();
    query_select_all_p_profile();
    query_select_all_profile();
    query_select_all_skills();
    //Post
    query_select_all_layout_post();
    query_select_all_post();        
    //Contact
    query_selct_all_p_contact();
    query_select_all_social();
    query_selct_all_msg();
    */        
    
    let port = env::var("PORT").expect("La variable de entorno PORT no existe.");
    let port:u16 = port.parse().unwrap();

    //Crear el servidor
    HttpServer::new(move || {
        //Use Tera
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();


        App::new()  
        .service(index)
        .service(query_index_get) //para que no hayan errores en la pool de los datos se deben de crear una copia  de los datos y inicializarla en el servidor con una nueva
        .service(query_index_post)
        .service(profile)
        .service(service)
        .service(post)
        .service(contact)
        .service(contact_post)
        .app_data(web::Data::new(pool.clone()))
        .app_data(web::Data::new(tera.clone()))
        
    }).bind(("0.0.0.0", port)).unwrap().run().await
}
//********************************************* END PRINCIPAL FUNCTION MAIN*********************************************
 




/*
ESte error es porque heroku te desabilita el servicio de postgres y no te da mantenimiento de tu servidor
thread 'main' panicked at 'No se pudo construir la Pool: 
Error(Some("connection to server at \"ec2-44-205-112-253.compute-1.amazonaws.com\" (44.205.112.253), 
port 5432 failed: FATAL:  password authentication failed for user \"rnnvfbvkkiaxgl\"\nconnection to server at \"ec2-44-205-112-253.compute-1.amazonaws.com\" (44.205.112.253), 
port 5432 failed: FATAL:  no pg_hba.conf entry for host \"186.80.98.57\", 
user \"rnnvfbvkkiaxgl\", 
database \"df1ae5c0ukj3ef\", 
no encryption\n"))', 
src/main.rs:189:50
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

/*
TIPS PARA LOS COMENTARIOS PARA DOCUMENTAR MY CODE DIOSSSS TODO LO QUE FALTA POR HACER "THANKS GOOD , FOR THE GOOD TIMES IN LIVE CODING "
 *aaaa
 !aaaa
 ? aaaa
 TODO : aa
 @param MyParam
*/

*/
