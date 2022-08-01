const config = getConfig();

module.exports = {
    packagerConfig: {
        ignore: [".wsl-env"],
        protocols: [
            {
                name: "Deep link schemes for namui app",
                schemes: config.deepLinkSchemes,
            },
        ],
    },
    hooks: {
        postPackage: async (forgeConfig, options) => {
            //  package result
            // {
            //     outputPath: String,
            //     arch: String,
            // }
            console.log(
                JSON.stringify({
                    outputPath: options.outputPaths[0],
                    arch: options.arch,
                }),
            );
        },
    },
};

function getConfig() {
    const config = {
        deepLinkSchemes: [],
    };
    const argv = [...process.argv];
    while (argv.length) {
        const arg = argv.pop();
        if (arg.startsWith("deepLink=")) {
            const deepLinkScheme = arg.slice("deepLink=".length);
            const schemeNotExist =
                config.deepLinkSchemes.findIndex(deepLinkScheme) < 0;
            if (schemeNotExist) {
                config.deepLinkSchemes.push(deepLinkScheme);
            }
        }
    }
    return config;
}
