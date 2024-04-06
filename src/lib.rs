//! # Oblivion
//!
//! Oblivion is a Rust implementation of Oblivion,an end-to-end encryption protocol developed by Turbolane to secure information.
//! It greatly improves the security, stability, and concurrency of Oblivion based on the Python implementation.
//!
//! Since the encryption algorithm required in the Oblivion protocol is the ECDHE algorithm,
//! it is based on an efficient and secure key derivation method,
//! which makes it possible to apply it to message dispatching and just-in-time communication.
pub extern crate oblivion_codegen;
pub extern crate proc_macro;
pub mod api;
pub mod exceptions;
pub mod sessions;

/// # Oblivion Utilities
///
/// Oblivion utility classes provide key creation, data encryption and decryption, and request resolution processing methods.
pub mod utils {
    pub mod decryptor;
    pub mod encryptor;
    pub mod gear;
    pub mod generator;
    pub mod parser;
}

/// # Oblivion Models
///
/// Oblivion provides all front- and back-end models, including packet building as well as client-side and server-side building.
pub mod models {
    pub mod client;
    pub mod handler;
    pub mod packet;
    pub mod render;
    pub mod router;
    pub mod server;
}

/// Absolute Routing Macros
///
/// Routing can be simply implemented using routing macros:
///
/// ```rust
/// use oblivion::path_route;
/// use oblivion::utils::parser::OblivionRequest;
/// use oblivion::models::render::{BaseResponse, Response};
/// use oblivion_codegen::async_route;
/// use oblivion::models::router::Router;
///
/// #[async_route]
/// fn welcome(mut req: OblivionRequest) -> Response {
///     Ok(BaseResponse::TextResponse(
///        format!("欢迎进入信息绝对安全区, 来自[{}]的朋友", req.get_ip()),
///        200,
///     ))
/// }
///
/// let mut router = Router::new();
/// path_route!(&mut router, "/welcome" => welcome);
/// ```
///
/// The above route will direct requests with the path `/welcome` or `/welcome/`.
#[macro_export]
macro_rules! path_route {
    ($router:expr, $path:expr => $handler:ident) => {{
        let route = $crate::models::router::Route::new($handler);
        $router.regist(
            $crate::models::router::RoutePath::new($path, $crate::models::router::RouteType::Path),
            route,
        );
    }};
}

/// Startswith Routing Macros
///
/// Starting routes can be simply implemented using the start route macro:
///
/// ```rust
/// use oblivion::startswith_route;
/// use oblivion::utils::parser::OblivionRequest;
/// use oblivion::models::render::{BaseResponse, Response};
/// use oblivion_codegen::async_route;
/// use oblivion::models::router::Router;
///
/// #[async_route]
/// fn welcome(mut req: OblivionRequest) -> Response {
///     Ok(BaseResponse::TextResponse(
///        format!("欢迎进入信息绝对安全区, 来自[{}]的朋友", req.get_ip()),
///        200,
///     ))
/// }
///
/// let mut router = Router::new();
/// startswith_route!(router, "/welcome" => welcome);
/// ```
///
/// The above route will direct all Oblivion Location Path String starting with `/welcome`.
#[macro_export]
macro_rules! startswith_route {
    ($router:expr, $path:expr => $handler:ident) => {{
        let route = $crate::models::router::Route::new($handler);
        $router.regist(
            $crate::models::router::RoutePath::new(
                $path,
                $crate::models::router::RouteType::StartswithPath,
            ),
            route,
        );
    }};
}

/// Regular routing macro
///
/// Regular routing can be simply implemented using regular routing macros:
///
/// ```rust
/// use futures::future::{BoxFuture, FutureExt};
/// use oblivion::regex_route;
/// use oblivion::utils::parser::OblivionRequest;
/// use oblivion::models::render::{BaseResponse, Response};
/// use oblivion_codegen::async_route;
/// use oblivion::models::router::Router;
///
/// #[async_route]
/// fn welcome(mut req: OblivionRequest) -> Response {
///     Ok(BaseResponse::TextResponse(
///        format!("欢迎进入信息绝对安全区, 来自[{}]的朋友", req.get_ip()),
///        200,
///     ))
/// }
///
/// let mut router = Router::new();
/// regex_route!(router, r"^/welcome/.*" => welcome);
/// ```
///
/// The above route will direct all Oblivion Location Path String starting with `/welcome/`.
///
/// You can also use `^/. *` to hijack all routes.
#[macro_export]
macro_rules! regex_route {
    ($router:expr, $path:expr => $handler:ident) => {{
        let route = $crate::models::router::Route::new($handler);
        $router.regist(
            $crate::models::router::RoutePath::new(
                $path,
                $crate::models::router::RouteType::RegexPath,
            ),
            route,
        );
    }};
}
