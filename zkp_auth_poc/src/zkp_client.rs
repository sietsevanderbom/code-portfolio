pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

mod constants;
mod utils;

use constants::public;
use num_bigint::BigUint;
use proceed::any_or_quit_with;
use std::env;
use std::io::stdin;
use tonic::transport::Channel;
use utils::{deserialize, random_number, serialize, solve};
use zkp_auth::auth_client::AuthClient;
use zkp_auth::{AuthenticationAnswerRequest, AuthenticationChallengeRequest, RegisterRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_address = env::var("SERVER_ADDRESS").unwrap_or("0.0.0.0:50051".to_string());

    let server_address = if server_address.starts_with("http://") {
        server_address
    } else {
        format!("http://{}", server_address)
    };

    let channel = Channel::from_shared(server_address)?.connect().await?;

    let mut zkp_client = AuthClient::new(channel);

    let mut buffer = String::new();

    let (g, h, p, q) = public();

    println!("Press any key to start the registration process, or 'q' to quit");
    if !any_or_quit_with('q') {
        println!("Quitting...");
        return Ok(());
    };

    println!("Enter your user-id:");
    stdin().read_line(&mut buffer).expect("Error: no input");
    let user_id = buffer.trim().to_string();

    println!("Enter your user-secret (a very large number):");
    buffer = String::new();
    stdin().read_line(&mut buffer).expect("Error: no input");

    let x_entry = buffer.trim();
    let x = x_entry
        .parse::<BigUint>()
        .expect("Expected a valid large number");

    let y1 = &g.modpow(&x, &p);
    let y2 = &h.modpow(&x, &p);
    let request = tonic::Request::new(RegisterRequest {
        user: user_id.clone(),
        y1: serialize(y1),
        y2: serialize(y2),
    });
    let _response = zkp_client.register(request).await?;

    let k = random_number();
    let r1 = &g.modpow(&k, &p);
    let r2 = &h.modpow(&k, &p);

    let request = tonic::Request::new(AuthenticationChallengeRequest {
        user: user_id,
        r1: serialize(r1),
        r2: serialize(r2),
    });

    let response = zkp_client.create_authentication_challenge(request).await?;

    let response = response.into_inner();
    let auth_id = response.auth_id;
    let c = deserialize(&response.c);

    let s = solve(&x, &k, &c, &q);

    let request = tonic::Request::new(AuthenticationAnswerRequest {
        auth_id,
        s: serialize(&s),
    });

    let response = zkp_client.verify_authentication(request).await?;

    let response = response.into_inner();
    let session_id = response.session_id;

    println!(
        "You successfully logged in with session id: {:?}",
        session_id
    );

    Ok(())
}
