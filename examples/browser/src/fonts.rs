use iced::Task;
use iced::font::Family;

#[derive(Debug, Clone)]
pub enum Message {
    CatalogLoaded(Result<fount::Catalog, fount::Error>),
    SystemFontsLoaded(Vec<Family>),
    Loaded(String, Result<(), fount::Error>),
}

pub fn init() -> Task<Message> {
    Task::batch([
        Task::future(fount::google::catalog(
            fount::google::DEFAULT_CATALOG_MAX_AGE,
        ))
        .map(Message::CatalogLoaded),
        iced::font::list()
            .map(Result::ok)
            .and_then(Task::done)
            .map(Message::SystemFontsLoaded),
    ])
}

pub fn load(name: String, catalog: Option<&fount::Catalog>) -> Task<Message> {
    let catalog = catalog.cloned();
    let n = name.clone();
    Task::future(async move { fount::google::load(&name, catalog.as_ref()).await }).then(
        move |result: Result<Vec<Vec<u8>>, fount::Error>| {
            let n = n.clone();
            match result {
                Ok(bytes_list) => Task::batch(bytes_list.into_iter().map({
                    let n = n.clone();
                    move |bytes| {
                        let n = n.clone();
                        iced::font::load(bytes)
                            .map(move |r| Message::Loaded(n.clone(), r.map_err(into_fount_error)))
                    }
                })),
                Err(e) => Task::done(Message::Loaded(n, Err(e))),
            }
        },
    )
}

fn into_fount_error(e: iced::font::Error) -> fount::Error {
    fount::Error::Io(format!("{e:?}"))
}
