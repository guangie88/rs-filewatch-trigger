use notify::{PollWatcher, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::path::Path;

pub trait WeakWatcher {
    fn watch(
        &mut self,
        path: &Path,
        recursive_mode: RecursiveMode,
    ) -> Result<()>;
}

impl WeakWatcher for RecommendedWatcher {
    fn watch(
        &mut self,
        path: &Path,
        recursive_mode: RecursiveMode,
    ) -> Result<()> {
        Watcher::watch(self, path, recursive_mode)
    }
}

impl WeakWatcher for PollWatcher {
    fn watch(
        &mut self,
        path: &Path,
        recursive_mode: RecursiveMode,
    ) -> Result<()> {
        Watcher::watch(self, path, recursive_mode)
    }
}
