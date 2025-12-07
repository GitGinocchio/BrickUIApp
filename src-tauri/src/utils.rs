
pub async fn spawn_blocking<T, E, F>(f: F) -> Result<T, E>
where
    F: FnOnce() -> Result<T, E> + Send + 'static,
    T: Send + 'static,
    E: Send + 'static,
{
    tauri::async_runtime::spawn_blocking(f)
        .await
        .map_err(|e| panic!("Join failure: {e}"))?
}
