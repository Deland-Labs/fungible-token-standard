import { Argument, Command, OptionValues } from 'commander';

const program = new Command();


program
    .command('reinstall-all')
    .description('reinstall all canisters')
    .action(async () => {
        require('./reinstallAll')
    });


program.parse(process.argv);