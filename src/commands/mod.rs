mod init;
pub use init::Init;
mod add;
pub use add::Add;
mod update;
pub use update::Update;
mod remove;
pub use remove::Remove;
mod build;
pub use build::Build;
mod run;
pub use run::Run;
mod new;
pub use new::New;

mod project_create {
    use std::path::Path;

    pub mod r#type {
        use std::path::Path;

        use super::{create_dir, create_file};

        pub fn bin(path: &Path) {
            create_dir(path, "src");
            create_file(path, "src/main.lua", "print(\"Hello, World!\")\n");
        }

        pub fn lib(path: &Path) {
            create_dir(path, "src");
            create_file(path, "src/lib.lua", "local exports = {}\n\nexports.add = function(a, b)\n    return a + b\nend\n\nreturn exports");
        }
    }

    pub fn create_file(path: &Path, name: &str, content: &str) {
        std::fs::write(path.join(name), content).unwrap();
    }

    pub fn create_dir(path: &Path, name: &str) {
        std::fs::create_dir(path.join(name)).unwrap();
    }
}
