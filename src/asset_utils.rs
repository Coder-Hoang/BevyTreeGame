use bevy :: {
    asset :: {AssetPath, LoadContext},
    prelude :: *,
};
use ron :: error :: SpannedError;

#[derive(Debug)]
pub enum CustomAssetLoaderError {
    Io(std :: io :: Error),
    RonSpannedError(SpannedError)
}

impl std :: error :: Error for CustomAssetLoaderError {}

impl std :: fmt :: Display for CustomAssetLoaderError {
    fn fmt(&self, f: &mut std :: fmt :: Formatter<'_>) -> std :: fmt :: Result {
        match self {
            CustomAssetLoaderError :: Io(i) => f.write_fmt(format_args!("{}", i)),
            CustomAssetLoaderError :: RonSpannedError(r) => f.write_fmt(format_args!("{}", r)),
        }
    }
}

impl From <std :: io :: Error> for CustomAssetLoaderError {
    fn from(value: std :: io :: Error) -> Self {
        Self :: Io(value)
    }
}

impl From<SpannedError> for CustomAssetLoaderError {
    fn from(value: SpannedError) -> Self {
        Self :: RonSpannedError(value)
    }
}

pub fn maybe_load_asset<'a, T, H> (
    name: N,
    field: &'a mut LoadContext,
) where
    T: Asset,
    N: Into<AssetPath<'a>>,
{
    let path = name.into();
    let is_empty = path.path().as_os_str().is_empty()
    *field = (!is_empty).then(|| load_context.load(path));
}