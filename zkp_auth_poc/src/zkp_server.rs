pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

mod constants;
mod utils;

use constants::public;
use tokio::sync::Mutex;
use tonic::{transport::Server, Code, Request, Response, Status};

use num_bigint::BigUint;
use num_traits::Zero;
use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use utils::{deserialize, random_number, random_string, serialize, verify, VerifyParams};
use zkp_auth::auth_server::{Auth, AuthServer};
use zkp_auth::{
    AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest,
    AuthenticationChallengeResponse, RegisterRequest, RegisterResponse,
};

#[derive(Debug, Default, Clone)]
pub struct UserInfo {
    pub y1: BigUint,
    pub y2: BigUint,
    pub r1: BigUint,
    pub r2: BigUint,
    pub c: BigUint,
    pub session_id: String,
}

#[derive(Debug, Default)]
pub struct AuthService {
    pub user_data: Mutex<HashMap<String, UserInfo>>,
    pub auth_info: Mutex<HashMap<String, String>>,
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        println!("Request to register: {:?}", request);
        let request = request.into_inner();
        let user_id = request.user;

        let user_data = UserInfo {
            y1: deserialize(&request.y1),
            y2: deserialize(&request.y2),
            r1: BigUint::zero(),
            r2: BigUint::zero(),
            c: BigUint::zero(),
            session_id: String::new(),
        };

        // Store y1 and y2 for the user
        // For PoC we simply store in memory
        let mut user_data_store = self.user_data.lock().await;
        user_data_store.insert(user_id, user_data.clone());

        Ok(Response::new(RegisterResponse {}))
    }

    async fn create_authentication_challenge(
        &self,
        request: tonic::Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        println!("Request for an authentication-challenge: {:?}", request);

        let request = request.into_inner();
        let user_id = request.user;

        let mut user_data_store = self.user_data.lock().await;

        if let Some(user_data) = user_data_store.get_mut(&user_id) {
            let auth_id = random_string(6);
            let c = random_number();

            user_data.r1 = deserialize(&request.r1);
            user_data.r2 = deserialize(&request.r2);
            user_data.c = c.clone();

            println!("{:?}", user_data_store);

            let mut auth_data_store = self.auth_info.lock().await;
            auth_data_store.insert(auth_id.clone(), user_id);

            Ok(Response::new(AuthenticationChallengeResponse {
                auth_id,
                c: serialize(&c),
            }))
        } else {
            Err(Status::new(
                Code::NotFound,
                format!("We did not find the user with id: {:?}", user_id),
            ))
        }
    }

    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> Result<tonic::Response<AuthenticationAnswerResponse>, Status> {
        println!("Request for authentication answer is: {:?}", request);

        let request = request.into_inner();
        let auth_id = request.auth_id;

        let auth_data_store = self.auth_info.lock().await;

        if let Some(user_id) = auth_data_store.get(&auth_id) {
            let mut user_data_store = self.user_data.lock().await;

            if let Some(user_data) = user_data_store.get_mut(user_id) {
                let s = deserialize(&request.s);
                let y1 = &user_data.y1;
                let y2 = &user_data.y2;
                let r1 = &user_data.r1;
                let r2 = &user_data.r2;
                let c = &user_data.c;

                let (g, h, p, _) = public();

                let params = VerifyParams {
                    g: &g,
                    h: &h,
                    p: &p,
                    y1,
                    y2,
                    r1,
                    r2,
                    c,
                    s: &s,
                };

                if verify(params) {
                    let session_id = random_string(9);
                    user_data.session_id = session_id.clone();

                    println!("This is the user_data_store: {:?}", user_data_store);

                    Ok(Response::new(AuthenticationAnswerResponse { session_id }))
                } else {
                    Err(Status::new(
                        Code::Unauthenticated,
                        "Verification failed".to_string(),
                    ))
                }
            } else {
                Err(Status::new(
                    Code::NotFound,
                    format!("Error, unknown user with id: {:?}", user_id),
                ))
            }
        } else {
            Err(Status::new(
                Code::NotFound,
                format!("Error, unknown auth id: {:?}", auth_id),
            ))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Server is up & running");

    let addr: SocketAddr = env::var("SERVER_ADDRESS")
        .unwrap_or("0.0.0.0:50051".to_string())
        .parse()?;
    let auth_service = AuthService::default();

    Server::builder()
        .add_service(AuthServer::new(auth_service))
        .serve(addr)
        .await?;

    Ok(())
}
