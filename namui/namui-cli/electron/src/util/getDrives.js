async function getDrives() {
    return new Promise((resolve, reject) => {
        exec("wmic logicaldisk get name", (error, stdout) => {
            if (error) {
                reject(error);
                return;
            }

            resolve(
                stdout
                    .split("\r\n")
                    .filter((line) => /([a-zA-Z]:)/.exec(line))
                    .map((line) => line.trim()),
            );
        });
    });
}

exports.getDrives = getDrives;
