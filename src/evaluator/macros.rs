#[macro_export]
macro_rules! get_evald(
    ($parsed:expr) => ({
        match $parsed {
            Ok(t) => t,
            Err(e) => {return Err(e);}
        }
    });
);
