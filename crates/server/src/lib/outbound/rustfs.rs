use std::io::Cursor;

use anyhow::ensure;
use async_trait::async_trait;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{config::Credentials, primitives::ByteStream, Client};
use image::{DynamicImage, ImageFormat, ImageReader};
use log::info;
use url::Url;

use crate::{
    config::S3Config,
    domain::event::{self, models::event::EventId},
};

const EVENT_IMAGE_BUCKET: &str = "event-image";
const USER_IMAGE_BUCKET: &str = "user-image";
const ARTIST_IMAGE_BUCKET: &str = "artist-image";

#[derive(Clone, Debug)]
pub struct RustFs {
    client: aws_sdk_s3::Client,
}

impl RustFs {
    pub async fn new(config: &S3Config) -> Self {
        info!("Initializing RustFs with endpoint: {:?}", config);
        let credentials =
            Credentials::new(&config.access_key, &config.secret_key, None, None, "rustfs");

        let shared_config = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new("us-east-1"))
            .credentials_provider(credentials)
            .endpoint_url(config.endpoint.as_str())
            .load()
            .await;

        let s3_config = aws_sdk_s3::config::Builder::from(&shared_config)
            .force_path_style(true)
            .build();

        let client = Client::from_conf(s3_config);

        let res = client
            .list_objects()
            .bucket(EVENT_IMAGE_BUCKET)
            .send()
            .await;
        info!("Head bucket response: {:?}", res);

        Self { client }
    }

    async fn upload_image(
        &self,
        bucket: &str,
        image_bytes: &[u8],
    ) -> Result<Url, Box<dyn std::error::Error + Send + Sync>> {
        let mut image = ImageReader::new(Cursor::new(image_bytes))
            .with_guessed_format()?
            .decode()?;
        // let orientation = decoder.orientation()?;
        // let mut image = DynamicImage::from_decoder(decoder)?;
        // image.apply_orientation(orientation);
        let mut image_bytes = Vec::new();
        image.write_to(&mut Cursor::new(&mut image_bytes), ImageFormat::Png)?;

        let key = format!("form/{}.png", uuid::Uuid::new_v4());

        let _ = self
            .client
            .put_object()
            .bucket(bucket)
            .key(&key)
            .body(ByteStream::from(image_bytes.to_vec()))
            .send()
            .await?;

        let url = format!("s3://{bucket}/{key}");
        Ok(Url::parse(&url)?)
    }

    async fn persist_image(
        &self,
        bucket: &str,
        new_key: &str,
        image_url: &Url,
    ) -> anyhow::Result<Url> {
        ensure!(image_url.scheme() == "s3", "Image URL must be an S3 URL");

        let source_key = format!("{bucket}/{}", image_url.path().trim_start_matches('/'));

        self.client
            .copy_object()
            .bucket(bucket)
            .key(new_key)
            .copy_source(source_key)
            .send()
            .await?;

        self.client
            .delete_object()
            .bucket(bucket)
            .key(image_url.path().trim_start_matches('/'))
            .send()
            .await?;

        let new_url = format!("s3://{bucket}/{new_key}");
        Ok(Url::parse(&new_url)?)
    }
}

#[async_trait]
impl event::ports::ImageRepository for RustFs {
    async fn upload_form_image(
        &self,
        image_bytes: &[u8],
    ) -> Result<Url, Box<dyn std::error::Error + Send + Sync>> {
        self.upload_image("event-image", image_bytes).await
    }

    async fn persist_form_image(
        &self,
        event_id: &EventId,
        image_url: &Url,
    ) -> Result<Url, Box<dyn std::error::Error + Send + Sync>> {
        // let segments =  image_url.h
        // let new_key = format!("events/{}.{}", event_id, image_url.path().trim
        Ok(self
            .persist_image(
                "event-image",
                &format!("events/{}.png", event_id),
                image_url,
            )
            .await?)
    }
}
