use super::*;

impl ImageBrowser {
    pub fn update(&mut self, event: &namui::Event) {
        event.is::<Event>(|event| {
            match event {
                Event::PlusButtonClicked => {
                    let storage = self.storage.clone();
                    spawn_local(async move {
                        let files = namui::file::dialog::open().await;
                        for file in files.iter() {
                            let data = file.content().await;
                            let result = storage.upload_resource(file.name(), &data).await;
                            if let Err(error) = result {
                                namui::log!("error: {:?}", error);
                            }
                        }
                        namui::event::send(Event::RequestListRefresh);
                    });
                }
                Event::RequestListRefresh => {
                    let storage = self.storage.clone();
                    spawn_local(async move {
                        let result = storage.list_resources().await;
                        match result {
                            Ok(resources) => {
                                namui::event::send(Event::ListRefreshed { resources });
                            }
                            Err(error) => {
                                namui::log!("error: {:?}", error);
                                // TODO
                            }
                        }
                    });
                }
                Event::ListRefreshed { resources } => {
                    self.resources = resources.clone();
                }
            }
        }
    }
}
