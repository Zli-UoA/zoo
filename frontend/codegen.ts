import type { CodegenConfig } from "@graphql-codegen/cli";

const config: CodegenConfig = {
	schema: "https://zoo-backend.zli.works/",
	documents: ["src/**/*.tsx"],
	ignoreNoDocuments: true, // for better experience with the watcher
	generates: {
		"./src/generated/": {
			preset: "client",
			config: {
				strictScalars: true,
				scalars: {},
				nonOptionalTypename: true,
			},
		},
	},
	hooks: {
		afterAllFileWrite: ["biome format --write"],
	},
};

export default config;
