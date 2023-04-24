use anyhow::{Context, Ok, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The workspace struct.
#[derive(Serialize, Deserialize)]
struct Workspace {
    name: String,
    path: PathBuf,
    identity: String,
}

/// The identity struct.
#[derive(Serialize, Deserialize, PartialEq)]
struct Identity {
    author: String,
    email: String,
    id: String,
}

/// The data structure of the config file.
#[derive(Serialize, Deserialize)]
struct Data {
    current_identity: Option<String>,
    workspaces: Vec<Workspace>,
    identities: Vec<Identity>,
}

/// The main struct of the program.
pub struct Gis {
    data: Data,
    config: PathBuf,
    pwd: PathBuf,
}
/// The default config.
fn default_config() -> Data {
    Data {
        current_identity: None,
        workspaces: vec![],
        identities: vec![],
    }
}

impl Gis {
    /// Add a new identity to the config.
    ///* The value must be in the format : "Author Name email@domain"
    pub fn add_identity(&mut self, value: &str) {
        if !value.contains("@") {
            println!("Invalid email or no email provided");
            return;
        }
        let mut parts = value.split_whitespace().collect::<Vec<&str>>();

        let email = parts
            .iter()
            .find(|&x| x.contains("@"))
            .expect("No email found")
            .to_string();

        parts.retain(|&x| !x.contains("@"));
        if parts.len() == 0 {
            println!("No author name provided");
            return;
        }
        let author = parts.join(" ");

        let id = format!("{:x}", md5::compute(format!("{} {}", author, email)));

        let identity = Identity { author, email, id };

        if self.data.identities.contains(&identity) {
            println!("Identity already exists");
            return;
        }
        if self.data.current_identity.is_none() {
            println!("No current identity set, setting this one as current");
            self.data.current_identity = Some(identity.id.clone());
        }
        self.data.identities.push(identity);
        self.save().expect("Failed to save config");
        println!("Added the following identity: {}", value)
    }
    /// Remove an identity from the config.
    pub fn remove_identity(&mut self, idx: usize) {
        if idx == 0 || idx > self.data.identities.len() {
            println!("Not a valid identity.");
            return;
        }
        let identity = self.data.identities.swap_remove(idx - 1);
        self.save().expect("Failed to save config");
        println!(
            "The following identity was removed :\nAuthor : {},\nEmail : {}",
            identity.author, identity.email
        );
    }
    /// Swap the current identity.
    pub fn swap_identity(&mut self, idx: usize) {
        if idx == 0 || idx > self.data.identities.len() {
            println!("Not a valid identity.");
            return;
        }
        let identity = self.data.identities.get(idx - 1).expect("Invalid index");
        let name_cmd = format!("git config --global user.name \"{}\"", identity.author);
        let email_cmd = format!("git config --global user.email \"{}\"", identity.email);

        let name_cmd = std::process::Command::new("sh")
            .arg("-c")
            .arg(name_cmd)
            .output()
            .expect("Failed to execute command");
        let email_cmd = std::process::Command::new("sh")
            .arg("-c")
            .arg(email_cmd)
            .output()
            .expect("Failed to execute command");

        if !name_cmd.status.success() || !email_cmd.status.success() {
            println!("Failed to set git config");
            return;
        }
        self.data.current_identity = Some(identity.id.clone());
        self.save().expect("Failed to save config");
        println!(
            "Swapped identity to : \nAuthor : {},\nEmail : {}",
            identity.author, identity.email
        );
    }
    /// List all identities.
    pub fn list_identities(&self) {
        println!("Identities :");
        for (idx, identity) in self.data.identities.iter().enumerate() {
            println!(
                "{}. Author : {}, Email : {}",
                idx + 1,
                identity.author,
                identity.email
            );
        }
    }
    /// Save the config.
    pub fn save(&self) -> Result<()> {
        if let Some(g) = self.config.parent() {
            if !std::fs::metadata(&g).is_ok() {
                std::fs::create_dir_all(g)?;
            }
        }
        let contents = serde_json::to_string(&self.data)?;
        std::fs::write(&self.config, contents)?;
        Ok(())
    }
    /// Add a new workspace to the config.
    pub fn add_workspace(&mut self, name: &str) {
        if let Some(identity) = &self.data.current_identity {
            let identity = self
                .data
                .identities
                .iter()
                .find(|&x| x.id == *identity)
                .expect("No identity found");
            println!(
                "This identity will be asigned to this workspace :\n\nAuthor : {} , Email : {}",
                identity.author, identity.email
            );
        } else {
            println!("There is no current identity set. Please add one and set one using the swap command.");
            return;
        }

        if name.is_empty() {
            println!("No workspace name provided");
            return;
        }
        let path = self.pwd.clone();
        let path = path.to_str().unwrap_or("").to_string();
        let path = PathBuf::from(path);
        println!("Added workspace {} : {}", name, path.display());
        let workspace = Workspace {
            name: name.to_string(),
            path,
            identity: self.data.current_identity.clone().unwrap_or_default(),
        };
        self.data.workspaces.push(workspace);
        self.save().expect("Failed to save config");
    }
    /// Remove a workspace from the config.
    pub fn remove_workspace(&mut self, idx: usize) {
        if idx == 0 || idx > self.data.workspaces.len() {
            println!("Not a valid workspace");
            return;
        }
        let workspace = self.data.workspaces.swap_remove(idx - 1);
        self.save().expect("Failed to save config");
        println!(
            "The following workspace was removed : \nDirectory : {}",
            workspace.path.display()
        )
    }
    /// List all workspaces.
    pub fn list_workspaces(&self) {
        println!("Workspaces :");
        for (idx, workspace) in self.data.workspaces.iter().enumerate() {
            let identity = self
                .data
                .identities
                .iter()
                .find(|&x| x.id == workspace.identity)
                .expect("No identity found");
            println!(
                "{}. {} at {} > Author : {} , Email : {}",
                idx + 1,
                workspace.name,
                workspace.path.display(),
                identity.author,
                identity.email
            );
        }
    }
    /// Check if the config has a current identity and at least one workspace.
    pub fn has_identity_and_workspace(&self) -> bool {
        self.data.current_identity.is_some() && self.data.workspaces.len() > 0
    }
    /// Check current identity.
    pub fn current_identity(&self) {
        if let Some(identity) = &self.data.current_identity {
            let identity = self
                .data
                .identities
                .iter()
                .find(|&x| x.id == *identity)
                .expect("No identity found");
            println!(
                "Current identity : \nAuthor : {},\nEmail : {}",
                identity.author, identity.email
            );
        } else {
            println!("No current identity set");
        }
    }
    /// Swap identity based on the current workspace.
    pub fn workspace_identity_swap(&mut self) {
        let current_dir = std::env::current_dir().context("Error getting current_dir");
        let current_dir = current_dir.unwrap();

        let workspace = self.data.workspaces.iter().find(|&x| x.path == current_dir);

        if let Some(workspace) = workspace {
            let identity = self
                .data
                .identities
                .iter()
                .find(|&x| x.id == workspace.identity);

            if let Some(identity) = identity {
                if identity.id == self.data.current_identity.clone().unwrap_or_default() {
                    return;
                }
                let idx = self
                    .data
                    .identities
                    .iter()
                    .position(|x| x.id == identity.id)
                    .expect("Failed to find identity");

                self.swap_identity(idx + 1);
            }
        }
    }
    /// Load the config from a file.
    pub fn from_config(config: PathBuf, pwd: PathBuf) -> Self {
        if std::fs::metadata(&config).is_ok() {
            let contents = std::fs::read_to_string(&config);
            let contents = contents.unwrap_or("{\"workspaces\": [], \"identities\": []}".into());
            let data = serde_json::from_str(&contents);
            let data = data.unwrap_or(default_config());
            return Gis { data, config, pwd };
        }
        return Gis {
            data: default_config(),
            config,
            pwd,
        };
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn get_data() -> Data {
        Data {
            current_identity: Some("d4eddef79dcda8098582c3aecb425e97".into()),
            workspaces: vec![Workspace {
                name: "test".into(),
                path: PathBuf::from("/tmp/test"),
                identity: "d4eddef79dcda8098582c3aecb425e97".into(),
            }],
            identities: vec![Identity {
                author: "Some Author".into(),
                email: "some@email.com".into(),
                id: "d4eddef79dcda8098582c3aecb425e97".into(),
            }],
        }
    }

    fn get_gis(pwd: PathBuf) -> Gis {
        // delete the config file if it exists
        std::fs::remove_file(".gisrc").ok();
        let mut current_working_dir = std::env::current_dir().unwrap();
        current_working_dir.push(".gisrc");
        Gis {
            data: get_data(),
            config: current_working_dir,
            pwd,
        }
    }

    #[test]
    fn test_add_identity() {
        let mut gis = get_gis(PathBuf::from("/tmp/test"));

        gis.add_identity("Another Author someother@email.com");
        let id = md5::compute(format!("{} {}", "Another Author", "someother@email.com"));
        assert_eq!(gis.data.identities.len(), 2);
        assert_eq!(gis.data.identities[1].author, "Another Author");
        assert_eq!(gis.data.identities[1].email, "someother@email.com");
        assert_eq!(gis.data.identities[1].id, format!("{:x}", id));
    }

    #[test]
    fn test_add_identity_no_email() {
        let mut gis = get_gis(PathBuf::from("/tmp/test"));

        gis.add_identity("Some Author");

        assert_eq!(gis.data.identities.len(), 1);
    }

    #[test]
    fn test_add_identity_no_author() {
        let mut gis = get_gis(PathBuf::from("/tmp/test"));

        gis.add_identity("some@email.com");

        assert_eq!(gis.data.identities.len(), 1);
    }

    #[test]
    fn test_add_identity_no_author_no_email() {
        let mut gis = get_gis(PathBuf::from("/tmp/test"));

        gis.add_identity("");

        assert_eq!(gis.data.identities.len(), 1);
    }

    #[test]
    fn test_remove_identity() {
        let mut gis = get_gis(PathBuf::from("/tmp/test"));

        gis.remove_identity(1);

        assert_eq!(gis.data.identities.len(), 0);
    }

    #[test]
    fn test_add_workspace() {
        let mut gis = get_gis(PathBuf::from("/tmp/test"));
        gis.add_workspace("test");

        assert_eq!(gis.data.workspaces.len(), 2);
        assert_eq!(gis.data.workspaces[1].name, "test");
        assert_eq!(gis.data.workspaces[1].path, PathBuf::from("/tmp/test"));
    }

    #[test]
    fn test_add_workspace_no_name() {
        let mut gis = get_gis(PathBuf::from("/tmp/test"));
        gis.add_workspace("");

        assert_eq!(gis.data.workspaces.len(), 1);
    }

    #[test]
    fn test_remove_workspace() {
        let mut gis = get_gis(PathBuf::from("/tmp/test"));
        gis.remove_workspace(1);

        assert_eq!(gis.data.workspaces.len(), 0);
    }

    #[test]
    fn test_swap_identity() {
        let mut gis = get_gis(PathBuf::from("/tmp/test"));
        gis.swap_identity(1);

        assert_eq!(
            gis.data.current_identity,
            Some("d4eddef79dcda8098582c3aecb425e97".into())
        );
    }
}
