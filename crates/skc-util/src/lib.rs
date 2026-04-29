async fn create_directory(directory_path: &Path) {
    ::std::fs::create_dir_all(directory_path)
}

async fn create_file(file_path: &Path) {}
