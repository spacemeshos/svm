#![warn(unused)]

use crate::storage::{Fingerprint, Storage};
use crate::StorageResult as Result;
use svm_types::{Address, BytesPrimitive, Layer, TemplateAddr};

pub struct GlobalState {
    storage: Storage,
}

impl GlobalState {
    pub async fn new(sqlite_uri: &str) -> Self {
        Self {
            storage: Storage::new(sqlite_uri).await.unwrap(),
        }
    }

    pub async fn in_memory() -> Self {
        Self {
            storage: Storage::in_memory().await.unwrap(),
        }
    }

    pub async fn account_balance(&self, account_addr: &Address) -> Result<Option<u64>> {
        let key = format!("accounts:{}:balance", account_addr.to_string());
        let opt_value = self.storage.get(key.as_bytes(), None).await?;

        if let Some(bytes) = opt_value {
            let s = std::str::from_utf8(&bytes[..]).expect("Invalid UTF-8 value.");
            let balance = str::parse(s).expect("Invalid numeric value.");
            Ok(Some(balance))
        } else {
            Ok(None)
        }
    }

    pub async fn account_nonce(&self, account_addr: &Address) -> Result<Option<u64>> {
        let key = format!("accounts:{}:nonce", account_addr.to_string());
        let opt_value = self.storage.get(key.as_bytes(), None).await?;

        if let Some(bytes) = opt_value {
            let s = std::str::from_utf8(&bytes[..]).expect("Invalid UTF-8 value.");
            let balance = str::parse(s).expect("Invalid numeric value.");
            Ok(Some(balance))
        } else {
            Ok(None)
        }
    }

    pub async fn template_address(&self, account_addr: &Address) -> Result<Option<TemplateAddr>> {
        let key = format!("accounts:{}:template_address", account_addr.to_string());
        let opt_value = self.storage.get(key.as_bytes(), None).await?;

        Ok(opt_value.map(|x| TemplateAddr::new(&x[..])))
    }

    pub async fn template_mandatory_sections(
        &self,
        account_address: &Address,
    ) -> Result<Option<TemplateAddr>> {
        let key = format!(
            "templates:{}:mandatory_sections",
            account_address.to_string()
        );
        let opt_value = self.storage.get(key.as_bytes(), None).await?;

        Ok(opt_value.map(|x| TemplateAddr::new(&x[..])))
    }

    pub async fn template_optional_sections(
        &self,
        account_address: &Address,
    ) -> Result<Option<TemplateAddr>> {
        let key = format!(
            "templates:{}:optional_sections",
            account_address.to_string()
        );
        let opt_value = self.storage.get(key.as_bytes(), None).await?;

        Ok(opt_value.map(|x| TemplateAddr::new(&x[..])))
    }

    pub async fn set_template_optional_sections<V>(
        &mut self,
        template_addr: &TemplateAddr,
        value: V,
    ) -> Result<()>
    where
        V: Into<Vec<u8>>,
    {
        let key = format!("templates:{}:optional_sections", template_addr.to_string());
        self.storage.upsert(key.as_bytes(), value.into()).await;
        Ok(())
    }

    pub async fn set_template_mandatory_sections<V>(
        &mut self,
        template_addr: &TemplateAddr,
        value: V,
    ) -> Result<()>
    where
        V: Into<Vec<u8>>,
    {
        let key = format!("templates:{}:mandatory_sections", template_addr.to_string());
        self.storage.upsert(key.as_bytes(), value.into()).await;
        Ok(())
    }

    pub async fn set_account_balance(
        &mut self,
        account_addr: &Address,
        balance: u64,
    ) -> Result<()> {
        let key = format!("accounts:{}:balance", account_addr.to_string());
        self.storage
            .upsert(key.as_bytes(), balance.to_string().as_bytes())
            .await;
        Ok(())
    }

    pub async fn set_account_nonce(&mut self, account_addr: &Address, nonce: u64) -> Result<()> {
        let key = format!("accounts:{}:nonce", account_addr.to_string());
        self.storage
            .upsert(key.as_bytes(), nonce.to_string().as_bytes())
            .await;
        Ok(())
    }

    pub async fn checkpoint(&mut self) -> Result<()> {
        self.storage.checkpoint().await?;
        Ok(())
    }

    pub async fn commit(&mut self) -> Result<(Layer, Fingerprint)> {
        let res = self.storage.commit().await?;
        Ok(res)
    }

    pub async fn current_layer(&mut self) -> Result<(Layer, Fingerprint)> {
        let res = self.storage.last_layer().await?;
        Ok(res)
    }

    pub async fn rollback(&mut self) -> Result<()> {
        self.storage.rollback().await?;
        Ok(())
    }

    pub async fn rewind(&mut self, layer_id: Layer) -> Result<()> {
        self.storage.rewind(layer_id).await?;
        Ok(())
    }
}
