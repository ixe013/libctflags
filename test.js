const ctflags = require('./index.js');
console.log('Testing ctflags bindings...');

try {
    const seed = ctflags.getSeedOrNull();
    console.log('Seed:', seed);

    const flag = ctflags.formatFlag('step1', 'somesalt');
    console.log('Flag:', flag);

    const flagContext = ctflags.formatFlagFromContext('ctx', 'step1', null);
    console.log('Flag Context:', flagContext);

    // Basic assertions
    if (typeof seed !== 'string') throw new Error('getSeedOrNull failed');
    if (typeof flag !== 'string' || !flag.startsWith('flag(step1).')) throw new Error('formatFlag failed: ' + flag);
    if (typeof flagContext !== 'string' || !flagContext.startsWith('flag(step1).')) throw new Error('formatFlagFromContext failed: ' + flagContext);

    console.log('SUCCESS');
} catch (error) {
    console.error('FAILURE:', error);
    process.exit(1);
}
