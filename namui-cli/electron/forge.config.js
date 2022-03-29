module.exports = {
    packagerConfig: {
        ignore: [".wsl-env"],
    },
    hooks: {
        postPackage: async (forgeConfig, options) => {
            //  package result
            // {
            //     outputPath: String,
            //     platform: String,
            //     arch: String,
            // }
            console.log(
                JSON.stringify({
                    outputPath: options.outputPaths[0],
                    platform: options.platform,
                    arch: options.arch,
                }),
            );
        },
    },
};
