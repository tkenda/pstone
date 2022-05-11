const fs = require('fs');
const path = require('path');
const toml = require('toml');
const { exec } = require('child_process');

require('dotenv').config();

module.exports = function (callback) {
    var name = '';
    var version = '';

    if (typeof process.env.INNO_DIR === 'undefined') {
        throw 'Setup INNO_DIR in scipts/.env file with Inno Setup main folder.'
    }

    try {
        const doc = toml.parse(fs.readFileSync(process.env.TOML_PATH, 'utf8'));
        name = process.env.APP_NAME.toLowerCase() + '_' + doc.package.version;
        version = doc.package.version;
    } catch (err) {
        throw err;
    }

    // remove previous
    let rootDir = path.join(process.env.TARGET_FOLDER, name);

    try {
        fs.rmdirSync(rootDir, { recursive: true });
    } catch (err) { }

    try {
        fs.unlinkSync(rootDir + '.exe');
    } catch (err) { }

    // create root folder
    fs.mkdirSync(rootDir);

    // create inno script
    const SCRIPT_NAME = process.env.APP_NAME.toLowerCase() + '.iss';
    const SCRIPT_PATH = path.join(rootDir, SCRIPT_NAME);
    var content = fs.readFileSync(path.join(__dirname, 'win', SCRIPT_NAME), 'utf-8');
    
    console.log(process.env.APP_NAME);
    content = content.replaceAll("%%APP_NAME%%", process.env.APP_NAME);
    content = content.replaceAll("%%EXE_NAME%%", process.env.EXE_NAME);
    content = content.replaceAll("%%OUTPUT_BASE_FILENAME%%", name);
    content = content.replaceAll("%%APP_VERSION%%", version);

    console.log(content);

    fs.writeFileSync(SCRIPT_PATH, content);

    const APP_PATH = path.join(process.env.INNO_DIR, 'iscc.exe');

    exec('"' + APP_PATH + '" "' + SCRIPT_PATH + '"', (err, stdout, stderr) => {
        if (err) {
            console.error(err);
            callback(true, '');
        } else {
            console.log(stdout);
            console.log(stderr);
            console.log('ok');
            callback(false, name + '.exe');
        }
    });
}