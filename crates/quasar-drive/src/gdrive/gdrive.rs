use tokio;
use google_drive3::api::File;
use google_drive3::{Result, Error};
use google_drive3::{
    DriveHub, FieldMask, hyper, hyper_rustls, hyper_util,
    yup_oauth2 as oauth2
};
use std::collections::HashMap;


type GDriveMetadata = HashMap<String, String>;
type GDrivePermissions = Vec<HashMap<String, String>>;


struct GDrive {
    credentials: oauth2::ServiceAccountKey,
    drive_hub: DriveHub,
    ud_scopes: Vec<&str>
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

        let scopes = vec![
            "https://www.googleapis.com/auth/drive.file",
            "https://www.googleapis.com/auth/drive",
            "https://www.googleapis.com/auth/drive.metadata"
        ];
        let auth = oauth2::ServiceAccountAuthenticator::builder(
                secret
        )
        .build()
        .await
        .expect("Can not build the authenticator using secret");
        let drive_hub = DriveHub::new(client, auth);

        Self {
            credentials: path,
            drive_hub: drive_hub,
            ud_scopes: scopes
        }
    }

    pub fn query_folder(self, folder_id: Optional<String>) -> GDriveFile {
        // todo!("I am yet to implement this because of the relation b/n drive and file");
        let folder_mime_type = "application/vnd.google-apps.document";
        let query = format!("mimeType='{}'", folder_mime_type)
        self.drive_hub.folder
    }
}



impl GDriveFile {
    pub fn file_path(drive: GDrive, folder_id: String) -> Vec<String> {
        todo!("Implement the file reconstruction path");
    }
}
