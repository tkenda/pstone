var fs = require('fs');
var path = require('path');
const toml = require('toml');
const { exec } = require('child_process');

module.exports = function (callback) {
    var version = '';
    var folder = '';

    try {
        const doc = toml.parse(fs.readFileSync(process.env.TOML_PATH, 'utf8'));
        version = doc.package.version;
        folder = process.env.APP_NAME.toLowerCase() + '_' + version + '-' + process.env.REVISION + '_amd64';
    } catch (err) {
        throw err;
    }

    // remove previous
    let rootDir = path.join(process.env.TARGET_FOLDER, folder);

    try {
        fs.rmdirSync(rootDir, { recursive: true });
    } catch (err) { }

    try {
        fs.unlinkSync(rootDir + '.deb');
    } catch (err) { }

    // create root folder
    fs.mkdirSync(rootDir);

    // create app folder
    fs.mkdirSync(path.join(rootDir, 'usr'));
    fs.mkdirSync(path.join(rootDir, 'usr', 'local'));
    let appDir = path.join(rootDir, 'usr', 'local', process.env.APP_NAME.toLowerCase());
    fs.mkdirSync(appDir);

    // copy executable
    fs.copyFileSync(path.join(process.env.TARGET_FOLDER, process.env.EXE_NAME), path.join(appDir, process.env.EXE_NAME));

    // control file
    var controlContent = '';
    controlContent += 'Package: ' + process.env.APP_NAME + '\n';
    controlContent += 'Version: ' + version + '\n';
    controlContent += 'Architecture: amd64\n';
    controlContent += 'Maintainer: Idria <kenda.tomas@idria.com.ar>\n';
    controlContent += 'Homepage: https://www.idria.com.ar\n';
    controlContent += 'Description: ' + process.env.DESCRIPTION + '\n';
    fs.mkdirSync(path.join(rootDir, 'DEBIAN'));
    fs.writeFileSync(path.join(rootDir, 'DEBIAN', 'control'), controlContent);

    // create postinst file
    fs.copyFileSync(path.join('deb', 'postinst'), path.join(rootDir, 'DEBIAN', 'postinst'));
    fs.chmodSync(path.join(rootDir, 'DEBIAN', 'postinst'), 0o755);

    // create systemd folder
    fs.mkdirSync(path.join(rootDir, 'etc'));
    fs.mkdirSync(path.join(rootDir, 'etc', 'systemd'));
    let systemdDir = path.join(rootDir, 'etc', 'systemd', 'system');
    fs.mkdirSync(systemdDir);

    // create service file
    const serviceName = process.env.APP_NAME.toLowerCase() + '.service';
    fs.copyFileSync(path.join('deb', serviceName), path.join(systemdDir, serviceName));

    // create .deb file
    exec('dpkg-deb --build --root-owner-group ' + rootDir, (err, stdout, stderr) => {
        if (err) {
            console.error(err);
            callback(true, '');
        } else {
            console.log(stdout);
            console.log(stderr);
            callback(false, folder + '.deb');
        }
    });
}