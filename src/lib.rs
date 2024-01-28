// Copyright (c) 2023 Murilo Ijanc' <mbsd@m0x.ru>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use std::path::Path;

/// A project allows you to group together a set of environments with the
/// objective to run the same application.
pub struct Project {
    name: String,
    envs: Option<Vec<Environment>>,
}

impl Project {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into(), envs: None }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn envs(&self) -> Option<&Vec<Environment>> {
        self.envs.as_ref()
    }

    pub fn has_env(&self) -> bool {
        self.envs.is_some()
    }

    pub fn push_env(&mut self, e: Environment) {
        if let Some(ref mut envs) = self.envs {
            envs.push(e);
        } else {
            self.envs = Some(vec![e]);
        }
    }

    pub fn count_envs(&self) -> usize {
        if self.has_env() {
            self.envs().unwrap().len()
        } else {
            0
        }
    }
}

pub struct Environment {
    name: String,
}

impl Environment {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

pub struct Application {
    name: String,
}

impl Application {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn from_path(p: &Path) -> Self {
        let name = get_name_on_path(p);
        Self::new(name)
    }
}

fn get_name_on_path(p: &Path) -> &str {
    p.file_name().unwrap().to_str().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn application_from_path() {
        let name = "python-flask-docker";
        let path = Path::new("../../").join(name);
        let a = Application::from_path(&path);
        assert_eq!(a.name(), name);
    }

    #[test]
    fn environment_new() {
        let name = "prod";
        let e = Environment::new(name);
        assert_eq!(e.name(), name);
    }

    #[test]
    fn project_new() {
        let name = "myproject";
        let p = Project::new(name);
        assert_eq!(p.name(), name);
    }

    #[test]
    fn project_new_environment() {
        let p_name = "myproject";
        let e_name = "prod";
        let e = Environment::new(e_name);
        assert_eq!(e.name(), e_name);

        let mut p = Project::new(p_name);
        assert!(!p.has_env());
        assert_eq!(p.count_envs(), 0);

        p.push_env(e);
        assert!(p.has_env());
        assert_eq!(p.count_envs(), 1);
    }
}
