use {
    vulkano::memory::allocator::StandardMemoryAllocator,
    std::{
        path::Path,
        sync::{
            Arc,
            mpsc::{
                channel,
                Receiver,
                TryRecvError
                }
            },
        thread::spawn
        },
    crate::{
        error::{
            LoadingInProgressError,
            LoadModelError
            },
        model::Model
        }
    };



/** Manager responsible for the asynchronous loading and storage of 3D models. */
#[derive(Debug)]
pub struct ModelManager {
    /** Optional channel receiver used to poll the status of a background model loading thread. */
    receiver: Option<Receiver<Result<Model, LoadModelError>>>,
    /** The currently loaded and active 3D model, if any. */
    model: Option<Model>
    }

impl ModelManager {
    /** Initialises a new, empty model manager with no active model or loading process. */
    pub const fn new() -> Self {
        Self {
            receiver: None,
            model: None
            }
        }

    /** Initialises a new model manager and immediately begins loading a model from the specified path in a background thread. */
    pub fn from_path(model_path: Box<Path>, memory_allocator: Arc<StandardMemoryAllocator>) -> Self {
        Self {
            receiver: Some(load_with_thread(model_path, memory_allocator)),
            model: None
            }
        }

    /**
    Requests the asynchronous loading of a new model from the specified path.
    
    # Errors
    
    Returns a `LoadingInProgressError` if another model is already currently being loaded.
    */
    pub fn request_model_load(&mut self, model_path: Box<Path>, memory_allocator: Arc<StandardMemoryAllocator>) -> Result<(), LoadingInProgressError> {
        if self.is_loading() {
            return Err(LoadingInProgressError);
            }

        self.receiver = Some(load_with_thread(model_path, memory_allocator));

        Ok(())
        }

    /**
    Polls the background loading thread to check if the requested model has finished loading.
    
    This method should be called periodically to transition the model from loading to active.
    
    # Errors
    
    Returns a `LoadModelError` if the model failed to parse, or if the background thread disconnected unexpectedly.
    */
    pub fn update(&mut self) -> Result<(), LoadModelError> {
        let Some(ref result) = self.receiver else {
            return Ok(());
            };
        
        match result.try_recv() {
            Ok(Ok(model)) => {
                self.receiver = None;
                self.model = Some(model);
                },
            Ok(Err(error)) => {
                self.receiver = None;
                return Err(error);
                },
            Err(TryRecvError::Disconnected) => {
                self.receiver = None;
                return Err(LoadModelError::LoadingFailure);
                },
            Err(TryRecvError::Empty) => ()
            }

        Ok(())
        }

    /** Unloads the currently active model, freeing its associated resources. */
    pub fn unload_model(&mut self) {
        self.model = None;
        }

    /** Cancels any currently active model loading process by discarding the receiver. */
    pub fn cancel_model_load(&mut self) {
        self.receiver = None;
        }

    /** Checks whether a model is currently being loaded in the background. */
    pub const fn is_loading(&self) -> bool {
        self.receiver.is_some()
        }

    /** Retrieves a reference to the currently loaded model, if one is available. */
    pub const fn model(&self) -> Option<&Model> {
        self.model.as_ref()
        }
    }

impl Default for ModelManager {
    fn default() -> Self {
        Self::new()
        }
    }

/** Spawns a background thread to load a 3D model and returns a channel receiver to poll its completion status. */
fn load_with_thread(model_path: Box<Path>, memory_allocator: Arc<StandardMemoryAllocator>) -> Receiver<Result<Model, LoadModelError>> {
    let (sender, receiver) = channel();

    spawn(move || {
        let result = Model::from_path(&model_path, memory_allocator);
        
        _ = sender.send(result);
        });

    receiver
    }