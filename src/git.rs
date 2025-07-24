use anyhow::{Context, Result};
use git2::{DiffOptions, Repository, Status, StatusOptions};
use std::path::Path;

pub struct GitRepo {
    repo: Repository,
}

impl GitRepo {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let repo = Repository::discover(path)
            .context("Failed to find Git repository. Make sure you're in a Git repository.")?;

        Ok(GitRepo { repo })
    }

    pub fn has_changes(&self, cached: bool) -> Result<bool> {
        if cached {
            self.has_staged_changes()
        } else {
            self.has_any_changes()
        }
    }

    pub fn has_staged_changes(&self) -> Result<bool> {
        let mut opts = StatusOptions::new();
        opts.include_ignored(false).include_untracked(false);

        let statuses = self
            .repo
            .statuses(Some(&mut opts))
            .context("Failed to get repository status")?;

        for entry in statuses.iter() {
            let status = entry.status();
            if status.intersects(
                Status::INDEX_NEW
                    | Status::INDEX_MODIFIED
                    | Status::INDEX_DELETED
                    | Status::INDEX_RENAMED
                    | Status::INDEX_TYPECHANGE,
            ) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn has_any_changes(&self) -> Result<bool> {
        let mut opts = StatusOptions::new();
        opts.include_ignored(false).include_untracked(true);

        let statuses = self
            .repo
            .statuses(Some(&mut opts))
            .context("Failed to get repository status")?;

        Ok(!statuses.is_empty())
    }

    pub fn get_diff(&self, cached: bool) -> Result<String> {
        let mut diff_opts = DiffOptions::new();
        diff_opts.context_lines(3);

        let diff = if cached {
            // Get staged changes (index vs HEAD)
            let head = self.repo.head()?.peel_to_tree()?;
            let index = self.repo.index()?;
            self.repo
                .diff_tree_to_index(Some(&head), Some(&index), Some(&mut diff_opts))?
        } else {
            // Get all changes (working directory vs HEAD)
            let head = self.repo.head()?.peel_to_tree()?;
            self.repo
                .diff_tree_to_workdir_with_index(Some(&head), Some(&mut diff_opts))?
        };

        let mut diff_text = String::new();
        diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
            match line.origin() {
                '+' | '-' | ' ' => {
                    diff_text.push(line.origin());
                    diff_text.push_str(std::str::from_utf8(line.content()).unwrap_or(""));
                }
                _ => {}
            }
            true
        })?;

        Ok(diff_text)
    }

    pub fn commit(&self, message: &str) -> Result<()> {
        let mut index = self.repo.index()?;
        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;

        let signature = self.repo.signature().context(
            "Failed to get Git signature. Make sure git user.name and user.email are configured.",
        )?;

        let parent_commit = match self.repo.head() {
            Ok(head) => Some(head.peel_to_commit()?),
            Err(_) => None, // Initial commit
        };

        let parents: Vec<&git2::Commit> = match &parent_commit {
            Some(commit) => vec![commit],
            None => vec![],
        };

        self.repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &parents,
        )?;

        Ok(())
    }

    /// Count the number of changed files
    pub fn count_changed_files(&self, cached: bool) -> Result<usize> {
        let mut opts = StatusOptions::new();
        opts.include_ignored(false);

        if cached {
            opts.include_untracked(false);
        } else {
            opts.include_untracked(true);
        }

        let statuses = self
            .repo
            .statuses(Some(&mut opts))
            .context("Failed to get repository status")?;

        let mut count = 0;
        for entry in statuses.iter() {
            let status = entry.status();

            if cached {
                // Count only staged files
                if status.intersects(
                    Status::INDEX_NEW
                        | Status::INDEX_MODIFIED
                        | Status::INDEX_DELETED
                        | Status::INDEX_RENAMED
                        | Status::INDEX_TYPECHANGE,
                ) {
                    count += 1;
                }
            } else {
                // Count all changed files (staged and unstaged)
                if !status.is_ignored() && status != Status::CURRENT {
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    /// Get status entries for display
    pub fn get_status_entries(&self) -> Result<Vec<StatusEntry>> {
        let mut opts = StatusOptions::new();
        opts.include_ignored(false);
        opts.include_untracked(true);

        let statuses = self
            .repo
            .statuses(Some(&mut opts))
            .context("Failed to get repository status")?;

        let mut entries = Vec::new();
        for entry in statuses.iter() {
            let path = entry.path().unwrap_or("unknown").to_string();
            let status = entry.status();

            entries.push(StatusEntry { path, status });
        }

        Ok(entries)
    }
}

/// Represents a git status entry
pub struct StatusEntry {
    pub path: String,
    pub status: Status,
}
