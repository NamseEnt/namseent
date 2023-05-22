const fs = require("fs");
const path = require("path");

let input = fs.readFileSync(path.join(__dirname, "src/input.json"), "utf-8");

const imageNames = fs.readdirSync(path.join(__dirname, "src/images"));
console.log("imageNames", imageNames);

imageNames.forEach((imageName, index) => {
    const extension = path.extname(imageName);
    const name = path.basename(imageName, extension);
    input = input.replaceAll(name, `${index}`);

    fs.cpSync(
        path.join(__dirname, `src/images/${imageName}`),
        path.join(__dirname, `src/renamedImages/${index}${extension}`),
    );
});

fs.writeFileSync(path.join(__dirname, "src/renamedInput.json"), input);
