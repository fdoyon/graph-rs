use graph_error::GraphError;
use graph_rs::http::AsyncIterator;
use graph_rs::http::NextSession;
use graph_rs::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use test_tools::oauthrequest::OAuthRequest;
use test_tools::oauthrequest::ASYNC_THROTTLE_MUTEX;
use test_tools::support::cleanup::AsyncCleanUp;
use test_tools::FileUtils;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

#[tokio::test]
async fn async_download() {
    let _lock = ASYNC_THROTTLE_MUTEX.lock().await;

    if let Some((id, token)) = OAuthRequest::request_access_token_async().await {
        let file_location = "./test_files/download_async.txt";
        let mut clean_up = AsyncCleanUp::new_remove_existing(file_location);
        clean_up.rm_files(file_location.into());

        let bearer = token.bearer_token();
        let client = Graph::new_async(&bearer);

        let download = client
            .v1()
            .users(id.as_str())
            .drive()
            .download(":/download_async.txt:", "./test_files");

        let path_buf: PathBuf = download.send().await.unwrap();
        FileUtils::verify_contents_async(path_buf.as_path(), "ONEDRIVE ASYNC DOWNLOAD TEST".into())
            .await;
    }
}

#[tokio::test]
async fn async_upload_session() {
    let _lock = ASYNC_THROTTLE_MUTEX.lock().await;

    if let Some((id, token)) = OAuthRequest::request_access_token_async().await {
        let client = Graph::new_async(token.bearer_token());

        let upload = serde_json::json!({
            "@microsoft.graph.conflictBehavior": Some("fail".to_string())
        });

        let response = client
            .v1()
            .users(id.as_str())
            .drive()
            .upload_session(
                ":/async_upload_session.txt:",
                "./test_files/async_upload_session.txt",
                &upload,
            )
            .send()
            .await;

        if let Err(e) = response {
            panic!(
                "Request error. Upload session new on initial request. Error: {:#?}",
                e
            );
        } else if let Ok(mut session) = response {
            let cancel_request = session.cancel().await;

            while let Some(next) = session.next().await {
                match next {
                    Ok(NextSession::Next(response)) => {
                        assert!(!GraphError::is_error(response.status()));
                    },
                    Ok(NextSession::Done(response)) => {
                        assert!(!GraphError::is_error(response.status()));
                        let drive_item = response.body();
                        let drive_item_id =
                            drive_item["id"].as_str().unwrap_or_default().to_string();
                        tokio::time::delay_for(Duration::from_secs(3)).await;

                        let delete_res = client
                            .v1()
                            .users(id.as_str())
                            .drive()
                            .delete(drive_item_id.as_str())
                            .send()
                            .await;

                        if let Ok(response) = delete_res {
                            assert!(
                                response.status() == 200 ||
                                    response.status() == 201 ||
                                    response.status() == 204
                            );
                        } else if let Err(e) = delete_res {
                            panic!("Request error. Upload session new. Error: {:#?}", e);
                        }
                        break;
                    },
                    Err(e) => {
                        let _ = cancel_request.send().await.unwrap();
                        panic!("Request error. Upload session new. Error: {:#?}", e);
                    },
                }
            }
        }
    }
}

#[tokio::test]
async fn async_upload_session_standalone_request() {
    let _lock = ASYNC_THROTTLE_MUTEX.lock().await;

    if let Some((id, token)) = OAuthRequest::request_access_token_async().await {
        let client = Graph::new_async(token.bearer_token());

        let upload = serde_json::json!({
            "@microsoft.graph.conflictBehavior": Some("fail".to_string())
        });

        let request = client
            .v1()
            .users(id.as_str())
            .drive()
            .upload_session(
                ":/async_upload_session_request.txt:",
                "./test_files/async_upload_session_request.txt",
                &upload,
            )
            .build()
            .await;

        let response = request.send().await;

        if let Err(e) = response {
            panic!(
                "Request error. Upload session new on initial request. Error: {:#?}",
                e
            );
        } else if let Ok(mut session) = response {
            let cancel_request = session.cancel().await;

            while let Some(next) = session.next().await {
                match next {
                    Ok(NextSession::Next(response)) => {
                        assert!(!GraphError::is_error(response.status()));
                    },
                    Ok(NextSession::Done(response)) => {
                        assert!(!GraphError::is_error(response.status()));
                        let drive_item = response.body();
                        let drive_item_id =
                            drive_item["id"].as_str().unwrap_or_default().to_string();
                        tokio::time::delay_for(Duration::from_secs(3)).await;

                        let delete_res = client
                            .v1()
                            .users(id.as_str())
                            .drive()
                            .delete(drive_item_id.as_str())
                            .send()
                            .await;

                        if let Ok(response) = delete_res {
                            assert!(
                                response.status() == 200 ||
                                    response.status() == 201 ||
                                    response.status() == 204
                            );
                        } else if let Err(e) = delete_res {
                            panic!("Request error. Upload session new. Error: {:#?}", e);
                        }
                        break;
                    },
                    Err(e) => {
                        let _ = cancel_request.send().await.unwrap();
                        panic!("Request error. Upload session new. Error: {:#?}", e);
                    },
                }
            }
        }
    }
}

#[tokio::test]
async fn create_delete_folder_async() {
    let _lock = ASYNC_THROTTLE_MUTEX.lock().await;
    if let Some((id, token)) = OAuthRequest::request_access_token_async().await {
        let client = Graph::new_async(token.bearer_token());
        let folder: HashMap<String, serde_json::Value> = HashMap::new();
        let create_folder_res = client
            .v1()
            .drives(id.as_str())
            .drive()
            .create_folder(
                "",
                &serde_json::json!({
                    "name": "ci_docs_async",
                    "folder": folder,
                    "@microsoft.graph.conflictBehavior": "rename"
                }),
            )
            .send()
            .await;

        if let Ok(response) = create_folder_res {
            let item_id = response.body()["id"].as_str().unwrap();
            tokio::time::delay_for(Duration::from_secs(2)).await;

            let req = client.v1().drives(id).drive().delete(item_id).send().await;

            if let Ok(response) = req {
                assert!(
                    response.status() == 200 ||
                        response.status() == 201 ||
                        response.status() == 204
                );
            } else if let Err(e) = req {
                panic!("Request error. Method: drive delete. Error: {:#?}", e);
            }
        } else if let Err(e) = create_folder_res {
            panic!("Request error. Method: create folder. Error: {:#?}", e);
        }
    }
}

#[tokio::test]
async fn drive_upload_new_and_replace_and_delete() {
    let _lock = ASYNC_THROTTLE_MUTEX.lock().await;
    if let Some((id, token)) = OAuthRequest::request_access_token_async().await {
        let client = Graph::new_async(token.bearer_token());
        let upload_res = client
            .v1()
            .drives(id.as_str())
            .drive()
            .upload_new(
                ":/test_upload_file_async.txt:",
                "./test_files/test_upload_file_async.txt",
            )
            .send()
            .await;

        if let Ok(value) = upload_res {
            assert!(value.body()["id"].as_str().is_some());
            let item_id = value.body()["id"].as_str().unwrap();

            let mut file = OpenOptions::new()
                .write(true)
                .open("./test_files/test_upload_file_async.txt")
                .await
                .unwrap();

            file.write_all("Test Update File".as_bytes()).await.unwrap();
            file.sync_all().await.unwrap();

            tokio::time::delay_for(Duration::from_secs(2)).await;
            let upload_replace = client
                .v1()
                .drives(id.as_str())
                .drive()
                .upload_replace(item_id, "./test_files/test_upload_file_async.txt")
                .send()
                .await;

            if let Ok(value) = upload_replace {
                let item_id2 = value.body()["id"].as_str().unwrap();
                assert_eq!(item_id, item_id2);
            } else if let Err(e) = upload_replace {
                panic!(
                    "Request Error. Method: drive upload replace. Error: {:#?}",
                    e
                );
            }

            tokio::time::delay_for(Duration::from_secs(2)).await;
            let delete_res = client
                .v1()
                .drives(id.as_str())
                .drive()
                .delete(item_id)
                .send()
                .await;

            if let Ok(response) = delete_res {
                assert!(
                    response.status() == 200 ||
                        response.status() == 201 ||
                        response.status() == 204
                );
            } else if let Err(e) = delete_res {
                panic!("Request Error. Method: drive delete. Error: {:#?}", e);
            }
        } else if let Err(e) = upload_res {
            panic!("Request Error. Method: drive upload. Error: {:#?}", e);
        }
    }
}
