use crate::identity::{AuthorizationSerializer, ClientSecretCredential};
use crate::oauth::{
    ConfidentialClientApplication, PublicClientApplication, ResourceOwnerPasswordCredential,
};
use std::env::VarError;

const AZURE_TENANT_ID: &str = "AZURE_TENANT_ID";
const AZURE_CLIENT_ID: &str = "AZURE_CLIENT_ID";
const AZURE_CLIENT_SECRET: &str = "AZURE_CLIENT_SECRET";
const AZURE_USERNAME: &str = "AZURE_USERNAME";
const AZURE_PASSWORD: &str = "AZURE_PASSWORD";

pub struct EnvironmentCredential {
    pub credential: Box<dyn AuthorizationSerializer + Send>,
}

impl EnvironmentCredential {
    pub fn resource_owner_password_credential() -> Result<PublicClientApplication, VarError> {
        match EnvironmentCredential::try_username_password_compile_time_env() {
            Ok(credential) => Ok(credential),
            Err(_) => EnvironmentCredential::try_username_password_runtime_env(),
        }
    }

    pub fn client_secret_credential() -> Result<ConfidentialClientApplication, VarError> {
        match EnvironmentCredential::try_azure_client_secret_compile_time_env() {
            Ok(credential) => Ok(credential),
            Err(_) => EnvironmentCredential::try_azure_client_secret_runtime_env(),
        }
    }

    fn try_azure_client_secret_compile_time_env() -> Result<ConfidentialClientApplication, VarError>
    {
        let tenant_id_option = option_env!("AZURE_TENANT_ID");
        let azure_client_id = option_env!("AZURE_CLIENT_ID").ok_or(VarError::NotPresent)?;
        let azure_client_secret = option_env!("AZURE_CLIENT_SECRET").ok_or(VarError::NotPresent)?;

        match tenant_id_option {
            Some(tenant_id) => Ok(ConfidentialClientApplication::new(
                ClientSecretCredential::new_with_tenant(
                    tenant_id,
                    azure_client_id,
                    azure_client_secret,
                ),
                Default::default(),
            )
            .map_err(|_| VarError::NotPresent)?),
            None => Ok(ConfidentialClientApplication::new(
                ClientSecretCredential::new(azure_client_id, azure_client_secret),
                Default::default(),
            )
            .map_err(|_| VarError::NotPresent)?),
        }
    }

    fn try_azure_client_secret_runtime_env() -> Result<ConfidentialClientApplication, VarError> {
        let tenant_id_result = std::env::var(AZURE_TENANT_ID);
        let azure_client_id = std::env::var(AZURE_CLIENT_ID)?;
        let azure_client_secret = std::env::var(AZURE_CLIENT_SECRET)?;

        if let Ok(tenant_id) = tenant_id_result {
            Ok(ConfidentialClientApplication::new(
                ClientSecretCredential::new_with_tenant(
                    tenant_id,
                    azure_client_id,
                    azure_client_secret,
                ),
                Default::default(),
            )
            .map_err(|_| VarError::NotPresent)?)
        } else {
            Ok(ConfidentialClientApplication::new(
                ClientSecretCredential::new(azure_client_id, azure_client_secret),
                Default::default(),
            )
            .map_err(|_| VarError::NotPresent)?)
        }
    }

    fn try_username_password_compile_time_env() -> Result<PublicClientApplication, VarError> {
        let tenant_id_option = option_env!("AZURE_TENANT_ID");
        let azure_client_id = option_env!("AZURE_CLIENT_ID").ok_or(VarError::NotPresent)?;
        let azure_username = option_env!("AZURE_USERNAME").ok_or(VarError::NotPresent)?;
        let azure_password = option_env!("AZURE_PASSWORD").ok_or(VarError::NotPresent)?;

        match tenant_id_option {
            Some(tenant_id) => Ok(PublicClientApplication::new(
                ResourceOwnerPasswordCredential::new_with_tenant(
                    tenant_id,
                    azure_client_id,
                    azure_username,
                    azure_password,
                ),
                Default::default(),
            )
            .map_err(|_| VarError::NotPresent)?),
            None => Ok(PublicClientApplication::new(
                ResourceOwnerPasswordCredential::new(
                    azure_client_id,
                    azure_username,
                    azure_password,
                ),
                Default::default(),
            )
            .map_err(|_| VarError::NotPresent)?),
        }
    }

    fn try_username_password_runtime_env() -> Result<PublicClientApplication, VarError> {
        let tenant_id_result = std::env::var(AZURE_TENANT_ID);
        let azure_client_id = std::env::var(AZURE_CLIENT_ID)?;
        let azure_username = std::env::var(AZURE_USERNAME)?;
        let azure_password = std::env::var(AZURE_PASSWORD)?;

        match tenant_id_result {
            Ok(tenant_id) => Ok(PublicClientApplication::new(
                ResourceOwnerPasswordCredential::new_with_tenant(
                    tenant_id,
                    azure_client_id,
                    azure_username,
                    azure_password,
                ),
                Default::default(),
            )
            .map_err(|_| VarError::NotPresent)?),
            Err(_) => Ok(PublicClientApplication::new(
                ResourceOwnerPasswordCredential::new(
                    azure_client_id,
                    azure_username,
                    azure_password,
                ),
                Default::default(),
            )
            .map_err(|_| VarError::NotPresent)?),
        }
    }
}

impl From<ClientSecretCredential> for EnvironmentCredential {
    fn from(value: ClientSecretCredential) -> Self {
        EnvironmentCredential {
            credential: Box::new(value),
        }
    }
}

impl From<ResourceOwnerPasswordCredential> for EnvironmentCredential {
    fn from(value: ResourceOwnerPasswordCredential) -> Self {
        EnvironmentCredential {
            credential: Box::new(value),
        }
    }
}

impl From<ConfidentialClientApplication> for EnvironmentCredential {
    fn from(value: ConfidentialClientApplication) -> Self {
        EnvironmentCredential {
            credential: Box::new(value),
        }
    }
}

impl From<PublicClientApplication> for EnvironmentCredential {
    fn from(value: PublicClientApplication) -> Self {
        EnvironmentCredential {
            credential: Box::new(value),
        }
    }
}
