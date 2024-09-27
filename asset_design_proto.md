# Asset and Editor System Design Prototype

## Goals

- A modular and hot reloadable asset system.
- Custom asset type definitions. (e.g. `Texture`, `Mesh`, `Material`, `MyCustomAssetType`)
- External, human readable asset files. Human editable.
- Asset files can be version controlled.
- Easy IDE and in-editor asset creation and editing.
- All assets registered with the asset server of a given asset type can be queried at runtime. (e.g. `asset_server.get_assets_of_type<Texture>(); => Vec<Handle<Texture>>` )
- Assets can be loaded and unloaded at runtime. (e.g. `asset_server.load_asset<Texture>("texture.png")`)

## Nice-To-Haves

- Assets type definitions could simply be a derive macro on a struct. (e.g. `#[derive(Asset)] struct Texture { ... }`)
