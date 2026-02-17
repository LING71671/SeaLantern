use crate::services::java_detector;

#[tauri::command]
pub async fn detect_java() -> Result<Vec<java_detector::JavaInfo>, String> {
    tauri::async_runtime::spawn_blocking(java_detector::detect_java_installations)
        .await
        .map_err(|e| format!("Java 检测任务失败: {}", e))
}

#[tauri::command]
pub async fn validate_java_path(path: String) -> Result<java_detector::JavaInfo, String> {
    tauri::async_runtime::spawn_blocking(move || java_detector::validate_java(path.as_str()))
        .await
        .map_err(|e| format!("Java 路径验证任务失败: {}", e))?
}

#[tauri::command]
pub async fn install_java<R: tauri::Runtime>(
    _app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
    url: String,
    version_name: String,
) -> Result<String, String> {
    use crate::services::java_installer;
    java_installer::download_and_install_java(url, version_name, window).await
}
