use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
};

use crate::domain::services::auth_service::AuthService;

pub struct JwtMiddleware {
    auth_service: Rc<AuthService>,
}

impl JwtMiddleware {
    pub fn new(auth_service: AuthService) -> Self {
        Self {
            auth_service: Rc::new(auth_service),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareService {
            service,
            auth_service: self.auth_service.clone(),
        }))
    }
}

pub struct JwtMiddlewareService<S> {
    service: S,
    auth_service: Rc<AuthService>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_service = self.auth_service.clone();
        
        // Extract JWT token from Authorization header
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "));

        match token {
            Some(token) => {
                match auth_service.validate_jwt(token) {
                    Ok(claims) => {
                        // Add claims to request extensions
                        req.extensions_mut().insert(claims);
                        let fut = self.service.call(req);
                        Box::pin(async move { fut.await })
                    }
                    Err(_) => {
                        Box::pin(async move {
                            Ok(req.error_response(
                                actix_web::error::ErrorUnauthorized("Invalid token")
                            ))
                        })
                    }
                }
            }
            None => {
                Box::pin(async move {
                    Ok(req.error_response(
                        actix_web::error::ErrorUnauthorized("Missing token")
                    ))
                })
            }
        }
    }
}