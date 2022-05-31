let fs = require('fs');
let logger = require('node-color-log')

// get content if feature_target.txt found
let target = fs.existsSync('feature_target.txt')
    ? fs.readFileSync('feature_target.txt', 'utf8')
    : 'scripts/features/**/*.feature';

logger.info('target: ' + target);

let common = [
    target,
    '--require-module ts-node/register',    // Load TypeScript module
    '--require-module tsconfig-paths/register',    // Load TypeScript module
    '--require scripts/features/stepDefinitions/**/*.ts',   // Load step definitions
    '--format progress',                // Load custom formatter
    '-f @cucumber/pretty-formatter',  // Load custom formatter
    '--publish-quiet',
    '--fail-fast',
];

let dev = Array.from(common);
dev.push('--tags @dev');

let report = Array.from(common);
report.push('--publish');
report.push('--format json:cucumber-report.json');


module.exports = {
    default: common.join(' '),
    dev: dev.join(' '),
    report: report.join(' ')
};
