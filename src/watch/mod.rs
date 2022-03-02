//
// mod.rs
// motive
// 
// Author: wess (me@wess.io)
// Created: 02/16/2022
// 
// Copywrite (c) 2022 Wess.io
//

use std::{
  collections::HashMap,
  path::{
    Path,
    PathBuf
  },
  sync::{
    Arc,
    Mutex,
    mpsc::{
      channel,
      Receiver
    }
  }
};


use notify::Watcher;
pub use notify::{
  self,
  DebouncedEvent as Event
};

mod error;
use error::Error as WatchError;

use crate::am;

type Registry = HashMap<PathBuf, Box<dyn FnMut(Event) + Send>>;

pub struct Watch {
  watcher: notify::RecommendedWatcher,
  handlers: Arc<Mutex<Registry>>,
}

impl Watch {
  pub fn new() -> Result<Self, WatchError> {
    let (tx, rx) = channel();
    let handlers = Arc::<Mutex<_>>::default();
    let delay = std::time::Duration::from_millis(500);
    let watcher = notify::Watcher::new(tx, delay).unwrap();

    Self::run(Arc::clone(&handlers), rx);

    Ok(
      Self {
        watcher,
        handlers
      }
    )
  }

  pub fn watch<P, F>(&mut self, path: P, handler: F) -> Result<(), WatchError>
  where
      P: AsRef<Path>,
      F: 'static + FnMut(Event) + Send,
  {
      let absolute_path = path.as_ref().canonicalize()?;
      self.watcher
          .watch(&absolute_path, notify::RecursiveMode::Recursive)?;
      let mut handlers = self.handlers.lock().expect("handler mutex poisoned!");
      handlers.insert(absolute_path, Box::new(handler));
      Ok(())
  }

  pub fn unwatch<P: AsRef<Path>>(&mut self, path: P) -> Result<(), WatchError> {
    let absolute_path = path.as_ref().canonicalize()?;
  
    self.watcher.unwatch(&absolute_path)?;
  
    let mut handlers = self.handlers.lock().expect("handler mutex poisoned!");
    handlers.remove(&absolute_path);
  
    Ok(())
  } 

  fn run(handlers: Arc<Mutex<Registry>>, rx: Receiver<Event>) {
    tokio::spawn(async move {
      match rx.recv() {
        Ok(event) => {
          console_info!("{:?}", event);

          let mut handlers = handlers.lock().unwrap();

          if let Some(handler) = handler_for_event(&event, &mut handlers) {
            handler(event);
          }
        },
        Err(e) => {
          console_panic!("{:?}", e);
        }
      }
    });
  }
}

impl std::fmt::Debug for Watch {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    fmt.debug_struct("Watch").finish()
  }
}

fn handler_for_event<'a, H>(e: &Event, handlers: &'a mut HashMap<PathBuf, H>) -> Option<&'a mut H> {
  fn path_from_event(e: &Event) -> Option<&Path> {
      match e {
          Event::NoticeWrite(p)
          | Event::NoticeRemove(p)
          | Event::Create(p)
          | Event::Write(p)
          | Event::Chmod(p)
          | Event::Remove(p)
          | Event::Rename(p, _) => Some(p.as_path()),
          _ => None,
      }
  }

  fn find_handler<'a, H>(path: &Path, handlers: &'a mut HashMap<PathBuf, H>) -> Option<&'a mut H> {
      let mut remaining_path = Some(path);

      while let Some(path) = remaining_path {
          console_debug!("matching against {:?}", path);

          if handlers.contains_key(path) {
              return handlers.get_mut(path);
          }

          remaining_path = path.parent();
      }

      None
  }

  path_from_event(e).and_then(move |path| find_handler(path, handlers))
}