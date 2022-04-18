module.exports = {
    packagerConfig: {
        ignore: [".wsl-env"],
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
