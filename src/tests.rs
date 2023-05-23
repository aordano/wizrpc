use std::time::Duration;

use crate::{Client, Method, Param, Request};
use tokio::time::sleep;

#[tokio::test]
async fn test_client_new() {
    Client::default().await.unwrap();
}

/* #[tokio::test]
async fn test_ping() {
    let client = Client::default().await.unwrap();

    client
        .register_device("Office".to_string(), "office.wiz.local".to_string())
        .await
        .unwrap();

    client
        .register_device("Room".to_string(), "room.wiz.local".to_string())
        .await
        .unwrap();

    client.ping("Office".to_string()).await.unwrap();
    client.ping("Room".to_string()).await.unwrap();
} */

/*
#[tokio::test]
async fn test_ping_5() {
    let client = Client::default().await.unwrap();

    client
        .register("Office".to_string(), "office.wiz.local".to_string())
        .await
        .unwrap();

    let amount = 0..5;

    for _ in amount {
        client.ping("Office".to_string()).await.unwrap();
    }
}
*/
// #[tokio::test]
// async fn test_ping_25() {
//     let client = Client::default().await.unwrap();

//     client
//         .register_device("Office".to_string(), "office.wiz.local".to_string())
//         .await
//         .unwrap();

//     let amount = 0..25;

//     for _ in amount {
//         client.ping("Office".to_string()).await.unwrap();
//     }
// }

// #[tokio::test]
// async fn test_send_no_params_non_modyfing() {
//     let client = Client::default().await.unwrap();

//     client
//         .register_device("Office".to_string(), "office.wiz.local".to_string())
//         .await
//         .unwrap();

//     let request = Request::new(Some("Office".to_string()), Method::GetSystemConfig, None);
//     let module = client
//         .send(request)
//         .await
//         .unwrap()
//         .result
//         .unwrap()
//         .module_name
//         .unwrap();

//     assert_eq!(module.as_str(), "ESP01_SHRGB1C_31");
// }

// #[tokio::test]
// async fn test_send_no_params_non_modyfing_25() {
//     let client = Client::default().await.unwrap();

//     client
//         .register_device("Office".to_string(), "office.wiz.local".to_string())
//         .await
//         .unwrap();

//     let amount = 0..25;

//     for _ in amount {
//         let request = Request::new(Some("Office".to_string()), Method::GetSystemConfig, None);
//         let module = client
//             .send(request)
//             .await
//             .unwrap()
//             .result
//             .unwrap()
//             .module_name
//             .unwrap();

//         assert_eq!(module.as_str(), "ESP01_SHRGB1C_31");
//     }
// }

/*

#[tokio::test]
async fn test_send_no_params() {
    let client = Client::default().await.unwrap();

    client
        .register("Office".to_string(), "office.wiz.local".to_string())
        .await
        .unwrap();

    let request = Request::new("Office".to_string(), Method::Reboot, None);

    let reboot = client
        .send(request)
        .await
        .unwrap()
        .result
        .unwrap()
        .success
        .unwrap();

    assert_eq!(reboot, true);
}
*/

/*
#[tokio::test]
async fn test_send() {
    let client = Client::default().await.unwrap();

    client
        .register_device("Office".to_string(), "office.wiz.local".to_string())
        .await
        .unwrap();

    let request1 = Request::new(
        Some("Office".to_string()),
        Method::SetState,
        Some(vec![Param::State(Some(false))]),
    );

    let request2 = Request::new(
        Some("Office".to_string()),
        Method::SetState,
        Some(vec![Param::State(Some(true))]),
    );

    let off = client
        .send(request1)
        .await
        .unwrap()
        .result
        .unwrap()
        .success
        .unwrap();

    assert_eq!(off, true);

    sleep(Duration::from_millis(100)).await;

    let on = client
        .send(request2)
        .await
        .unwrap()
        .result
        .unwrap()
        .success
        .unwrap();
    assert_eq!(on, true);
}
 */

// #[tokio::test]
// async fn test_client_discover() {
//     let mut client = Client::default().await.unwrap();

//     client.discover().await.unwrap();

//     assert_ne!(client.devices().unwrap().len(), 0);
// }
