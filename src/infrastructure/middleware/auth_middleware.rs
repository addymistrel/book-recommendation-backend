use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
};

use crate::domain::services::auth_service::{AuthService, Claims};

pub struct AuthMiddleware {
    auth_service: Rc<AuthService>,
}

impl AuthMiddleware {
    pub fn new(auth_service: AuthService) -> Self {
        Self {
            auth_service: Rc::new(auth_service),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service,
            auth_service: self.auth_service.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
    auth_service: Rc<AuthService>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
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
        
        // Skip auth for public routes
        let path = req.path();
        if path.starts_with("/api/auth/") || 
           path.starts_with("/swagger-ui/") || 
           path.starts_with("/api-docs/") ||
           path == "/health" {
            let fut = self.service.call(req);
            return Box::pin(async move { fut.await });
        }

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
                                actix_web::error::ErrorUnauthorized("Invalid or expired token")
                            ))
                        })
                    }
                }
            }
            None => {
                Box::pin(async move {
                    Ok(req.error_response(
                        actix_web::error::ErrorUnauthorized("Missing authorization token")
                    ))
                })
            }
        }
    }
}