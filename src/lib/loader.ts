export enum Loader {
	Forge = 0,
	Fabric = 1,
	Quilt = 2,
	Neoforge = 3,
}

export function loaderToString(loader: Loader) {
	switch (loader) {
		case Loader.Forge:
			return 'forge';
		case Loader.Fabric:
			return 'fabric';
		case Loader.Quilt:
			return 'quilt';
		case Loader.Neoforge:
			return 'neoforge';
	}
}
