use k8s_openapi::serde::__private::fmt::Debug;
use k8s_openapi::serde::de::DeserializeOwned;
use kube::api::ListParams;
use kube::{Api, Client, Resource};

use crate::Error;

pub async fn list_resource<T>(client: &Client, namespace: &str) -> Result<Vec<T>, Error>
where
    T: Clone + Debug + DeserializeOwned + Resource,
    <T as Resource>::DynamicType: Default,
{
    let resource_api = Api::<T>::namespaced(client.clone(), namespace);
    Ok(resource_api.list(&ListParams::default()).await?.items)
}

pub async fn list_resource_with_name<T>(client: &Client, namespace: &str, name: &str) -> Result<Vec<T>, Error>
where
    T: Clone + Debug + DeserializeOwned + Resource,
    <T as Resource>::DynamicType: Default,
{
    let resource_api = Api::<T>::namespaced(client.clone(), namespace);
    let lp = ListParams::default()
        .fields(&("metadata.name=".to_owned() + name));
    Ok(resource_api.list(&lp).await?.items)
}
