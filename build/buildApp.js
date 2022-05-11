var buildWin = require('./buildWin.js');
var buildDeb = require('./buildDeb.js');

if (process.platform === 'win32' || process.platform === 'win64') {
    buildWin(function (error, file) {
        if (error) {
            console.log(error);
            return 1;
        }

        console.log('Artifact: ' + file + '.');
    });
} else if (process.platform === 'linux') {
    buildDeb(function (error, file) {
        if (error) {
            console.log(error);
            return 1;
        }

        console.log('Artifact: ' + file + ".");
    });
} else {
    console.log('OS Not Supported!');
}