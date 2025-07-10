use tokio;
use google_drive3::api::File;
use google_drive3::{Result, Error};
use google_drive3::{
    DriveHub, FieldMask, hyper, hyper_rustls, hyper_util,
    yup_oauth2 as oauth2
};
use crate::hyper_util::client::legacy::connect::HttpConnector;

use std::{
    collections::HashMap,
    pin::Pin,
    future::Future,
    result::Result
};



// let scopes = vec![
//     "https://www.googleapis.com/auth/drive/file",
//     "https://www.googleapis.com/auth/drive",
//     "https://www.googleapis.com/drive.metadata"
// ];
//

type BoxFutureResult<'a, T> = Pin<Box<dyn Future<Output = Result<T, anyhow::Error> + Send + 'a>>>;


type GDriveMetadata = HashMap<String, String>;
type GDrivePermissions = Vec<HashMap<String, String>>;


struct GDrive {
    credentials: oauth2::ServiceAccountKey,
    drive_hub: DriveHub,
}


struct GDriveFile {
    drive_name: String,
    file_path: String,
    metadata: GDriveMetadata,
    permissions: GDrivePermission,
    md5_hash: String
}


impl GDrive {
    pub async fn new() -> Self {
        let path = std::env::var("GOOGLE_DRIVE_APPLICATIONS_CREDENTIALS")
            .expect("Env variable of that name does not exists");
        let user_added_path: std::path::Path = std::path::Path::new(
            &path
        );

        let service_account: oauth2::ServiceAccountKey = oauth2::read_service_account_key(
            path
        )
        .await
        .expect("Drive Credentials not added...");

         let https: hyper_rustls::HttpsConnector<_> = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .expect("failed to find native root certificates")
            .https_only()
            .enable_http1()
            .enable_http2()
            .build();

         let client = hyper_util::client::legacy::Client::builder(
             hyper_util::rt::TokioExecutor::new()
        ).build(https);

       let auth = oauth2::ServiceAccountAuthenticator::builder(
                secret
        )
        .build()
        .await
        .expect("Can not build the authenticator using secret");
        let drive_hub = DriveHub::new(client, auth);

        Self {
            credentials: path,
            drive_hub: drive_hub
        }
    }


    pub async fn query(self, query: &str) -> Vec<File> {
        // Takes care of querying files given a query parameter
        let scopes: Vec<&str> = vec![
            "https://www.googleapis.com/auth/drive.file",
            "https://www.googleapis.com/auth/drive",
            "https://www.googleapis.com/auth/drive.metadata"
        ];
        let mut files: Vec<File> = vec![];
        let mut page_token: Option<String> = None;
        loop {
            let mut req = self.drive_hub.files().list()
                .add_scopes(&scopes)
                .param(
                    "fields",
                    "nextPageToken,files(id,name,permissions,parents,mimeType,createdTime,modifiedTime)"
                )
                .q("mimeType!='application/vnd.google-apps.folder'")
                .page_size(100);

            if let Some(ref token) = page_token {
                req = req.page_token(token)
            }

            let (_, file_list) = req
                .doit()
                .await
                .expect("Fecthing drive response failed!");

            if let Some(file_list_content) = file_list.files {
                files.extend_from_slice(file_list_content.as_slice());
            }

            match file_list.next_page_token {
                Some(token) => {
                    page_token = Some(token)
                }
                None => break,
            }
        }

        files
    }

}


