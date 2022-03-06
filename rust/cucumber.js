let common = [
    'scripts/features/**/*.feature',                // Specify our feature files
    '--require-module ts-node/register',    // Load TypeScript module
    '--require-module tsconfig-paths/register',    // Load TypeScript module
    '--require scripts/features/step_definitions/**/*.ts',   // Load step definitions
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


